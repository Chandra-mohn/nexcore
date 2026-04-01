// Nexflow ApiDSL Grammar -- Service Contract Specification
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// Defines .api files: declarative, protocol-agnostic service contracts.
// Covers endpoints, events, auth, versioning, deprecation, dependencies.
//
// Design principles:
//   P1: Dual-author (machine emitter + human editor) -- simple and readable
//   P4: Grammar = syntax, Compiler = semantics (validation is compiler's job)
//   P5: Keywords are structural only; config values are identifiers/strings
//   P7: v1 minimalism -- easier to add constructs than remove them
//
// Qualified annotation pattern: namespace.value (e.g., cfg.rate_limit, pii.ssn)
//   - cfg.X = config indirection, resolved from config block or external file
//   - Compiler validates known namespaces and their vocabularies
//
// Keyword count: 33 structural + 5 HTTP methods = 38 total

grammar ApiDSL;

// ===================================================================
// Parser Rules
// ===================================================================

compilationUnit
    : importStatement* apiDefinition EOF
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

// Allows keywords to appear in import paths (e.g., ./schemas/errors.schema)
word
    : IDENTIFIER | API | ASYNC | AUTH | BASE | BURST | CACHE | CONFIG
    | CONTENT_TYPE | CORS | DEFAULT | DEFAULT_SIZE | DEPENDENCIES
    | DEPRECATED | DESCRIPTION | END | ENDPOINT | ERRORS | EVENT
    | HEADER | HEADERS | HEALTH | IDEMPOTENT | IMPORT | KEY | LIST
    | MAX_AGE | MAX_SIZE | METHOD | METHODS | OPTIONAL | ORIGINS
    | PAGINATED | PAGINATION | PARAMS | PARTITIONED_BY | PATH | PAYLOAD
    | PER | QUERY | RATE_LIMIT | READY | REPLACEMENT | REQUEST
    | REQUIRED | RESPONSE | SCOPE | SUNSET | TARGETS | TIMEOUT
    | TOPIC | USING | VALIDATE | VERSION
    ;

// --- API Definition ------------------------------------------------

apiDefinition
    : API IDENTIFIER apiBody END
    ;

apiBody
    : apiElement*
    ;

apiElement
    : versionDecl
    | baseDecl
    | descriptionDecl
    | targetsDecl
    | authDefaultDecl
    | contentTypeDecl
    | rateLimitDecl
    | paginationDecl
    | configBlock
    | corsBlock
    | endpointDecl
    | deprecatedEndpointDecl
    | eventDecl
    | dependenciesBlock
    | healthDecl
    | readyDecl
    ;

// --- Header Declarations -------------------------------------------

versionDecl
    : VERSION STRING_LITERAL
    ;

baseDecl
    : BASE STRING_LITERAL
    ;

descriptionDecl
    : DESCRIPTION STRING_LITERAL
    ;

targetsDecl
    : TARGETS identifierList
    ;

authDefaultDecl
    : AUTH DEFAULT authScheme
    ;

contentTypeDecl
    : CONTENT_TYPE IDENTIFIER
    ;

rateLimitDecl
    : RATE_LIMIT valueOrCfg PER IDENTIFIER (BURST valueOrCfg)?
    ;

paginationDecl
    : PAGINATION IDENTIFIER DEFAULT_SIZE INTEGER MAX_SIZE INTEGER
    ;

// --- Config Block (defaults for cfg. references) -------------------

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

// --- CORS Block ----------------------------------------------------

corsBlock
    : CORS corsDirective* END
    ;

corsDirective
    : ORIGINS valueOrCfgList
    | METHODS httpMethodList
    | HEADERS stringList
    | MAX_AGE valueOrCfg
    ;

// --- Endpoint Declaration ------------------------------------------

endpointDecl
    : ENDPOINT IDENTIFIER endpointBody END
    ;

deprecatedEndpointDecl
    : DEPRECATED ENDPOINT IDENTIFIER endpointBody sunsetDecl? replacementDecl? END
    ;

