// Nexflow ServiceDSL Grammar -- Microservice Orchestration
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// Defines .service files: request/response microservice implementations.
// Covers handlers, config, transactions, sagas, caching, health.
//
// Design principles:
//   P2: Small orthogonal primitives (8 verbs) + composition operators (4)
//   P4: Grammar = syntax, Compiler = semantics
//   P5: Keywords are structural; infrastructure values are identifiers/strings
//   P6: Expressions are predicates and assignments, not computations
//
// Primitives:  authorize, validate, lookup, transform, call, persist, publish, respond
// Composition: sequence (implicit), parallel, transaction, saga
// Error:       on_error, on_rollback, compensate, fallback
//
// Config block replaces middleware -- generic key-value pairs for infrastructure.
// cfg.X references resolve from config block or external nexflow.config.yaml.
//
// Keyword count: 37 structural

grammar ServiceDSL;

// ===================================================================
// Parser Rules
// ===================================================================

compilationUnit
    : importStatement* serviceDefinition EOF
    ;

// --- Imports -------------------------------------------------------

importStatement
    : IMPORT importPath
    ;

importPath
    : (DOT_SLASH | DOT_DOT_SLASH)? pathSegment (SLASH pathSegment)* DOT word
    ;

pathSegment
    : word (DASH word)*
    ;

// Allows keywords to appear in import paths (e.g., ./rules/account-validation.rules)
word
    : IDENTIFIER | AUTHORIZE | CACHE | CALL | CHECK | COMPENSATE | CONFIG
    | CONSUMES | CONTAINS | DESCRIPTION | EACH | END | FALLBACK | FOR
    | FROM | HANDLER | HEALTH | IMPLEMENTS | IMPORT | IN | INTO
    | INVALIDATE_ON | IS | LOOKUP | NULL | ON_ERROR | ON_ROLLBACK
    | PARALLEL | PERSIST | PUBLISH | READY | RESPOND | SAGA | SCOPE
    | SERVICE | STEP | TIMEOUT | TO | TRANSACTION | TRANSFORM | TTL
    | USING | VALIDATE | WHEN | WHERE | WITH
    ;

// --- Service Definition --------------------------------------------

serviceDefinition
    : SERVICE IDENTIFIER serviceBody END
    ;

serviceBody
    : serviceElement*
    ;

serviceElement
    : descriptionDecl
    | implementsDecl
    | consumesDecl
    | configBlock
    | handlerDecl
    | cacheBlock
    | healthBlock
    | readyDecl
    ;

// --- Header Declarations -------------------------------------------

descriptionDecl
    : DESCRIPTION STRING_LITERAL
    ;

implementsDecl
    : IMPLEMENTS identifierList
    ;

consumesDecl
    : CONSUMES identifierList
    ;

// --- Config Block (infrastructure defaults) ------------------------
// Generic key-value pairs. The compiler validates known keys.
// Values here are defaults; overridable via nexflow.config.yaml.
// Example:
//   config
//       tracing_provider "opentelemetry"
//       metrics_namespace "account"
//       db_pool_size 10
//   end

configBlock
    : CONFIG configDirective* END
    ;

configDirective
    : IDENTIFIER configValue (COMMA configValue)*
    ;

configValue
    : INTEGER
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | IDENTIFIER
    | TRUE
    | FALSE
    ;

// --- Handler -------------------------------------------------------

handlerDecl
    : HANDLER IDENTIFIER handlerBody END
    ;

handlerBody
    : handlerStatement*
    ;

handlerStatement
    : authorizeStmt
    | validateStmt
    | lookupStmt
    | transformStmt
    | persistStmt
    | callStmt
    | publishStmt
    | respondStmt
    | onErrorBlock
    | transactionBlock
    | sagaBlock
    | parallelBlock
    ;

// --- Authorize -----------------------------------------------------

authorizeStmt
    : AUTHORIZE SCOPE STRING_LITERAL
    ;

// --- Validate ------------------------------------------------------

validateStmt
    : VALIDATE expression USING qualifiedRef
    ;

