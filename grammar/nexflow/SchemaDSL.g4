// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

/**
 * SchemaDSL v2 - Schema Registry Domain-Specific Language
 *
 * ANTLR4 Grammar for L2 Schema Registry DSL
 *
 * v2 SYNTAX CHANGES:
 * - Removed ':' separator between field name and type
 * - Type parameters use parentheses: decimal(10,2) instead of decimal[precision: 10, scale: 2]
 * - Constraints (enum, range, pattern) moved to constraints block
 * - Inline field syntax: field_name type [required] [default(...)] [pii.category]
 *
 * This grammar defines the syntax for schema definitions including:
 * - Data mutation patterns (14 patterns)
 * - Type system (base types with parenthesized parameters)
 * - Streaming annotations (key fields, time semantics, watermarks)
 * - Schema evolution (versioning, compatibility)
 * - Constraints block (enum, range, pattern, business rules)
 *
 * SEMANTIC VALIDATION NOTES (enforced by compiler, not grammar):
 * - identity block required for most patterns
 * - streaming block required for event_log pattern
 * - state transitions must form valid graph for state_machine
 * - compatibility mode validated against previous version
 */

grammar SchemaDSL;

// ============================================================================
// PARSER RULES
// ============================================================================

// ----------------------------------------------------------------------------
// Top-Level Structure
// ----------------------------------------------------------------------------

program
    : importStatement* (schemaDefinition | typeAliasBlock)+ EOF
    ;

// ----------------------------------------------------------------------------
// Import Statement
// ----------------------------------------------------------------------------

importStatement
    : IMPORT importPath
    ;

importPath
    : importPathSegment+ importFileExtension  // Path ends with file extension
    ;

importPathSegment
    : DOTDOT | DOT | SLASH | IDENTIFIER | UPPER_IDENTIFIER | MINUS  // Allow: ./path, ../path, /abs/path, PascalCase, path-with-hyphens
    ;

importFileExtension
    : DOT ('schema' | 'transform' | 'flow' | 'rules')  // File extension marks end of import
    ;

schemaDefinition
    : 'schema' schemaName
        patternDecl?
        targetsDecl?                    // Target technologies for code generation
        versionBlock?
        compatibilityDecl?              // Allow standalone evolution/compatibility declaration
        retentionDecl?
        identityBlock?
        streamingBlock?
        serializationBlock?             // Serialization format configuration
        fieldsBlock?
        nestedObjectBlock*
        computedBlock?                  // Computed/derived fields
        constraintsBlock?               // Validation constraints (enum, range, pattern, rules)
        immutableDecl?                  // immutable true/false for audit schemas (can appear after fields)
        stateMachineBlock?
        parametersBlock?
        entriesBlock?
        ruleBlock*
        migrationBlock?
      'end'
    ;

schemaName
    : IDENTIFIER
    | mutationPattern                   // Allow pattern keywords as schema names
    | timeSemanticsType                 // Allow event_time, processing_time as names
    ;

// ----------------------------------------------------------------------------
// Pattern Declaration (The 14 Mutation Patterns)
// ----------------------------------------------------------------------------

patternDecl
    : 'pattern' mutationPattern (',' mutationPattern)*
    ;

mutationPattern
    : 'master_data'                   // SCD Type 2 with full history
    | 'immutable_ledger'              // Append-only financial records
    | 'versioned_configuration'       // Immutable versions with effective dates
    | 'operational_parameters'        // Hot-reloadable parameters
    | 'event_log'                     // Append-only event stream
    | 'state_machine'                 // Workflow state tracking
    | 'temporal_data'                 // Effective-dated values
    | 'reference_data'                // Lookup tables
    | 'business_logic'                // Compiled rules
    | 'command'                       // Command/request pattern
    | 'response'                      // Response pattern
    | 'aggregate'                     // Aggregate/summary pattern
    | 'document'                      // Document/output pattern
    | 'audit_event'                   // Audit trail events
    ;

// ----------------------------------------------------------------------------
// Targets Declaration (Technology-specific code generation)
// ----------------------------------------------------------------------------
// Specifies which technologies to generate code for.
// If omitted, defaults are determined by the pattern type.
//
// Example:
//   schema OrderEvent
//       pattern event_log
//       targets kafka, flink         // Generate Kafka and Flink artifacts only
//       ...
//   end

