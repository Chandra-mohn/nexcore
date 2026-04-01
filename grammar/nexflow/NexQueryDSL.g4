// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

/**
 * NexQueryDSL - Code Intelligence Query Language
 *
 * ANTLR4 Grammar for NexQuery
 *
 * NexQuery is a domain-specific query language for exploring COBOL codebases
 * during modernization. It provides composable, English-readable queries
 * over a code intelligence graph.
 *
 * Spec: docs/nexquery_spec.md
 *
 * Three clause types:
 * - Traverse: <node-type> <traversal-verb> <target> <filter?>
 * - Expand:   <node-type> <filter?>
 * - Verb:     <domain-verb> <target?> <modifier*>
 *
 * Clauses compose via newlines into pipeline statements.
 * Statements terminate with semicolons.
 */

grammar NexQueryDSL;

// ============================================================================
// PARSER RULES
// ============================================================================

// A script is one or more statements
program
    : statement+ EOF
    ;

// A statement is one or more clauses terminated by semicolon
statement
    : clause+ SEMICOLON?
    ;

// A clause is traverse, expand, or verb
// Ambiguity: NODE_TYPE followed by TRAVERSAL_VERB -> traverse, otherwise -> expand
clause
    : traverseClause
    | expandClause
    | verbClause
    ;

// <node-type> <traversal-verb> <target> <filter?>
traverseClause
    : nodeType traversalVerb target filter?
    ;

// <node-type> <filter?>
expandClause
    : nodeType filter?
    ;

// <domain-verb> <target?> <modifier*>
verbClause
    : domainVerb target? modifier*
    ;

// ----------------------------------------------------------------------------
// Node Types (7 entity types in the code graph)
// ----------------------------------------------------------------------------

nodeType
    : PROGRAMS
    | PARAGRAPHS
    | FIELDS
    | COPYBOOKS
    | FILES
    | TABLES
    | RULES
    ;

// ----------------------------------------------------------------------------
// Traversal Verbs (7 forward + 7 reverse = 14 verbs)
// ----------------------------------------------------------------------------

traversalVerb
    : CALLING
    | CALLED_BY
    | PERFORMING
    | PERFORMED_BY
    | READING
    | READ_BY
    | WRITING
    | WRITTEN_BY
    | USING
    | USED_BY
    | ACCESSING
    | ACCESSED_BY
    | CONTAINING
    | WITHIN
    ;

// ----------------------------------------------------------------------------
// Domain Verbs (11 high-level commands)
// ----------------------------------------------------------------------------

domainVerb
    : TRACE
    | RANK
    | SIMILAR
    | FIND_DEAD
    | DEPS
    | IMPACT
    | COMPARE
    | DISCOVER_PROCESSES
    | ESTIMATE_COST
    | SAVE
    | RUN
    ;

// ----------------------------------------------------------------------------
// Targets
// ----------------------------------------------------------------------------

target
    : EACH
    | list
    | nodeType                  // Allow node types as targets (e.g., "rank programs")
    | IDENTIFIER
    ;

list
    : LBRACKET listItem (COMMA listItem)* RBRACKET
    ;

listItem
    : IDENTIFIER
    | STRING
    ;

// ----------------------------------------------------------------------------
// Filters (parenthesized predicate expressions)
// ----------------------------------------------------------------------------

filter
    : LPAREN filterExpr RPAREN
    ;

filterExpr
    : filterExpr AND filterExpr
    | filterExpr OR filterExpr
    | NOT filterExpr
    | LPAREN filterExpr RPAREN
    | predicate
    ;

predicate
    : fieldRef compareOp value
    ;

fieldRef
    : IDENTIFIER (DOT IDENTIFIER)?
    ;

compareOp
    : EQ
    | NE
    | GT
    | LT
    | GE
    | LE
    | GLOB
    | REGEX
    | IN
    | HAS
    ;

value
    : STRING
    | NUMBER
    | list
    ;

// ----------------------------------------------------------------------------
// Modifiers (keyword arguments for domain verbs)
// ----------------------------------------------------------------------------

modifier
    : modifierKeyword modifierValue
    ;

