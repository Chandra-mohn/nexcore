// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

/**
 * BmsDSL v1 - CICS BMS (Basic Mapping Support) Map Definition Grammar
 *
 * ANTLR4 Grammar for parsing IBM CICS BMS macro definitions.
 *
 * BMS files define 3270 terminal screen layouts using three assembler macros:
 * - DFHMSD: Mapset definition (container for one or more maps)
 * - DFHMDI: Map definition (one screen/panel within a mapset)
 * - DFHMDF: Field definition (one field on a screen)
 *
 * Additional directives:
 * - TITLE: Assembler title directive (optional, informational)
 * - END: Assembler end directive (terminates the source file)
 * - DFHMSD TYPE=FINAL: Terminates the mapset definition
 *
 * BMS SYNTAX FEATURES:
 * - Continuation lines: line ends with '-' near/at column 72
 * - Options: KEY=VALUE pairs, comma-separated
 * - Parenthesized lists: ATTRB=(PROT,BRT,NUM), POS=(3,15), SIZE=(24,80)
 * - Quoted strings: INITIAL='text', PICIN='999', PICOUT='ZZ9.99'
 * - Named fields: label DFHMDF ... (data fields accessible by program)
 * - Unnamed fields: DFHMDF ... (labels, spacers, attribute bytes)
 * - Comment lines: '*' in column 1
 *
 * PREPROCESSING:
 * The lexer handles continuation lines by treating '-' at/near column 72
 * followed by content on the next line as a single logical line.
 * Comment lines (starting with '*') are skipped.
 *
 * SEMANTIC VALIDATION (enforced by compiler, not grammar):
 * - POS values must be within SIZE bounds
 * - Field names must be unique within a map (for named fields)
 * - DFHMSD TYPE=FINAL must appear to close the mapset
 */

grammar BmsDSL;

// ============================================================================
// PARSER RULES
// ============================================================================

// ----------------------------------------------------------------------------
// Top-Level Structure
// ----------------------------------------------------------------------------

program
    : directive* mapsetDefinition directive* EOF
    ;

directive
    : titleDirective
    | endDirective
    ;

titleDirective
    : TITLE quotedString
    ;

endDirective
    : END
    ;

// ----------------------------------------------------------------------------
// Mapset Definition (DFHMSD)
// ----------------------------------------------------------------------------

mapsetDefinition
    : mapsetName DFHMSD mapsetOptions
      mapDefinition*
      mapsetFinal
    ;

mapsetName
    : IDENTIFIER
    ;

mapsetOptions
    : option (COMMA option)*
    ;

mapsetFinal
    : DFHMSD TYPE EQUALS FINAL
    ;

// ----------------------------------------------------------------------------
// Map Definition (DFHMDI)
// ----------------------------------------------------------------------------

mapDefinition
    : mapName DFHMDI mapOptions
      fieldDefinition*
    ;

mapName
    : IDENTIFIER
    ;

mapOptions
    : option (COMMA option)*
    ;

// ----------------------------------------------------------------------------
// Field Definition (DFHMDF)
// ----------------------------------------------------------------------------

fieldDefinition
    : fieldName? DFHMDF fieldOptions
    ;

fieldName
    : IDENTIFIER
    ;

fieldOptions
    : option (COMMA option)*
    ;

// ----------------------------------------------------------------------------
// Options (KEY=VALUE pairs)
// ----------------------------------------------------------------------------

option
    : optionKey EQUALS optionValue
    ;

optionKey
    : IDENTIFIER
    ;

optionValue
    : simpleValue
    | parenthesizedList
    | quotedString
    | ampersandValue
    ;

simpleValue
    : IDENTIFIER
    | INTEGER
    ;

parenthesizedList
    : LPAREN listItem (COMMA listItem)* RPAREN
    ;

listItem
    : IDENTIFIER
    | INTEGER
    ;

quotedString
    : STRING_LITERAL
    ;

ampersandValue
    : AMPERSAND IDENTIFIER
    | AMPERSAND AMPERSAND IDENTIFIER
    ;


// ============================================================================
// LEXER RULES
// ============================================================================

// BMS macro keywords
DFHMSD      : 'DFHMSD' ;
DFHMDI      : 'DFHMDI' ;
DFHMDF      : 'DFHMDF' ;
TYPE        : 'TYPE' ;
FINAL       : 'FINAL' ;
TITLE       : 'TITLE' ;
END         : 'END' ;

// Punctuation
LPAREN      : '(' ;
RPAREN      : ')' ;
COMMA       : ',' ;
EQUALS      : '=' ;
AMPERSAND   : '&' ;

// Literals
INTEGER
    : [0-9]+
    ;

STRING_LITERAL
    : '\'' ( ~['\r\n] | '\'\'' )* '\''
    ;

// Identifiers (BMS uses uppercase, but accept any case)
IDENTIFIER
    : [a-zA-Z#@$] [a-zA-Z0-9#@$_-]*
    ;

// Comment lines: '*' in column 1 (or any line starting with '*')
COMMENT
    : '*' ~[\r\n]* -> skip
    ;

// Continuation: '-' followed by optional spaces and newline, then
// optional spaces on the next line before content continues.
// This joins logical lines so the parser sees one continuous statement.
CONTINUATION
    : '-' [ \t]* ('\r'? '\n') [ \t]* -> skip
    ;

// Whitespace (spaces and tabs within a line)
WS
    : [ \t]+ -> skip
    ;

// Newlines (line boundaries -- significant for separating macros)
NEWLINE
    : '\r'? '\n' -> skip
    ;