targetsDecl
    : 'targets' targetList
    ;

targetList
    : target (',' target)*
    ;

target
    : 'kafka'                         // Kafka topics, producers, consumers, serializers
    | 'flink'                         // Flink process functions, state descriptors
    | 'mongo'                         // MongoDB collections, repositories, indexes
    | 'java'                          // Java helper classes (pattern-specific)
    | 'all'                           // Generate for all applicable technologies
    ;

// ----------------------------------------------------------------------------
// Version Block (Schema Evolution)
// ----------------------------------------------------------------------------

versionBlock
    : 'version' VERSION_NUMBER
      compatibilityDecl?
      previousVersionDecl?
      deprecationDecl?
      migrationGuideDecl?
    ;

compatibilityDecl
    : ('compatibility' | 'evolution') compatibilityMode
    ;

compatibilityMode
    : 'backward'                      // New reads old
    | 'forward'                       // Old reads new
    | 'full'                          // Both directions
    | 'none'                          // Breaking changes allowed
    | 'backward_compatible'           // Alias for backward
    | 'forward_compatible'            // Alias for forward
    ;

previousVersionDecl
    : 'previous_version' VERSION_NUMBER
    ;

deprecationDecl
    : 'deprecated' ':' STRING
      ('deprecated_since' ':' STRING)?
      ('removal_version' ':' VERSION_NUMBER)?
    ;

migrationGuideDecl
    : 'migration_guide' ':' (STRING | MULTILINE_STRING)
    ;

// ----------------------------------------------------------------------------
// Retention Declaration
// ----------------------------------------------------------------------------

retentionDecl
    : 'retention' duration
    ;

// ----------------------------------------------------------------------------
// Immutable Declaration (for audit schemas)
// ----------------------------------------------------------------------------

immutableDecl
    : 'immutable' BOOLEAN
    ;

// ----------------------------------------------------------------------------
// Constraints Block (validation rules - v2 syntax)
// ----------------------------------------------------------------------------
// v2: All validation constraints moved here from inline field declarations
//
// Syntax:
//   field_name enum(value1, value2, ...)           // Enum constraint for any type
//   field_name range(min, max)                     // Range constraint for numerics
//   field_name pattern("regex")                    // Pattern constraint for strings
//   field_name length(min, max)                    // Length constraint for strings/lists
//   expression as "error message"                  // Business rule constraint
//
// Example:
//   constraints
//       status enum(pending, confirmed, shipped)
//       priority enum(1, 2, 3)
//       amount range(0.01, 999999.99)
//       email pattern("^[a-z]+@[a-z]+\\.[a-z]+$")
//       amount > 0 as "Amount must be positive"
//   end

constraintsBlock
    : 'constraints' constraintDecl+ 'end'
    ;

constraintDecl
    : fieldPath enumConstraint                               // field enum(a, b, c)
    | fieldPath rangeConstraint                              // field range(1, 100)
    | fieldPath patternConstraint                            // field pattern("...")
    | fieldPath lengthConstraint                             // field length(0, 100)
    | condition 'as' STRING                                  // expr as "message"
    ;

enumConstraint
    : 'enum' '(' enumValue (',' enumValue)* ')'
    ;

enumValue
    : IDENTIFIER                      // Unquoted identifier
    | STRING                          // Quoted string
    | INTEGER                         // Integer value
    | BOOLEAN                         // Boolean value
    ;

rangeConstraint
    : 'range' '(' numberLiteral ',' numberLiteral ')'
    ;

patternConstraint
    : 'pattern' '(' STRING ')'
    ;

lengthConstraint
    : 'length' '(' INTEGER ',' INTEGER ')'
    | 'length' '(' INTEGER ')'                              // Exact length
    ;

// ----------------------------------------------------------------------------
// Identity Block (v2 syntax - no colon separator)
// ----------------------------------------------------------------------------

identityBlock
    : 'identity' identityFieldV2+ 'end'
    ;

identityFieldV2
    : fieldName fieldTypeV2 fieldQualifierV2* ','?
    ;

// ----------------------------------------------------------------------------
// Streaming Block
// ----------------------------------------------------------------------------

streamingBlock
    : 'streaming' streamingDecl+ 'end'
    ;

