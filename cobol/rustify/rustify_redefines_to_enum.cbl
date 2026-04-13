       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-REDEFINES-TO-ENUM-TEST.
      *---------------------------------------------------------------
      * Rustification test: REDEFINES -> enum
      * Tier 3a - Data structures. Tests that REDEFINES can be
      * converted to Rust enums or union-like types.
      * Edge cases: write via one view read via other, different
      * field layouts over same memory, numeric/alpha redefines.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple numeric redefines
       01  WS-AMOUNT-NUM           PIC 9(8) VALUE 12345678.
       01  WS-AMOUNT-STR           REDEFINES WS-AMOUNT-NUM
                                   PIC X(8).
      * Date redefines
       01  WS-DATE-NUM             PIC 9(8) VALUE 20250315.
       01  WS-DATE-PARTS           REDEFINES WS-DATE-NUM.
           05  WS-DATE-YEAR        PIC 9(4).
           05  WS-DATE-MONTH       PIC 9(2).
           05  WS-DATE-DAY         PIC 9(2).
      * Group redefines
       01  WS-RECORD-A.
           05  WS-REC-TYPE         PIC X VALUE "A".
           05  WS-REC-A-NAME       PIC X(20) VALUE SPACES.
           05  WS-REC-A-AMT        PIC 9(7)V99 VALUE 0.
       01  WS-RECORD-B             REDEFINES WS-RECORD-A.
           05  WS-REC-B-TYPE       PIC X.
           05  WS-REC-B-CODE       PIC 9(5).
           05  WS-REC-B-DESC       PIC X(15).
           05  WS-REC-B-QTY        PIC 9(5).
           05  WS-REC-B-FILL       PIC X(4).
      * Numeric viewed as alpha (already covered by AMOUNT above)
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Write as numeric, read as string
           DISPLAY WS-AMOUNT-NUM.
           DISPLAY WS-AMOUNT-STR.
      * Date parts via redefines
           DISPLAY WS-DATE-NUM.
           DISPLAY WS-DATE-YEAR.
           DISPLAY WS-DATE-MONTH.
           DISPLAY WS-DATE-DAY.
      * Write as record type A
           MOVE "A" TO WS-REC-TYPE.
           MOVE "JOHN DOE" TO WS-REC-A-NAME.
           MOVE 1500.00 TO WS-REC-A-AMT.
           DISPLAY WS-REC-TYPE.
           DISPLAY WS-REC-A-NAME.
           DISPLAY WS-REC-A-AMT.
      * Read same memory as record type B
           DISPLAY WS-REC-B-TYPE.
           DISPLAY WS-REC-B-CODE.
           DISPLAY WS-REC-B-DESC.
      * Switch to type B layout
           MOVE "B" TO WS-REC-B-TYPE.
           MOVE 10001 TO WS-REC-B-CODE.
           MOVE "WIDGET" TO WS-REC-B-DESC.
           MOVE 250 TO WS-REC-B-QTY.
           DISPLAY WS-REC-B-TYPE.
           DISPLAY WS-REC-B-CODE.
           DISPLAY WS-REC-B-DESC.
           DISPLAY WS-REC-B-QTY.
      * Read back as type A (cross-view)
           DISPLAY WS-REC-A-NAME.
      * Verify date redefines after MOVE
           MOVE 20261225 TO WS-DATE-NUM.
           DISPLAY WS-DATE-YEAR.
           DISPLAY WS-DATE-MONTH.
           DISPLAY WS-DATE-DAY.
           STOP RUN.