modifierKeyword
    : BY
    | TOP
    | IN
    | WITH
    | DEPTH
    | LEVEL
    | ORDER
    | SCOPE
    | THRESHOLD
    | AS
    ;

modifierValue
    : IDENTIFIER
    | NUMBER
    | nodeType
    | STRING
    ;

// ============================================================================
// LEXER RULES
// ============================================================================

// ----------------------------------------------------------------------------
// Node Type Keywords
// ----------------------------------------------------------------------------

PROGRAMS    : 'programs' ;
PARAGRAPHS  : 'paragraphs' ;
FIELDS      : 'fields' ;
COPYBOOKS   : 'copybooks' ;
FILES       : 'files' ;
TABLES      : 'tables' ;
RULES       : 'rules' ;

// ----------------------------------------------------------------------------
// Traversal Verb Keywords
// ----------------------------------------------------------------------------

CALLING      : 'calling' ;
CALLED_BY    : 'called-by' ;
PERFORMING   : 'performing' ;
PERFORMED_BY : 'performed-by' ;
READING      : 'reading' ;
READ_BY      : 'read-by' ;
WRITING      : 'writing' ;
WRITTEN_BY   : 'written-by' ;
USING        : 'using' ;
USED_BY      : 'used-by' ;
ACCESSING    : 'accessing' ;
ACCESSED_BY  : 'accessed-by' ;
CONTAINING   : 'containing' ;
WITHIN       : 'within' ;

// ----------------------------------------------------------------------------
// Domain Verb Keywords
// ----------------------------------------------------------------------------

TRACE              : 'trace' ;
RANK               : 'rank' ;
SIMILAR            : 'similar' ;
FIND_DEAD          : 'find-dead' ;
DEPS               : 'deps' ;
IMPACT             : 'impact' ;
COMPARE            : 'compare' ;
DISCOVER_PROCESSES : 'discover-processes' ;
ESTIMATE_COST      : 'estimate-cost' ;
SAVE               : 'save' ;
RUN                : 'run' ;

// ----------------------------------------------------------------------------
// Modifier Keywords
// ----------------------------------------------------------------------------

BY        : 'by' ;
TOP       : 'top' ;
IN        : 'in' ;
WITH      : 'with' ;
DEPTH     : 'depth' ;
LEVEL     : 'level' ;
ORDER     : 'order' ;
SCOPE     : 'scope' ;
THRESHOLD : 'threshold' ;
AS        : 'as' ;
EACH      : 'each' ;

// ----------------------------------------------------------------------------
// Filter Keywords
// ----------------------------------------------------------------------------

AND : 'and' ;
OR  : 'or' ;
NOT : 'not' ;
HAS : 'has' ;

// ----------------------------------------------------------------------------
// Operators
// ----------------------------------------------------------------------------

EQ    : '=' ;
NE    : '!=' ;
GT    : '>' ;
LT    : '<' ;
GE    : '>=' ;
LE    : '<=' ;
GLOB  : '~' ;
REGEX : '~~' ;

// ----------------------------------------------------------------------------
// Punctuation
// ----------------------------------------------------------------------------

SEMICOLON : ';' ;
COMMA     : ',' ;
DOT       : '.' ;
LPAREN    : '(' ;
RPAREN    : ')' ;
LBRACKET  : '[' ;
RBRACKET  : ']' ;

// ----------------------------------------------------------------------------
// Literals
// ----------------------------------------------------------------------------

NUMBER
    : [0-9]+ ('.' [0-9]+)?
    ;

STRING
    : '\'' (~['\r\n])* '\''
    ;

// Identifiers: COBOL names can have hyphens
IDENTIFIER
    : [A-Za-z_] [A-Za-z0-9_-]*
    ;

// ----------------------------------------------------------------------------
// Continuation dots (visual aid, ignored)
// ----------------------------------------------------------------------------

CONTINUATION
    : '..' -> skip
    ;

// ----------------------------------------------------------------------------
// Comments and Whitespace
// ----------------------------------------------------------------------------

LINE_COMMENT
    : '--' ~[\r\n]* -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