streamingDecl
    : keyFieldsDecl
    | timeFieldDecl
    | timeSemanticsDecl
    | watermarkDecl
    | lateDataDecl
    | allowedLatenessDecl
    | idleDecl
    | sparsityDecl
    | retentionBlockDecl
    ;

keyFieldsDecl
    : 'key_fields' fieldArray
    ;

timeFieldDecl
    : 'time_field' (fieldPath | timeSemanticsType)  // Allow time_field event_time
    ;

timeSemanticsDecl
    : 'time_semantics' timeSemanticsType
    ;

timeSemanticsType
    : 'event_time'
    | 'processing_time'
    | 'ingestion_time'
    ;

watermarkDecl
    : 'watermark_delay' duration
    | 'watermark_strategy' watermarkStrategy
    | 'max_out_of_orderness' duration
    | 'watermark_interval' duration
    | 'watermark_field' fieldPath
    ;

watermarkStrategy
    : 'bounded_out_of_orderness'
    | 'periodic'
    | 'punctuated'
    ;

lateDataDecl
    : 'late_data_handling' lateDataStrategy
    | 'late_data_stream' IDENTIFIER
    ;

lateDataStrategy
    : 'side_output'
    | 'drop'
    | 'update'
    ;

allowedLatenessDecl
    : 'allowed_lateness' duration
    ;

idleDecl
    : 'idle_timeout' duration
    | 'idle_behavior' idleBehavior
    ;

idleBehavior
    : 'mark_idle'
    | 'advance_to_infinity'
    | 'keep_waiting'
    ;

sparsityDecl
    : 'sparsity' sparsityBlock 'end'
    ;

sparsityBlock
    : ('dense' fieldArray)?
      ('moderate' fieldArray)?
      ('sparse' fieldArray)?
    ;

retentionBlockDecl
    : 'retention' retentionOptions 'end'
    ;

retentionOptions
    : ('time' duration)?
      ('size' sizeSpec)?
      ('policy' retentionPolicy)?
    ;

retentionPolicy
    : 'delete_oldest'
    | 'archive'
    | 'compact'
    ;

// ----------------------------------------------------------------------------
// Serialization Block (Kafka Serialization Format Configuration)
// ----------------------------------------------------------------------------
// Declares the preferred serialization format for this schema.
// Used for Kafka sources/sinks. Can be overridden at process level.
//
// Example:
//   serialization
//       format avro
//       compatibility backward
//       subject "orders-value"
//   end

serializationBlock
    : 'serialization' serializationDecl+ 'end'
    ;

serializationDecl
    : formatDecl
    | serializationCompatibilityDecl
    | subjectDecl
    | registryDecl
    ;

formatDecl
    : 'format' serializationFormat
    ;

serializationFormat
    : 'json'
    | 'avro'
    | 'confluent_avro'
    | 'protobuf'
    ;

serializationCompatibilityDecl
    : 'compatibility' compatibilityMode
    ;

subjectDecl
    : 'subject' STRING
    ;

registryDecl
    : 'registry' STRING
    ;

// ----------------------------------------------------------------------------
// Fields Block (v2 syntax - no colon separator)
// ----------------------------------------------------------------------------
// v2 Syntax: field_name type[(params)] [required] [default(...)] [pii.category]
//
// Examples:
//   order_id uuid required
//   amount decimal(10,2) required
//   status string required
//   currency char(3) required default("USD")
//   email string pii.email
//   tags list(string)
//   metadata map(string, integer)
//   address object { street string, city string required }

fieldsBlock
    : 'fields' fieldDeclV2+ 'end'
    ;

fieldDeclV2
    : fieldName fieldTypeV2 fieldQualifierV2* ','?
    ;

fieldName
    : IDENTIFIER
    | timeSemanticsType                 // Allow event_time, processing_time as field names
    | stateQualifier                    // Allow initial, terminal as field names
    ;

// ----------------------------------------------------------------------------
// Nested Object Block (v2 syntax)
// ----------------------------------------------------------------------------

nestedObjectBlock
    : fieldName 'object' fieldDeclV2* nestedObjectBlock* 'end'
    | fieldName 'list' LANGLE 'object' RANGLE fieldDeclV2* nestedObjectBlock* 'end'
    ;

// ----------------------------------------------------------------------------
// Computed Block (derived fields calculated from other fields)
// ----------------------------------------------------------------------------