// --- Lookup --------------------------------------------------------

lookupStmt
    : LOOKUP IDENTIFIER FROM IDENTIFIER whereClause* END
    ;

whereClause
    : WHERE expression comparator expression
    ;

// --- Transform -----------------------------------------------------
// Source is optional: `transform using X.xform into Y` (implicit request)
// or explicit: `transform a, b using X.xform into Y`

transformStmt
    : TRANSFORM expressionList USING qualifiedRef INTO IDENTIFIER
    | TRANSFORM USING qualifiedRef INTO IDENTIFIER
    ;

// --- Persist -------------------------------------------------------

persistStmt
    : PERSIST expression TO IDENTIFIER
    | PERSIST expression EQ expression TO IDENTIFIER
    ;

// --- Call (service-to-service) -------------------------------------
// Supports three forms:
//   call AuditAPI.logChange with action "account.created"
//   call AccountAPI.getBalance into balance
//   call CatalogAPI.getProduct with product_id = item.product_id
//       for each item in request.items into product_details end

callStmt
    : CALL qualifiedRef withClause* forEachClause? intoClause? END?
    ;

withClause
    : WITH IDENTIFIER EQ expression
    | WITH IDENTIFIER STRING_LITERAL
    ;

forEachClause
    : FOR EACH IDENTIFIER IN expression
    ;

intoClause
    : INTO IDENTIFIER
    ;

// --- Publish -------------------------------------------------------

publishStmt
    : PUBLISH IDENTIFIER
    ;

// --- Respond -------------------------------------------------------

respondStmt
    : RESPOND INTEGER schemaRef?
    ;

// --- on_error Block ------------------------------------------------
// Predicate-based error mapping. Expressions are guards, not computations.
// Complex logic belongs in .rules -- P6.

onErrorBlock
    : ON_ERROR errorCase+ END
    ;

errorCase
    : INTEGER WHEN predicate
    | FALLBACK expression TO IDENTIFIER
    ;

predicate
    : expression IS NULL
    | expression comparator expression
    | expression CONTAINS (IDENTIFIER | NULL)
    ;

// --- Transaction Block ---------------------------------------------

transactionBlock
    : TRANSACTION handlerStatement* onRollbackBlock? END
    ;

onRollbackBlock
    : ON_ROLLBACK handlerStatement*
    ;

// --- Saga Block (distributed transactions) -------------------------

sagaBlock
    : SAGA sagaStep+ END
    ;

sagaStep
    : STEP IDENTIFIER handlerStatement* compensateBlock END
    ;

compensateBlock
    : COMPENSATE handlerStatement* END
    ;

// --- Parallel Block ------------------------------------------------

parallelBlock
    : PARALLEL handlerStatement+ END
    ;

// --- Cache Block ---------------------------------------------------

cacheBlock
    : CACHE cacheEntry+ END
    ;

cacheEntry
    : IDENTIFIER TTL valueOrCfg IDENTIFIER (INVALIDATE_ON identifierList)?
    ;

// --- Health Block --------------------------------------------------

healthBlock
    : HEALTH STRING_LITERAL healthCheck* END
    ;

healthCheck
    : CHECK word word TIMEOUT valueOrCfg word
    ;
    // e.g., check database AccountDB timeout 5 seconds
    //        check service AuditAPI timeout cfg.audit_timeout seconds
    // First word = check kind (database, service -- keywords allowed)
    // Second word = resource name
    // valueOrCfg = timeout value (literal or cfg. ref)
    // Last word = time unit

readyDecl
    : READY STRING_LITERAL
    ;

// --- Qualified Annotations (namespace.value) -----------------------
// Unified pattern: cfg.X, pii.X, audit.X, sec.X, etc.

qualifiedAnnotation
    : IDENTIFIER DOT IDENTIFIER
    ;

// --- Value or Config Reference -------------------------------------
// Allows literal or cfg. reference in the same position.

valueOrCfg
    : INTEGER
    | DECIMAL_LITERAL
    | qualifiedAnnotation              // cfg.pool_size, cfg.timeout, etc.
    ;

