// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

/**
 * ScreenDSL v1 - View Definition Domain-Specific Language
 *
 * ANTLR4 Grammar for L2 Screen/View Contract DSL
 *
 * DESIGN PRINCIPLES:
 * - Schema-driven: fields get their types from .schema, not inline
 * - Convention over configuration: all fields editable by default,
 *   override with readonly/hidden/required/emphasis
 * - Schema-as-shorthand: reference a schema and all fields appear;
 *   use groups only when custom layout is needed
 * - No technology jargon: domain terms only (no 3270, no HTML, no CSS)
 * - Forward-looking: works for web, mobile, voice, API, any renderer
 * - Import support: shared defaults and lookups via import
 *
 * LAYER: Layer 2 (Contract) -- defines WHAT the user sees and does
 *
 * SEMANTIC VALIDATION NOTES (enforced by compiler, not grammar):
 * - Fields referenced must exist in an imported schema
 * - Link targets must reference existing views
 * - Lookup names must reference existing lookup definitions
 * - When conditions must reference fields in scope
 *
 * RENDERING NOTES (handled by renderer, not DSL):
 * - Keyboard shortcuts (Escape=cancel, Ctrl+S=submit, Tab=next)
 * - Sorting/filtering on list columns
 * - Responsive layout (desktop vs mobile)
 * - Accessibility (ARIA, screen reader, tab order)
 * - Permission-based field visibility (auth layer filters at runtime)
 */

grammar ScreenDSL;

// ============================================================================
// PARSER RULES
// ============================================================================

// ----------------------------------------------------------------------------
// Top-Level Structure
// ----------------------------------------------------------------------------

program
    : importStatement* (viewDefinition | defaultsBlock | lookupDefinition)+ EOF
    ;

// ----------------------------------------------------------------------------
// Import Statement (shared with all Nexflow grammars)
// ----------------------------------------------------------------------------

importStatement
    : IMPORT importPath
    ;

importPath
    : importPathSegment+ importFileExtension
    ;

importPathSegment
    : DOTDOT | DOT | SLASH | IDENTIFIER | UPPER_IDENTIFIER | MINUS
    ;

importFileExtension
    : DOT ('screen' | 'schema' | 'rules' | 'xform' | 'proc')
    ;

// ----------------------------------------------------------------------------
// View Definition
// ----------------------------------------------------------------------------

viewDefinition
    : 'view' viewName title?
        viewBody
      'end'
    ;

viewName
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

title
    : STRING
    ;

viewBody
    : viewElement*
    ;

viewElement
    : schemaRef
    | focusDecl
    | groupBlock
    | fieldDecl
    | listBlock
    | rowBlock
    | labelDecl
    | linkDecl
    | actionDecl
    ;

// ----------------------------------------------------------------------------
// Schema Reference
// ----------------------------------------------------------------------------

schemaRef
    : 'schema' schemaName modifier*
    ;

schemaName
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

// ----------------------------------------------------------------------------
// Focus Declaration
// ----------------------------------------------------------------------------

focusDecl
    : 'focus' fieldRef
    ;

// ----------------------------------------------------------------------------
// Group Block (logical grouping of fields with optional condition)
// ----------------------------------------------------------------------------

groupBlock
    : 'group' title
        whenClause?
        groupElement*
      'end'
    ;

groupElement
    : fieldDecl
    | listBlock
    | rowBlock
    | labelDecl
    | linkDecl
    ;

// ----------------------------------------------------------------------------
// Row Block (side-by-side layout hint)
// ----------------------------------------------------------------------------

rowBlock
    : 'row'
        fieldDecl+
      'end'
    ;

// ----------------------------------------------------------------------------
// Field Declaration
// ----------------------------------------------------------------------------
// Fields are editable by default. Use modifiers for exceptions:
//   readonly  -- display only, user cannot change
//   hidden    -- not visible (conditionally shown via 'when')
//   required  -- must have a value before submit
//   emphasis  -- visually highlighted (e.g., financial totals)
//
// Labels are optional -- defaults to the field name from schema.
// Override the label for this specific view with a trailing string.

fieldDecl
    : 'field' fieldRef modifier* title? fieldClause*
    ;

fieldRef
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

modifier
    : 'readonly'
    | 'hidden'
    | 'required'
    | 'emphasis'
    ;

fieldClause
    : whenClause
    | optionsClause
    | fieldLink
    ;

// Field as a clickable link (in lists: click value to navigate)
fieldLink
    : ARROW 'view' viewRef
    ;

// ----------------------------------------------------------------------------
// List Block (editable grid / data table)
// ----------------------------------------------------------------------------
// Lists are editable grids by default (like Excel).
// Add 'readonly' to make the entire list display-only.
// Individual columns can be overridden with field modifiers.
//
// If schema is specified with no fields, ALL schema fields appear.
// If fields are listed, only those fields appear.

listBlock
    : 'list' listName modifier* 'visible' '=' INTEGER
        listBody
      'end'
    ;

listName
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

listBody
    : listElement*
    ;

listElement
    : schemaRef
    | fieldDecl
    | selectDecl
    ;

// Row selection navigation (click/select a row to navigate)
selectDecl
    : 'select' ARROW 'view' viewRef
    ;

// ----------------------------------------------------------------------------
// Label (static text)
// ----------------------------------------------------------------------------

labelDecl
    : 'label' STRING
    ;

