       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-COMPUTE-TO-EXPR-TEST.
      *---------------------------------------------------------------
      * Rustification test: COMPUTE -> native arithmetic expressions
      * Tier 3b - Compound operations. Tests that COMPUTE statements
      * can use native Rust arithmetic operators.
      * Edge cases: operator precedence, overflow, ROUNDED,
      * mixed types, parenthesized expressions.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A                    PIC 9(5) VALUE 100.
       01  WS-B                    PIC 9(5) VALUE 25.
       01  WS-C                    PIC 9(5) VALUE 10.
       01  WS-RESULT               PIC 9(9)V99 VALUE 0.
       01  WS-INT-RESULT           PIC 9(9) VALUE 0.
       01  WS-SMALL                PIC 9(3) VALUE 0.
       01  WS-DEC-A                PIC S9(5)V99 VALUE 123.45.
       01  WS-DEC-B                PIC S9(5)V99 VALUE 67.89.
       01  WS-DEC-RESULT           PIC S9(7)V99 VALUE 0.
       01  WS-PACK-A               PIC S9(7)V99 COMP-3 VALUE 500.25.
       01  WS-PACK-B               PIC S9(7)V99 COMP-3 VALUE 3.00.
       01  WS-PACK-RESULT          PIC S9(7)V99 COMP-3 VALUE 0.
       01  WS-ROUND-RESULT         PIC S9(5)V99 VALUE 0.
       01  WS-ERR-FLAG             PIC 9 VALUE 0.
       01  WS-COMP-VAL             PIC S9(9) COMP VALUE 1000.
       01  WS-POWER-BASE           PIC 9(3) VALUE 2.
       01  WS-POWER-EXP            PIC 9(3) VALUE 10.
       01  WS-POWER-RESULT         PIC 9(9) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple addition
           COMPUTE WS-RESULT = WS-A + WS-B.
           DISPLAY WS-RESULT.
      * Subtraction
           COMPUTE WS-RESULT = WS-A - WS-B.
           DISPLAY WS-RESULT.
      * Multiplication
           COMPUTE WS-RESULT = WS-A * WS-B.
           DISPLAY WS-RESULT.
      * Division
           COMPUTE WS-RESULT = WS-A / WS-B.
           DISPLAY WS-RESULT.
      * Operator precedence: * before +
           COMPUTE WS-RESULT = WS-A + WS-B * WS-C.
           DISPLAY WS-RESULT.
      * Parenthesized expression
           COMPUTE WS-RESULT = (WS-A + WS-B) * WS-C.
           DISPLAY WS-RESULT.
      * Complex expression
           COMPUTE WS-RESULT =
               (WS-A * WS-B + WS-C) / (WS-B - WS-C).
           DISPLAY WS-RESULT.
      * Exponentiation
           COMPUTE WS-POWER-RESULT =
               WS-POWER-BASE ** WS-POWER-EXP.
           DISPLAY WS-POWER-RESULT.
      * Decimal arithmetic
           COMPUTE WS-DEC-RESULT =
               WS-DEC-A + WS-DEC-B * 2.
           DISPLAY WS-DEC-RESULT.
      * COMPUTE ROUNDED
           COMPUTE WS-ROUND-RESULT ROUNDED =
               WS-DEC-A / 3.
           DISPLAY WS-ROUND-RESULT.
      * COMPUTE without ROUNDED (truncation)
           COMPUTE WS-DEC-RESULT = WS-DEC-A / 3.
           DISPLAY WS-DEC-RESULT.
      * Packed decimal COMPUTE
           COMPUTE WS-PACK-RESULT =
               WS-PACK-A * WS-PACK-B - 100.50.
           DISPLAY WS-PACK-RESULT.
      * Mixed types in COMPUTE
           COMPUTE WS-RESULT =
               WS-A + WS-DEC-A + WS-COMP-VAL.
           DISPLAY WS-RESULT.
      * COMPUTE ON SIZE ERROR (overflow)
           COMPUTE WS-SMALL = 500 * 500
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-COMPUTE.
           DISPLAY WS-ERR-FLAG.
           DISPLAY WS-SMALL.
      * Nested parentheses
           COMPUTE WS-RESULT =
               ((WS-A + WS-B) * (WS-C + 5)) / 2.
           DISPLAY WS-RESULT.
      * Unary minus
           COMPUTE WS-DEC-RESULT = -WS-DEC-A + WS-DEC-B.
           DISPLAY WS-DEC-RESULT.
      * Literal-heavy expression
           COMPUTE WS-INT-RESULT = 10 + 20 * 3 - 5.
           DISPLAY WS-INT-RESULT.
           STOP RUN.
