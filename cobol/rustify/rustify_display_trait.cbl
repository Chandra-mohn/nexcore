       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-DISPLAY-TRAIT-TEST.
      *---------------------------------------------------------------
      * Rustification test: display_bytes() -> Display trait
      * Tier 1 - Cosmetic. Tests that DISPLAY statements using
      * raw byte output can be replaced with Rust Display trait.
      * Edge cases: leading zeros, sign display, space padding,
      * mixed alpha-numeric, concatenated DISPLAY.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-INT                  PIC 9(5) VALUE 42.
       01  WS-SINT                 PIC S9(5) VALUE -123.
       01  WS-DEC                  PIC 9(5)V99 VALUE 123.45.
       01  WS-SDEC                 PIC S9(5)V99 VALUE -67.89.
       01  WS-ALPHA                PIC X(15) VALUE "HELLO WORLD".
       01  WS-SHORT-STR            PIC X(5) VALUE "AB".
       01  WS-PACK                 PIC S9(7)V99 COMP-3 VALUE 1234.56.
       01  WS-COMP                 PIC S9(9) COMP VALUE 9876.
       01  WS-ZERO                 PIC 9(5) VALUE 0.
       01  WS-BLANK                PIC X(10) VALUE SPACES.
       01  WS-LABEL                PIC X(8) VALUE "RESULT: ".
       01  WS-GROUP.
           05  WS-GRP-ID           PIC 9(3) VALUE 1.
           05  WS-GRP-NAME         PIC X(10) VALUE "TEST".
       01  WS-EDITED               PIC Z,ZZ9.99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple numeric display
           DISPLAY WS-INT.
           DISPLAY WS-SINT.
           DISPLAY WS-DEC.
           DISPLAY WS-SDEC.
      * Alphanumeric display with padding
           DISPLAY WS-ALPHA.
           DISPLAY WS-SHORT-STR.
      * Packed and binary display
           DISPLAY WS-PACK.
           DISPLAY WS-COMP.
      * Edge: leading zeros and blank
           DISPLAY WS-ZERO.
           DISPLAY WS-BLANK.
      * Concatenated DISPLAY (multiple items)
           DISPLAY WS-LABEL WS-INT.
           DISPLAY WS-LABEL WS-ALPHA.
      * Group display (flat byte representation)
           DISPLAY WS-GROUP.
      * Edited field display
           MOVE 1234.56 TO WS-EDITED.
           DISPLAY WS-EDITED.
      * DISPLAY with literal
           DISPLAY "LITERAL OUTPUT".
           DISPLAY "VALUE=" WS-DEC.
           STOP RUN.
