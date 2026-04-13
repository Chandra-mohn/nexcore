       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-COMP-TO-NATIVE-TEST.
      *---------------------------------------------------------------
      * Rustification test: COMP/COMP-5 -> native int types
      * Tier 2a - Type promotion. Tests that COMP and COMP-5
      * fields can be promoted to native Rust integer types.
      * Edge cases: COMP PIC-limited range vs COMP-5 full binary,
      * mixed arithmetic, sign, boundary values.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-COMP-SMALL           PIC 9(4) COMP VALUE 1000.
       01  WS-COMP-MED             PIC S9(9) COMP VALUE -50000.
       01  WS-COMP-LARGE           PIC S9(18) COMP VALUE 0.
       01  WS-COMP5-SMALL          PIC 9(4) COMP-5 VALUE 1000.
       01  WS-COMP5-MED            PIC S9(9) COMP-5 VALUE -50000.
       01  WS-COMP5-LARGE          PIC S9(18) COMP-5 VALUE 0.
       01  WS-COMP-RESULT          PIC S9(9) COMP VALUE 0.
       01  WS-COMP5-RESULT         PIC S9(9) COMP-5 VALUE 0.
       01  WS-COMP-MAX             PIC 9(4) COMP VALUE 9999.
       01  WS-COMP5-OVER           PIC 9(4) COMP-5 VALUE 9999.
       01  WS-DISPLAY-VAL          PIC S9(18) VALUE 0.
       01  WS-ERR-FLAG             PIC 9 VALUE 0.
       01  WS-PACK-VAL             PIC S9(7)V99 COMP-3 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Basic COMP display
           DISPLAY WS-COMP-SMALL.
           DISPLAY WS-COMP-MED.
           DISPLAY WS-COMP5-SMALL.
           DISPLAY WS-COMP5-MED.
      * COMP arithmetic
           ADD WS-COMP-SMALL TO WS-COMP-RESULT.
           DISPLAY WS-COMP-RESULT.
           MULTIPLY 10 BY WS-COMP-RESULT.
           DISPLAY WS-COMP-RESULT.
      * COMP-5 arithmetic
           ADD WS-COMP5-SMALL TO WS-COMP5-RESULT.
           DISPLAY WS-COMP5-RESULT.
           MULTIPLY 10 BY WS-COMP5-RESULT.
           DISPLAY WS-COMP5-RESULT.
      * COMP overflow at PIC limit (9999 + 1)
           ADD 1 TO WS-COMP-MAX
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-ADD.
           DISPLAY WS-ERR-FLAG.
           DISPLAY WS-COMP-MAX.
      * COMP-5 beyond PIC limit (uses full binary range)
           ADD 1 TO WS-COMP5-OVER.
           DISPLAY WS-COMP5-OVER.
      * Mixed: COMP to COMP-3
           MOVE 12345 TO WS-COMP-RESULT.
           MOVE WS-COMP-RESULT TO WS-PACK-VAL.
           DISPLAY WS-PACK-VAL.
      * Mixed: COMP-5 to display
           MOVE 67890 TO WS-COMP5-RESULT.
           MOVE WS-COMP5-RESULT TO WS-DISPLAY-VAL.
           DISPLAY WS-DISPLAY-VAL.
      * Negative COMP
           MOVE -999 TO WS-COMP-RESULT.
           DISPLAY WS-COMP-RESULT.
      * Large value in COMP-5
           MOVE 999999999 TO WS-COMP5-LARGE.
           MULTIPLY 1000 BY WS-COMP5-LARGE.
           DISPLAY WS-COMP5-LARGE.
      * Comparison
           IF WS-COMP-MED < 0
               DISPLAY "COMP IS NEGATIVE"
           END-IF.
           IF WS-COMP5-LARGE > WS-COMP5-MED
               DISPLAY "COMP5-LARGE > COMP5-MED"
           END-IF.
           STOP RUN.
