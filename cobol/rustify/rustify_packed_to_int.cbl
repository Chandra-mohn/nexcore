       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-PACKED-TO-INT-TEST.
      *---------------------------------------------------------------
      * Rustification test: PackedDecimal(scale=0) -> i32/i64
      * Tier 2a - Type promotion. Tests that COMP-3 fields with
      * zero decimal places can be promoted to native integers.
      * Edge cases: overflow (999+1 into PIC 9(3)), near-max i64,
      * unsigned receiving negative, sign handling.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-PACK-SMALL           PIC 9(3) COMP-3 VALUE 100.
       01  WS-PACK-MED             PIC S9(5) COMP-3 VALUE -500.
       01  WS-PACK-LARGE           PIC S9(9) COMP-3 VALUE 123456789.
       01  WS-PACK-VLARGE          PIC S9(18) COMP-3 VALUE 0.
       01  WS-PACK-ZERO            PIC S9(5) COMP-3 VALUE 0.
       01  WS-PACK-ONE             PIC S9(5) COMP-3 VALUE 1.
       01  WS-PACK-NEG             PIC S9(5) COMP-3 VALUE -999.
       01  WS-RESULT-SMALL         PIC 9(3) COMP-3 VALUE 0.
       01  WS-RESULT-MED           PIC S9(5) COMP-3 VALUE 0.
       01  WS-RESULT-LARGE         PIC S9(9) COMP-3 VALUE 0.
       01  WS-UNSIGNED             PIC 9(5) COMP-3 VALUE 0.
       01  WS-ERR-FLAG             PIC 9 VALUE 0.
       01  WS-DISPLAY-VAL          PIC 9(9) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Basic packed display
           DISPLAY WS-PACK-SMALL.
           DISPLAY WS-PACK-MED.
           DISPLAY WS-PACK-LARGE.
      * Arithmetic on packed integers
           ADD WS-PACK-SMALL TO WS-RESULT-MED.
           DISPLAY WS-RESULT-MED.
           SUBTRACT 200 FROM WS-RESULT-MED.
           DISPLAY WS-RESULT-MED.
           MULTIPLY WS-PACK-SMALL BY WS-RESULT-MED.
           DISPLAY WS-RESULT-MED.
      * Division (integer truncation)
           MOVE 100 TO WS-RESULT-MED.
           DIVIDE 3 INTO WS-RESULT-MED.
           DISPLAY WS-RESULT-MED.
      * Overflow: 999 + 1 into PIC 9(3) COMP-3
           MOVE 999 TO WS-RESULT-SMALL.
           ADD 1 TO WS-RESULT-SMALL
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-ADD.
           DISPLAY WS-ERR-FLAG.
           DISPLAY WS-RESULT-SMALL.
      * Negative into unsigned
           MOVE WS-PACK-NEG TO WS-UNSIGNED.
           DISPLAY WS-UNSIGNED.
      * Near-max value
           MOVE 999999999 TO WS-PACK-LARGE.
           DISPLAY WS-PACK-LARGE.
           ADD 1 TO WS-PACK-LARGE
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
           END-ADD.
           DISPLAY WS-ERR-FLAG.
      * Large packed arithmetic
           MOVE 100000000 TO WS-PACK-VLARGE.
           MULTIPLY 100 BY WS-PACK-VLARGE.
           DISPLAY WS-PACK-VLARGE.
      * MOVE packed to display numeric
           MOVE WS-PACK-SMALL TO WS-DISPLAY-VAL.
           DISPLAY WS-DISPLAY-VAL.
      * Comparisons
           IF WS-PACK-MED < WS-PACK-ZERO
               DISPLAY "NEG < ZERO"
           END-IF.
           IF WS-PACK-LARGE > 0
               DISPLAY "LARGE > ZERO"
           END-IF.
           STOP RUN.