endpointBody
    : endpointClause*
    ;

endpointClause
    : methodDecl
    | pathDecl
    | descriptionDecl
    | paramsBlock
    | queryBlock
    | headersBlock
    | requestDecl
    | responseDecl
    | errorsBlock
    | authDecl
    | validateDecl
    | rateLimitDecl
    | timeoutDecl
    | cacheDecl
    | idempotentDecl
    | asyncDecl
    ;

methodDecl
    : METHOD httpMethod
    ;

httpMethod
    : GET | POST | PUT | PATCH | DELETE
    ;

pathDecl
    : PATH STRING_LITERAL
    ;

paramsBlock
    : PARAMS paramDecl+ END
    ;

queryBlock
    : QUERY paramDecl+ END
    ;

headersBlock
    : HEADERS paramDecl+ END
    ;

paramDecl
    : IDENTIFIER typeRef paramModifier*
    ;

paramModifier
    : REQUIRED
    | OPTIONAL
    | DEFAULT literal
    ;

requestDecl
    : REQUEST schemaRef
    ;

responseDecl
    : RESPONSE responseModifier? schemaRef
    ;

responseModifier
    : PAGINATED
    | LIST
    ;

errorsBlock
    : ERRORS errorMapping+ END
    ;

errorMapping
    : INTEGER USING schemaRef
    ;

authDecl
    : AUTH authScheme
    ;

// Auth scheme: first identifier is scheme name (bearer, api_key, oauth, mtls)
// Compiler validates known schemes; grammar accepts any identifier.
authScheme
    : IDENTIFIER authSchemeArg*
    ;

authSchemeArg
    : SCOPE STRING_LITERAL
    | HEADER STRING_LITERAL
    ;

validateDecl
    : VALIDATE USING qualifiedRef
    ;

timeoutDecl
    : TIMEOUT valueOrCfg IDENTIFIER    // e.g., timeout 5 seconds / timeout cfg.x seconds
    ;

cacheDecl
    : CACHE valueOrCfg IDENTIFIER      // e.g., cache 60 seconds / cache cfg.ttl seconds
    ;

idempotentDecl
    : IDEMPOTENT KEY STRING_LITERAL
    ;

asyncDecl
    : ASYNC
    ;

sunsetDecl
    : SUNSET STRING_LITERAL
    ;

replacementDecl
    : REPLACEMENT IDENTIFIER
    ;

// --- Event Declaration ---------------------------------------------

eventDecl
    : EVENT IDENTIFIER eventBody END
    ;

eventBody
    : eventClause*
    ;

eventClause
    : topicDecl
    | payloadDecl
    | partitionedByDecl
    ;

topicDecl
    : TOPIC STRING_LITERAL
    ;

payloadDecl
    : PAYLOAD schemaRef
    ;

partitionedByDecl
    : PARTITIONED_BY IDENTIFIER
    ;

// --- Dependencies --------------------------------------------------

dependenciesBlock
    : DEPENDENCIES dependencyDecl+ END
    ;

dependencyDecl
    : IDENTIFIER VERSION STRING_LITERAL
    ;

// --- Health / Ready ------------------------------------------------

healthDecl
    : HEALTH STRING_LITERAL
    ;

readyDecl
    : READY STRING_LITERAL
    ;

// --- Qualified Annotations (namespace.value) -----------------------
// Unified pattern for cfg.X, pii.X, audit.X, sec.X, etc.
// The namespace is an identifier; the compiler validates known namespaces.

qualifiedAnnotation
    : IDENTIFIER DOT IDENTIFIER
    ;

// --- Value or Config Reference -------------------------------------
// Allows either a literal value or a cfg. reference in the same position.
// Examples: rate_limit 1000 per minute    (literal)
//           rate_limit cfg.rate per minute (config indirection)

valueOrCfg
    : INTEGER
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | qualifiedAnnotation              // cfg.rate_limit, cfg.timeout, etc.
    ;