// ----------------------------------------------------------------------------
// Link Declaration (navigation -- goes somewhere, passes data)
// ----------------------------------------------------------------------------
// Static links: link "Help" -> view HelpPage end
// Data links:   link "Statement" -> view Statement(AccountNumber) end
// Compound keys: link "Detail" -> view LineDetail(OrderId, LineId) end

linkDecl
    : 'link' title? ARROW 'view' viewRef whenClause?
    ;

// ----------------------------------------------------------------------------
// Action Declaration (buttons -- does something, triggers operation)
// ----------------------------------------------------------------------------
// Actions DO things: save, delete, validate, submit.
// Links GO places: navigate to another view.
//
// Severity modifiers for destructive/caution actions:
//   danger  -- destructive, irreversible (renderer: red, requires confirm)
//   warning -- significant change, use caution (renderer: amber)
//
// Target prefixes:
//   validate then <op>  -- run validation before operation
//   confirm then <op>   -- show confirmation dialog before operation

actionDecl
    : 'action' actionIntent title? ARROW actionTarget severity? whenClause?
    ;

actionIntent
    : 'submit'
    | 'navigate'
    | 'paginate'
    | 'cancel'
    | 'help'
    ;

actionTarget
    : 'validate'? 'then'? 'confirm'? 'then'? operationName   // submit -> validate then confirm then delete
    | 'view' viewRef                                           // navigate -> view AccountDetail(Id)
    | 'back'
    | 'next'
    | 'previous'
    | 'help'
    ;

operationName
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

severity
    : 'danger'
    | 'warning'
    ;

// ----------------------------------------------------------------------------
// View Reference (with optional parameters for data passing)
// ----------------------------------------------------------------------------

viewRef
    : viewName ('(' fieldRef (',' fieldRef)* ')')?
    ;

// ----------------------------------------------------------------------------
// When Clause (conditional visibility / availability)
// ----------------------------------------------------------------------------
// Used on: fields, groups, actions, links
// Simple condition: field_name operator value

whenClause
    : 'when' fieldRef compareOp conditionValue
    ;

compareOp
    : '='
    | '!='
    | '>'
    | '<'
    | '>='
    | '<='
    ;

conditionValue
    : STRING
    | INTEGER
    | 'true'
    | 'false'
    ;

// ----------------------------------------------------------------------------
// Options Clause (dropdown / select values from a lookup)
// ----------------------------------------------------------------------------

optionsClause
    : 'options' 'from' lookupRef
    ;

lookupRef
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

// ----------------------------------------------------------------------------
// Defaults Block (convention library -- field behavior patterns)
// ----------------------------------------------------------------------------
// Defines default modifiers for fields matching name patterns.
// Pattern matching: * matches any characters.
//   field CreatedBy     hidden        -- exact match
//   field *Id           readonly      -- ends with "Id"
//   field Created*      hidden        -- starts with "Created"
//
// Precedence (highest to lowest):
//   1. Explicit field declaration in a view
//   2. Imported defaults (later imports override earlier)
//   3. Built-in default: all fields editable

defaultsBlock
    : 'defaults'
        defaultEntry*
      'end'
    ;

defaultEntry
    : 'field' fieldPattern modifier+
    ;

fieldPattern
    : IDENTIFIER                         // Exact match: CreatedBy
    | UPPER_IDENTIFIER                   // Exact match: CreatedDate
    | WILDCARD_PATTERN                   // Glob: *Id, Created*, *Balance*
    ;

// ----------------------------------------------------------------------------
// Lookup Definition (reusable value lists for dropdowns)
// ----------------------------------------------------------------------------
// Define once, reference from any view via 'options from LookupName'.

lookupDefinition
    : 'lookup' lookupName
        lookupEntry+
      'end'
    ;

lookupName
    : IDENTIFIER
    | UPPER_IDENTIFIER
    ;

lookupEntry
    : STRING STRING                      // code "value", display "label"
    ;


// ============================================================================
// LEXER RULES
// ============================================================================

// Keywords
IMPORT      : 'import' ;
ARROW       : '->' ;

// Punctuation
LPAREN      : '(' ;
RPAREN      : ')' ;
LBRACE      : '{' ;
RBRACE      : '}' ;
LBRACKET    : '[' ;
RBRACKET    : ']' ;
COMMA       : ',' ;
DOT         : '.' ;
DOTDOT      : '..' ;
SLASH       : '/' ;
MINUS       : '-' ;
EQUALS      : '=' ;
NOT_EQUALS  : '!=' ;
GT          : '>' ;
LT          : '<' ;
GTE         : '>=' ;
LTE         : '<=' ;

// Literals
INTEGER
    : [0-9]+
    ;

STRING
    : '"' (~["\r\n])* '"'
    ;

// Wildcard patterns for defaults block
WILDCARD_PATTERN
    : '*' [a-zA-Z0-9_]+                 // *Id, *Balance, *_date
    | [a-zA-Z0-9_]+ '*'                 // Created*, Modified*
    | '*' [a-zA-Z0-9_]+ '*'             // *balance*, *status*
    ;

// Identifiers (case-preserving)
IDENTIFIER
    : [a-z_] [a-zA-Z0-9_]*
    ;

UPPER_IDENTIFIER
    : [A-Z] [a-zA-Z0-9_]*
    ;

// Comments and whitespace
COMMENT
    : '//' ~[\r\n]* -> skip
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
