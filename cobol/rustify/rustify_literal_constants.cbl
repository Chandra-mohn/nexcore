       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-LITERAL-CONSTANTS-TEST.
      *---------------------------------------------------------------
      * Rustification test: Literal parse -> const/dec!()
      * Tier 1 - Cosmetic. Tests that numeric and string literals
      * used in VALUE clauses and MOVE statements can be promoted
      * to Rust const declarations and dec!() macros.
      * Edge cases: sign, decimal alignment, hex/zero literals.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-INT-POS              PIC 9(5) VALUE 12345.
       01  WS-INT-ZERO             PIC 9(5) VALUE 0.
       01  WS-INT-ONE              PIC 9(5) VALUE 1.
       01  WS-SIGNED-POS           PIC S9(5) VALUE +500.
       01  WS-SIGNED-NEG           PIC S9(5) VALUE -999.
       01  WS-DEC-SIMPLE           PIC 9(5)V99 VALUE 123.45.
       01  WS-DEC-ZERO             PIC 9(5)V99 VALUE 0.00.
       01  WS-DEC-TRAIL            PIC 9(5)V99 VALUE 100.10.
       01  WS-SDEC                 PIC S9(5)V99 VALUE -456.78.
       01  WS-LARGE-INT            PIC 9(9) VALUE 999999999.
       01  WS-STR-LIT              PIC X(10) VALUE "HELLO".
       01  WS-STR-SPACES           PIC X(10) VALUE SPACES.
       01  WS-STR-ZEROS            PIC X(5) VALUE ZEROS.
       01  WS-RESULT               PIC 9(9)V99 VALUE 0.
       01  WS-TEMP                 PIC S9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Display initial VALUE literals
           DISPLAY WS-INT-POS.
           DISPLAY WS-INT-ZERO.
           DISPLAY WS-INT-ONE.
           DISPLAY WS-SIGNED-POS.
           DISPLAY WS-SIGNED-NEG.
           DISPLAY WS-DEC-SIMPLE.
           DISPLAY WS-DEC-ZERO.
           DISPLAY WS-DEC-TRAIL.
           DISPLAY WS-SDEC.
           DISPLAY WS-LARGE-INT.
           DISPLAY WS-STR-LIT.
           DISPLAY WS-STR-SPACES.
           DISPLAY WS-STR-ZEROS.
      * Literal in MOVE
           MOVE 42 TO WS-TEMP.
           DISPLAY WS-TEMP.
           MOVE -100 TO WS-TEMP.
           DISPLAY WS-TEMP.
      * Literal in arithmetic
           ADD 10.50 TO WS-DEC-SIMPLE.
           DISPLAY WS-DEC-SIMPLE.
           MULTIPLY 2 BY WS-INT-POS.
           DISPLAY WS-INT-POS.
      * Edge: max value literal
           MOVE 999999999 TO WS-LARGE-INT.
           DISPLAY WS-LARGE-INT.
           STOP RUN.
