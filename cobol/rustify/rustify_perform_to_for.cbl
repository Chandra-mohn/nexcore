       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-PERFORM-TO-FOR-TEST.
      *---------------------------------------------------------------
      * Rustification test: PERFORM VARYING -> for/while loops
      * Tier 2b - Control flow. Tests that PERFORM statements can
      * be converted to Rust for/while loops.
      * Edge cases: BY -1 decrementing, WITH TEST AFTER, nested
      * VARYING with AFTER, UNTIL condition, PERFORM n TIMES.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-I                    PIC 9(3) VALUE 0.
       01  WS-J                    PIC 9(3) VALUE 0.
       01  WS-K                    PIC 9(3) VALUE 0.
       01  WS-SUM                  PIC 9(9) VALUE 0.
       01  WS-PRODUCT              PIC 9(9) VALUE 1.
       01  WS-COUNT                PIC 9(3) VALUE 0.
       01  WS-RESULT               PIC 9(9) VALUE 0.
       01  WS-FLAG                 PIC 9 VALUE 0.
       01  WS-TABLE.
           05  WS-ENTRY            PIC 9(3)
                                   OCCURS 5 TIMES.
       01  WS-IDX                  PIC 9(3) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * PERFORM n TIMES
           MOVE 0 TO WS-COUNT.
           PERFORM 5 TIMES
               ADD 1 TO WS-COUNT
           END-PERFORM.
           DISPLAY WS-COUNT.
      * PERFORM VARYING (count up)
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               ADD WS-I TO WS-SUM
           END-PERFORM.
           DISPLAY WS-SUM.
      * PERFORM VARYING (count down BY -1)
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 5 BY -1
               UNTIL WS-I < 1
               ADD WS-I TO WS-SUM
           END-PERFORM.
           DISPLAY WS-SUM.
      * PERFORM VARYING BY 2 (step)
           MOVE 0 TO WS-COUNT.
           PERFORM VARYING WS-I FROM 1 BY 2
               UNTIL WS-I > 10
               ADD 1 TO WS-COUNT
           END-PERFORM.
           DISPLAY WS-COUNT.
      * PERFORM UNTIL (while loop)
           MOVE 1 TO WS-I.
           MOVE 1 TO WS-PRODUCT.
           PERFORM UNTIL WS-I > 5
               MULTIPLY WS-I BY WS-PRODUCT
               ADD 1 TO WS-I
           END-PERFORM.
           DISPLAY WS-PRODUCT.
      * PERFORM WITH TEST AFTER
           MOVE 0 TO WS-COUNT.
           MOVE 1 TO WS-I.
           PERFORM WITH TEST AFTER
               UNTIL WS-I > 3
               ADD 1 TO WS-COUNT
               ADD 1 TO WS-I
           END-PERFORM.
           DISPLAY WS-COUNT.
      * Nested PERFORM VARYING (2D iteration)
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               PERFORM VARYING WS-J FROM 1 BY 1
                   UNTIL WS-J > 3
                   COMPUTE WS-RESULT = WS-I * WS-J
                   ADD WS-RESULT TO WS-SUM
               END-PERFORM
           END-PERFORM.
           DISPLAY WS-SUM.
      * PERFORM VARYING with table fill
           PERFORM VARYING WS-IDX FROM 1 BY 1
               UNTIL WS-IDX > 5
               COMPUTE WS-ENTRY(WS-IDX) = WS-IDX * 10
           END-PERFORM.
           PERFORM VARYING WS-IDX FROM 1 BY 1
               UNTIL WS-IDX > 5
               DISPLAY WS-ENTRY(WS-IDX)
           END-PERFORM.
      * PERFORM paragraph
           PERFORM COMPUTE-PARA.
           DISPLAY WS-RESULT.
           STOP RUN.
       COMPUTE-PARA.
           COMPUTE WS-RESULT = 42 * 2 + 1.