computedBlock
    : 'computed' computedField+ 'end'
    ;

computedField
    : fieldName '=' computedExpression
    ;

// Computed expressions support arithmetic, when/then/else, and field references
computedExpression
    : computedExpression (STAR | SLASH) computedExpression      // Multiplication/Division (highest precedence)
    | computedExpression (PLUS | MINUS) computedExpression      // Addition/Subtraction
    | computedExpression comparisonOp computedExpression        // Comparisons
    | computedExpression 'and' computedExpression               // Logical AND
    | computedExpression 'or' computedExpression                // Logical OR
    | 'not' computedExpression                                  // Logical NOT
    | '(' computedExpression ')'                                // Parenthesized expression
    | computedWhenExpression                                    // Conditional expression
    | functionCall                                              // Function call
    | fieldPath                                                 // Field reference
    | literal                                                   // Literal value
    ;

computedWhenExpression
    : 'when' computedExpression 'then' computedExpression
      ('when' computedExpression 'then' computedExpression)*
      'else' computedExpression
    ;

// ----------------------------------------------------------------------------
// State Machine Block (for state_machine pattern)
// ----------------------------------------------------------------------------

stateMachineBlock
    : forEntityDecl?
      statesBlock
      initialStateDecl?
      transitionsBlock?
      onTransitionBlock?
    ;

// Explicit initial state declaration (alternative to inline :initial qualifier)
initialStateDecl
    : 'initial_state' IDENTIFIER
    ;

forEntityDecl
    : 'for_entity' IDENTIFIER
    ;

// New intuitive states block: states ... end with optional qualifiers
statesBlock
    : 'states' (statesDecl | stateDefList) 'end'?
    ;

// Original compact syntax: states [s1, s2, s3]
statesDecl
    : stateArray
    ;

// New intuitive syntax: each state on its own line
stateDefList
    : stateDef+
    ;

stateDef
    : IDENTIFIER (':' stateQualifier)?     // received: initial OR just received
    ;

stateQualifier
    : 'initial'
    | 'terminal'
    | 'final'                              // alias for terminal
    ;

stateArray
    : '[' IDENTIFIER (',' IDENTIFIER)* ']'
    ;

// Transitions block supports both syntaxes
transitionsBlock
    : 'transitions' (transitionDecl | transitionArrowDecl)+ 'end'
    ;

// Original syntax: from state [target1, target2]
transitionDecl
    : 'from' IDENTIFIER stateArray
    ;

// New arrow syntax: state -> state: trigger_event
transitionArrowDecl
    : (IDENTIFIER | '*') ARROW IDENTIFIER (':' IDENTIFIER)?  // from -> to: trigger
    ;

onTransitionBlock
    : 'on_transition' transitionAction+ 'end'
    ;

transitionAction
    : 'to' IDENTIFIER ':' actionCall
    ;

actionCall
    : IDENTIFIER '(' (STRING (',' STRING)*)? ')'
    ;

// ----------------------------------------------------------------------------
// Parameters Block (for operational_parameters pattern - v2 syntax)
// ----------------------------------------------------------------------------

parametersBlock
    : 'parameters' parameterDeclV2+ 'end'
    ;

parameterDeclV2
    : fieldName fieldTypeV2
        parameterOption*
    ;

parameterOption
    : 'default' '(' literal ')'
    | 'range' '(' numberLiteral ',' numberLiteral ')'
    | 'can_schedule' BOOLEAN
    | 'change_frequency' IDENTIFIER
    ;

// ----------------------------------------------------------------------------
// Entries Block (for reference_data pattern - v2 syntax)
// ----------------------------------------------------------------------------

entriesBlock
    : 'entries' entryDecl+ 'end'
    ;

entryDecl
    : IDENTIFIER ':'
        entryFieldV2+
    ;

entryFieldV2
    : fieldName literal
    | 'deprecated' BOOLEAN
    | 'deprecated_reason' STRING
    ;

// ----------------------------------------------------------------------------
// Rule Block (for business_logic pattern - v2 syntax)
// ----------------------------------------------------------------------------

ruleBlock
    : 'rule' IDENTIFIER
        givenBlock
        calculateBlock?
        returnBlock
      'end'
    ;