// --- Expressions (predicates and paths) ----------------------------
// P6: Expressions are dotted paths and simple predicates.
// Complex logic belongs in .rules and .xform.

expression
    : literal
    | qualifiedAnnotation              // cfg.X or any namespace.value
    | IDENTIFIER (DOT word)*           // dotted path: request.from, request.to, etc.
    ;

expressionList
    : expression (COMMA expression)*
    ;

comparator
    : EQ | NEQ | LT | GT | LTE | GTE
    ;

// --- Common References ---------------------------------------------

qualifiedRef
    : IDENTIFIER DOT IDENTIFIER
    ;

schemaRef
    : IDENTIFIER (DOT IDENTIFIER)?
    ;

identifierList
    : IDENTIFIER (COMMA IDENTIFIER)*
    ;

literal
    : INTEGER
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | TRUE
    | FALSE
    | NULL
    ;

// ===================================================================
// Lexer Rules
// ===================================================================

// --- Structural Keywords -------------------------------------------
// Intentionally minimal. Infrastructure config values (provider names,
// time units, check kinds) are parsed as IDENTIFIER.

AUTHORIZE    : 'authorize' ;
CACHE        : 'cache' ;
CALL         : 'call' ;
CHECK        : 'check' ;
COMPENSATE   : 'compensate' ;
CONFIG       : 'config' ;
CONSUMES     : 'consumes' ;
CONTAINS     : 'contains' ;
DESCRIPTION  : 'description' ;
EACH         : 'each' ;
END          : 'end' ;
FALLBACK     : 'fallback' ;
FOR          : 'for' ;
FROM         : 'from' ;
HANDLER      : 'handler' ;
HEALTH       : 'health' ;
IMPLEMENTS   : 'implements' ;
IMPORT       : 'import' ;
IN           : 'in' ;
INTO         : 'into' ;
INVALIDATE_ON : 'invalidate_on' ;
IS           : 'is' ;
LOOKUP       : 'lookup' ;
NULL         : 'null' ;
ON_ERROR     : 'on_error' ;
ON_ROLLBACK  : 'on_rollback' ;
PARALLEL     : 'parallel' ;
PERSIST      : 'persist' ;
PUBLISH      : 'publish' ;
READY        : 'ready' ;
RESPOND      : 'respond' ;
SAGA         : 'saga' ;
SCOPE        : 'scope' ;
SERVICE      : 'service' ;
STEP         : 'step' ;
TIMEOUT      : 'timeout' ;
TO           : 'to' ;
TRANSACTION  : 'transaction' ;
TRANSFORM    : 'transform' ;
TTL          : 'ttl' ;
USING        : 'using' ;
VALIDATE     : 'validate' ;
WHEN         : 'when' ;
WHERE        : 'where' ;
WITH         : 'with' ;

// --- Boolean and Null Literals -------------------------------------

TRUE         : 'true' ;
FALSE        : 'false' ;

// --- Operators -----------------------------------------------------

EQ           : '=' ;
NEQ          : '!=' ;
LT           : '<' ;
GT           : '>' ;
LTE          : '<=' ;
GTE          : '>=' ;

// --- Numeric Literals ----------------------------------------------

INTEGER
    : [0-9]+
    ;

DECIMAL_LITERAL
    : [0-9]+ '.' [0-9]+
    ;

// --- String Literal ------------------------------------------------

STRING_LITERAL
    : '"' (~["\\\r\n] | '\\' .)* '"'
    ;

// --- Identifiers ---------------------------------------------------
// Catches all non-keyword names: type names, schema refs, store names,
// time units (seconds, minutes), check kinds (database, service),
// config keys, and any future vocabulary additions.

IDENTIFIER
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

// --- Symbols -------------------------------------------------------

DOT           : '.' ;
COMMA         : ',' ;
SLASH         : '/' ;
DASH          : '-' ;
DOT_SLASH     : './' ;
DOT_DOT_SLASH : '../' ;

// ===================================================================
// Whitespace and Comments
// ===================================================================

LINE_COMMENT
    : '//' ~[\r\n]* -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
