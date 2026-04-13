       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-PACKED-TO-DECIMAL-TEST.
      *---------------------------------------------------------------
      * Rustification test: PackedDecimal(scale>0) -> Decimal
      * Tier 2a - Type promotion. Tests that COMP-3 fields with
      * decimal places can be promoted to rust_decimal::Decimal.
      * Edge cases: ROUNDED division, precision chain, scale
      * mismatch across operations.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-PRICE                PIC S9(5)V99 COMP-3 VALUE 123.45.
       01  WS-TAX-RATE             PIC V9(4) COMP-3 VALUE 0.0825.
       01  WS-TAX-AMT              PIC S9(5)V99 COMP-3 VALUE 0.
       01  WS-TOTAL                PIC S9(7)V99 COMP-3 VALUE 0.
       01  WS-DISCOUNT             PIC S9(3)V99 COMP-3 VALUE 10.50.
       01  WS-QTY                  PIC S9(3) COMP-3 VALUE 7.
       01  WS-UNIT-PRICE           PIC S9(5)V9(4) COMP-3 VALUE 0.
       01  WS-GROSS                PIC S9(7)V99 COMP-3 VALUE 1000.00.
       01  WS-PRECISION-A          PIC S9(3)V9(4) COMP-3 VALUE 1.1111.
       01  WS-PRECISION-B          PIC S9(3)V9(4) COMP-3 VALUE 2.2222.
       01  WS-PRECISION-C          PIC S9(3)V9(4) COMP-3 VALUE 0.
       01  WS-DIV-RESULT           PIC S9(5)V99 COMP-3 VALUE 0.
       01  WS-ROUND-RESULT         PIC S9(5)V99 COMP-3 VALUE 0.
       01  WS-SMALL-DEC            PIC S9(3)V99 COMP-3 VALUE 0.
       01  WS-ERR-FLAG             PIC 9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Tax calculation chain
           MULTIPLY WS-PRICE BY WS-TAX-RATE
               GIVING WS-TAX-AMT ROUNDED.
           DISPLAY WS-TAX-AMT.
           ADD WS-PRICE WS-TAX-AMT GIVING WS-TOTAL.
           DISPLAY WS-TOTAL.
      * Discount subtraction
           SUBTRACT WS-DISCOUNT FROM WS-TOTAL.
           DISPLAY WS-TOTAL.
      * Division with ROUNDED
           DIVIDE WS-GROSS BY WS-QTY
               GIVING WS-UNIT-PRICE ROUNDED.
           DISPLAY WS-UNIT-PRICE.
      * Division truncation (no ROUNDED)
           DIVIDE 100.00 BY 3 GIVING WS-DIV-RESULT.
           DISPLAY WS-DIV-RESULT.
      * Division ROUNDED
           DIVIDE 100.00 BY 3 GIVING WS-ROUND-RESULT ROUNDED.
           DISPLAY WS-ROUND-RESULT.
      * Precision chain: add two high-precision values
           ADD WS-PRECISION-A WS-PRECISION-B
               GIVING WS-PRECISION-C.
           DISPLAY WS-PRECISION-C.
      * Multiply with scale mismatch
           MOVE 99.99 TO WS-PRICE.
           MULTIPLY WS-PRICE BY WS-QTY
               GIVING WS-TOTAL.
           DISPLAY WS-TOTAL.
      * Overflow into small field
           MOVE 999.99 TO WS-SMALL-DEC.
           ADD 0.01 TO WS-SMALL-DEC
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-ADD.
           DISPLAY WS-ERR-FLAG.
           DISPLAY WS-SMALL-DEC.
      * Negative result chain
           MOVE -50.25 TO WS-PRICE.
           MOVE 2.00 TO WS-TAX-AMT.
           MULTIPLY WS-PRICE BY WS-TAX-AMT
               GIVING WS-TOTAL.
           DISPLAY WS-TOTAL.
           ADD 200.00 TO WS-TOTAL.
           DISPLAY WS-TOTAL.
           STOP RUN.