givenBlock
    : 'given' ruleFieldDeclV2+ 'end'
    ;

ruleFieldDeclV2
    : fieldName fieldTypeV2
    ;

calculateBlock
    : 'calculate' calculation+ 'end'
    ;

calculation
    : fieldName '=' expression
    ;

returnBlock
    : 'return' ruleFieldDeclV2+ 'end'
    ;

// ----------------------------------------------------------------------------
// Migration Block
// ----------------------------------------------------------------------------

migrationBlock
    : 'migration' migrationStatement+ 'end'
    ;

migrationStatement
    : fieldPath '=' expression
    | '(' fieldList ')' '=' functionCall
    ;

// ----------------------------------------------------------------------------
// Type Alias Block (v2 syntax)
// ----------------------------------------------------------------------------

typeAliasBlock
    : 'types' typeAliasV2+ 'end'
    ;

typeAliasV2
    : aliasName fieldTypeV2
    | aliasName 'object' fieldDeclV2* 'end'
    ;

aliasName
    : UPPER_IDENTIFIER
    ;

// ----------------------------------------------------------------------------
// Type System v2 (no colon separator, parenthesized parameters)
// ----------------------------------------------------------------------------
// v2 Types:
//   uuid, boolean, timestamp, date, json, text, bytes, bizdate  - no params
//   string, string(100), char(3)                                 - optional max length / fixed length
//   integer, decimal(10,2), float                                - optional precision/scale
//   list(type), set(type), map(key_type, value_type)            - parameterized collections
//   object { field type, ... }                                   - inline object
//   CustomType, TypeAlias                                        - custom/alias types

fieldTypeV2
    : baseTypeV2                                // string, integer, etc.
    | collectionTypeV2                          // list(T), set(T), map(K,V)
    | inlineObjectTypeV2                        // object { fieldDecl* }
    | IDENTIFIER                                // Custom/domain type
    | UPPER_IDENTIFIER                          // Type alias reference
    ;

// Base types with optional parenthesized parameters
baseTypeV2
    : 'string' typeParams?                      // string or string(100)
    | 'char' typeParams                         // char(3) - fixed length required
    | 'text'                                    // Unbounded text
    | 'integer' typeParams?                     // integer or integer(precision)
    | 'decimal' typeParams?                     // decimal or decimal(precision, scale)
    | 'float'                                   // Float
    | 'boolean'                                 // Boolean
    | 'date'                                    // Date
    | 'timestamp'                               // Timestamp
    | 'uuid'                                    // UUID
    | 'bytes'                                   // Binary
    | 'bizdate'                                 // Business date
    | 'json'                                    // JSON blob
    ;

// Parenthesized type parameters: (10) or (10, 2)
typeParams
    : '(' INTEGER (',' INTEGER)? ')'
    ;

// Collection types with parenthesized element types
collectionTypeV2
    : 'list' '(' fieldTypeV2 ')'
    | 'set' '(' fieldTypeV2 ')'
    | 'map' '(' fieldTypeV2 ',' fieldTypeV2 ')'
    ;

// Inline object type: object { field type, ... }
inlineObjectTypeV2
    : 'object' LBRACE inlineFieldDeclV2* RBRACE
    ;

inlineFieldDeclV2
    : fieldName fieldTypeV2 fieldQualifierV2*
    ;

// ----------------------------------------------------------------------------
// Field Qualifiers v2
// ----------------------------------------------------------------------------

fieldQualifierV2
    : 'required'
    | 'optional'
    | 'unique'
    | 'cannot_change'
    | 'encrypted'
    | piiModifier
    | defaultClauseV2
    | deprecatedClauseV2
    ;

// ----------------------------------------------------------------------------
// PII Modifier (Voltage Encryption)
// ----------------------------------------------------------------------------
// Syntax: pii.profile_name or just pii (defaults to full encryption)
// Examples:
//   ssn string required pii.ssn           // SSN format - last 4 clear
//   card_number string pii.pan            // Credit card - first 4 + last 4 clear
//   email string pii.email                // Email format preserving
//   secret string pii                     // Full encryption (default)
//   custom_field string pii.my_profile    // Custom profile from config

piiModifier
    : PII DOT IDENTIFIER                    // pii.ssn, pii.pan, pii.custom
    | PII                                   // pii (defaults to full encryption)
    ;