valueOrCfgList
    : valueOrCfg (COMMA valueOrCfg)*
    ;

// --- Common References ---------------------------------------------

schemaRef
    : IDENTIFIER (DOT IDENTIFIER)?
    ;

qualifiedRef
    : IDENTIFIER DOT IDENTIFIER
    ;

typeRef
    : IDENTIFIER                       // string, integer, decimal, boolean, datetime,
    ;                                  // or any schema type -- compiler validates

identifierList
    : IDENTIFIER (COMMA IDENTIFIER)*
    ;

stringList
    : STRING_LITERAL (COMMA STRING_LITERAL)*
    ;

httpMethodList
    : httpMethod (COMMA httpMethod)*
    ;

literal
    : INTEGER
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | TRUE
    | FALSE
    ;

// ===================================================================
// Lexer Rules
// ===================================================================

// --- Structural Keywords -------------------------------------------
// These define grammar shape. Intentionally minimal.
// Config values (bearer, rest, seconds, etc.) are parsed as IDENTIFIER.

API          : 'api' ;
ASYNC        : 'async' ;
AUTH         : 'auth' ;
BASE         : 'base' ;
BURST        : 'burst' ;
CACHE        : 'cache' ;
CONFIG       : 'config' ;
CONTENT_TYPE : 'content_type' ;
CORS         : 'cors' ;
DEFAULT      : 'default' ;
DEFAULT_SIZE : 'default_size' ;
DEPENDENCIES : 'dependencies' ;
DEPRECATED   : 'deprecated' ;
DESCRIPTION  : 'description' ;
END          : 'end' ;
ENDPOINT     : 'endpoint' ;
ERRORS       : 'errors' ;
EVENT        : 'event' ;
HEADER       : 'header' ;
HEADERS      : 'headers' ;
HEALTH       : 'health' ;
IDEMPOTENT   : 'idempotent' ;
IMPORT       : 'import' ;
KEY          : 'key' ;
LIST         : 'list' ;
MAX_AGE      : 'max_age' ;
MAX_SIZE     : 'max_size' ;
METHOD       : 'method' ;
METHODS      : 'methods' ;
OPTIONAL     : 'optional' ;
ORIGINS      : 'origins' ;
PAGINATED    : 'paginated' ;
PAGINATION   : 'pagination' ;
PARAMS       : 'params' ;
PARTITIONED_BY : 'partitioned_by' ;
PATH         : 'path' ;
PAYLOAD      : 'payload' ;
PER          : 'per' ;
QUERY        : 'query' ;
RATE_LIMIT   : 'rate_limit' ;
READY        : 'ready' ;
REPLACEMENT  : 'replacement' ;
REQUEST      : 'request' ;
REQUIRED     : 'required' ;
RESPONSE     : 'response' ;
SCOPE        : 'scope' ;
SUNSET       : 'sunset' ;
TARGETS      : 'targets' ;
TIMEOUT      : 'timeout' ;
TOPIC        : 'topic' ;
USING        : 'using' ;
VALIDATE     : 'validate' ;
VERSION      : 'version' ;

// --- HTTP Method Keywords ------------------------------------------
// Kept as keywords (universally known, finite set, improves readability).

GET          : 'GET' ;
POST         : 'POST' ;
PUT          : 'PUT' ;
PATCH        : 'PATCH' ;
DELETE       : 'DELETE' ;

// --- Boolean Literals ----------------------------------------------

TRUE         : 'true' ;
FALSE        : 'false' ;

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
// Catches all non-keyword names: type names, schema refs, auth schemes
// (bearer, api_key, oauth, mtls), targets (rest, graphql, grpc),
// pagination styles (cursor, offset, keyset), time units (seconds,
// minutes, hours), and any future vocabulary additions.

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

// --- Whitespace and Comments ---------------------------------------

LINE_COMMENT
    : '//' ~[\r\n]* -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
