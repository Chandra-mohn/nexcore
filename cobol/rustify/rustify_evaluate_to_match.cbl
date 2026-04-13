       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-EVALUATE-TO-MATCH-TEST.
      *---------------------------------------------------------------
      * Rustification test: EVALUATE -> match expression
      * Tier 2b - Control flow. Tests that EVALUATE statements can
      * be converted to Rust match expressions.
      * Edge cases: THRU ranges, WHEN OTHER, nested EVALUATE,
      * multi-subject ALSO, EVALUATE TRUE, fall-through semantics.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-GRADE                PIC X VALUE SPACES.
       01  WS-SCORE                PIC 9(3) VALUE 0.
       01  WS-RESULT               PIC X(30) VALUE SPACES.
       01  WS-MONTH                PIC 99 VALUE 0.
       01  WS-REGION               PIC X(5) VALUE SPACES.
       01  WS-SEASON               PIC X(10) VALUE SPACES.
       01  WS-CODE                 PIC 99 VALUE 0.
       01  WS-STATUS               PIC X(2) VALUE SPACES.
       01  WS-TYPE                 PIC X VALUE SPACES.
       01  WS-PRIORITY             PIC 9 VALUE 0.
       01  WS-ACTION               PIC X(20) VALUE SPACES.
       01  WS-DAY                  PIC 9 VALUE 0.
       01  WS-DAY-NAME             PIC X(10) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple EVALUATE variable
           MOVE "B" TO WS-GRADE.
           EVALUATE WS-GRADE
               WHEN "A"
                   MOVE "EXCELLENT" TO WS-RESULT
               WHEN "B"
                   MOVE "GOOD" TO WS-RESULT
               WHEN "C"
                   MOVE "AVERAGE" TO WS-RESULT
               WHEN "D"
                   MOVE "BELOW AVERAGE" TO WS-RESULT
               WHEN "F"
                   MOVE "FAILING" TO WS-RESULT
               WHEN OTHER
                   MOVE "UNKNOWN" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE with THRU ranges
           MOVE 85 TO WS-SCORE.
           EVALUATE WS-SCORE
               WHEN 90 THRU 100
                   MOVE "A GRADE" TO WS-RESULT
               WHEN 80 THRU 89
                   MOVE "B GRADE" TO WS-RESULT
               WHEN 70 THRU 79
                   MOVE "C GRADE" TO WS-RESULT
               WHEN 60 THRU 69
                   MOVE "D GRADE" TO WS-RESULT
               WHEN 0 THRU 59
                   MOVE "F GRADE" TO WS-RESULT
               WHEN OTHER
                   MOVE "INVALID SCORE" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE TRUE
           MOVE 42 TO WS-SCORE.
           EVALUATE TRUE
               WHEN WS-SCORE > 90
                   MOVE "HIGH" TO WS-RESULT
               WHEN WS-SCORE > 50
                   MOVE "MEDIUM" TO WS-RESULT
               WHEN WS-SCORE > 25
                   MOVE "LOW" TO WS-RESULT
               WHEN OTHER
                   MOVE "VERY LOW" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE with ALSO (multi-subject)
           MOVE 7 TO WS-MONTH.
           MOVE "NORTH" TO WS-REGION.
           EVALUATE WS-MONTH ALSO WS-REGION
               WHEN 1 THRU 3 ALSO "NORTH"
                   MOVE "Q1 NORTH" TO WS-RESULT
               WHEN 1 THRU 3 ALSO "SOUTH"
                   MOVE "Q1 SOUTH" TO WS-RESULT
               WHEN 4 THRU 6 ALSO ANY
                   MOVE "Q2 ANY" TO WS-RESULT
               WHEN 7 THRU 9 ALSO "NORTH"
                   MOVE "Q3 NORTH" TO WS-RESULT
               WHEN 7 THRU 9 ALSO ANY
                   MOVE "Q3 OTHER" TO WS-RESULT
               WHEN 10 THRU 12 ALSO ANY
                   MOVE "Q4 ANY" TO WS-RESULT
               WHEN OTHER
                   MOVE "UNKNOWN COMBO" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * Nested EVALUATE
           MOVE "OK" TO WS-STATUS.
           MOVE "A" TO WS-TYPE.
           EVALUATE WS-STATUS
               WHEN "OK"
                   EVALUATE WS-TYPE
                       WHEN "A"
                           MOVE 1 TO WS-CODE
                       WHEN "B"
                           MOVE 2 TO WS-CODE
                       WHEN OTHER
                           MOVE 3 TO WS-CODE
                   END-EVALUATE
               WHEN "ER"
                   MOVE 99 TO WS-CODE
               WHEN OTHER
                   MOVE 0 TO WS-CODE
           END-EVALUATE.
           DISPLAY WS-CODE.
      * EVALUATE TRUE ALSO FALSE
           MOVE 50 TO WS-SCORE.
           EVALUATE TRUE ALSO FALSE
               WHEN WS-SCORE > 25 ALSO WS-SCORE > 100
                   MOVE "MATCH TF" TO WS-RESULT
               WHEN OTHER
                   MOVE "NO MATCH TF" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * Day name lookup (sequential values)
           MOVE 3 TO WS-DAY.
           EVALUATE WS-DAY
               WHEN 1
                   MOVE "MONDAY" TO WS-DAY-NAME
               WHEN 2
                   MOVE "TUESDAY" TO WS-DAY-NAME
               WHEN 3
                   MOVE "WEDNESDAY" TO WS-DAY-NAME
               WHEN 4
                   MOVE "THURSDAY" TO WS-DAY-NAME
               WHEN 5
                   MOVE "FRIDAY" TO WS-DAY-NAME
               WHEN 6
                   MOVE "SATURDAY" TO WS-DAY-NAME
               WHEN 7
                   MOVE "SUNDAY" TO WS-DAY-NAME
               WHEN OTHER
                   MOVE "INVALID" TO WS-DAY-NAME
           END-EVALUATE.
           DISPLAY WS-DAY-NAME.
           STOP RUN.