// v2: default(value) with parentheses
defaultClauseV2
    : 'default' '(' (literal | functionCall) ')'
    ;

deprecatedClauseV2
    : 'deprecated' '(' STRING ')'
    | 'removal' '(' VERSION_NUMBER ')'
    ;

// ----------------------------------------------------------------------------
// Expressions (for calculations and migrations)
// ----------------------------------------------------------------------------

expression
    : literal
    | fieldPath
    | functionCall
    | timeUnit                          // Allow time units as expression (e.g., years, days)
    | expression operator expression
    | '(' expression ')'
    | whenExpression
    ;

whenExpression
    : 'when' condition ':' expression
      ('when' condition ':' expression)*
      'otherwise' ':' expression
    ;

condition
    : expression comparisonOp expression
    | expression 'and' condition
    | expression 'or' condition
    | '(' condition ')'
    ;

comparisonOp
    : EQ | NE | LANGLE | RANGLE | LE | GE
    ;

operator
    : '+' | '-' | '*' | '/'
    ;

functionCall
    : IDENTIFIER '(' (expression (',' expression)*)? ')'
    ;

// ----------------------------------------------------------------------------
// Common Rules
// ----------------------------------------------------------------------------

fieldPath
    : IDENTIFIER ('.' IDENTIFIER)*
    ;

fieldList
    : fieldPath (',' fieldPath)*
    ;

fieldArray
    : '[' fieldPath (',' fieldPath)* ']'
    | '[' ']'                                   // Empty array (broadcast)
    ;

duration
    : INTEGER timeUnit
    | DURATION_LITERAL
    ;

timeUnit
    : 'seconds' | 'second'
    | 'minutes' | 'minute'
    | 'hours'   | 'hour'
    | 'days'    | 'day'
    | 'weeks'   | 'week'
    | 'months'  | 'month'
    | 'years'   | 'year'
    | 'milliseconds' | 'millisecond'
    ;

sizeSpec
    : INTEGER sizeUnit
    ;

sizeUnit
    : 'bytes' | 'KB' | 'MB' | 'GB' | 'TB'
    ;

literal
    : STRING
    | numberLiteral
    | BOOLEAN
    | 'null'
    ;

numberLiteral
    : INTEGER
    | DECIMAL
    | '-' INTEGER
    | '-' DECIMAL
    ;

// ----------------------------------------------------------------------------
// Keywords
// ----------------------------------------------------------------------------

// Import
IMPORT : 'import' ;

PII : 'pii' ;

// ----------------------------------------------------------------------------
// Literals
// ----------------------------------------------------------------------------

VERSION_NUMBER
    : [0-9]+ '.' [0-9]+ '.' [0-9]+
    ;

INTEGER
    : [0-9]+
    ;

DECIMAL
    : [0-9]+ '.' [0-9]+
    ;

DURATION_LITERAL
    : [0-9]+ ('ms' | 's' | 'm' | 'h' | 'd' | 'w')
    ;

BOOLEAN
    : 'true' | 'false'
    ;

IDENTIFIER
    : [a-z_] [a-z0-9_]*
    ;

UPPER_IDENTIFIER
    : [A-Z] [a-zA-Z0-9_]*
    ;

STRING
    : '"' (~["\r\n])* '"'
    ;

MULTILINE_STRING
    : '"""' .*? '"""'
    ;

// ----------------------------------------------------------------------------
// Operators
// ----------------------------------------------------------------------------

COLON : ':' ;
COMMA : ',' ;
DOTDOT : '..' ;                       // Must come before DOT for correct lexing
DOT : '.' ;
LBRACKET : '[' ;
RBRACKET : ']' ;
LPAREN : '(' ;
RPAREN : ')' ;
LANGLE : '<' ;
RANGLE : '>' ;
EQ : '=' | '==' ;                     // Support both = and == for equality
NE : '!=' ;
LE : '<=' ;
GE : '>=' ;
PLUS : '+' ;
MINUS : '-' ;
STAR : '*' ;
SLASH : '/' ;
ARROW : '->' ;
LBRACE : '{' ;
RBRACE : '}' ;

// ----------------------------------------------------------------------------
// Comments and Whitespace
// ----------------------------------------------------------------------------

COMMENT
    : '//' ~[\r\n]* -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
