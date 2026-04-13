       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-LEVEL88-TO-ENUM-TEST.
      *---------------------------------------------------------------
      * Rustification test: Level-88 -> enum/bool
      * Tier 3a - Data structures. Tests that level-88 condition
      * names can be converted to Rust enums or bool constants.
      * Edge cases: multiple 88s on same field, THRU ranges,
      * SET statement, nested 88 checks, numeric 88s.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-STATUS               PIC X(2) VALUE SPACES.
           88  STATUS-ACTIVE       VALUE "AC".
           88  STATUS-INACTIVE     VALUE "IN".
           88  STATUS-PENDING      VALUE "PE".
           88  STATUS-CLOSED       VALUE "CL".
       01  WS-GRADE                PIC 9 VALUE 0.
           88  GRADE-EXCELLENT     VALUE 1.
           88  GRADE-GOOD          VALUE 2.
           88  GRADE-AVERAGE       VALUE 3.
           88  GRADE-POOR          VALUE 4 THRU 5.
           88  GRADE-VALID         VALUE 1 THRU 5.
       01  WS-YESNO                PIC X VALUE "N".
           88  IS-YES              VALUE "Y".
           88  IS-NO               VALUE "N".
       01  WS-MONTH                PIC 99 VALUE 0.
           88  MONTH-WINTER        VALUE 12, 1, 2.
           88  MONTH-SPRING        VALUE 3 THRU 5.
           88  MONTH-SUMMER        VALUE 6 THRU 8.
           88  MONTH-FALL          VALUE 9 THRU 11.
       01  WS-ERROR-CODE           PIC 9(3) VALUE 0.
           88  NO-ERROR            VALUE 0.
           88  WARN-LEVEL          VALUE 1 THRU 99.
           88  ERROR-LEVEL         VALUE 100 THRU 499.
           88  FATAL-LEVEL         VALUE 500 THRU 999.
       01  WS-RESULT               PIC X(20) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * SET to establish condition
           SET STATUS-ACTIVE TO TRUE.
           DISPLAY WS-STATUS.
      * Test 88-level condition
           IF STATUS-ACTIVE
               DISPLAY "STATUS IS ACTIVE"
           END-IF.
      * Switch condition
           SET STATUS-PENDING TO TRUE.
           DISPLAY WS-STATUS.
           EVALUATE TRUE
               WHEN STATUS-ACTIVE
                   MOVE "ACTIVE" TO WS-RESULT
               WHEN STATUS-INACTIVE
                   MOVE "INACTIVE" TO WS-RESULT
               WHEN STATUS-PENDING
                   MOVE "PENDING" TO WS-RESULT
               WHEN STATUS-CLOSED
                   MOVE "CLOSED" TO WS-RESULT
               WHEN OTHER
                   MOVE "UNKNOWN" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * Numeric 88 with THRU
           MOVE 4 TO WS-GRADE.
           IF GRADE-POOR
               DISPLAY "GRADE IS POOR"
           END-IF.
           IF GRADE-VALID
               DISPLAY "GRADE IS VALID"
           END-IF.
      * Boolean-style 88
           SET IS-YES TO TRUE.
           IF IS-YES
               DISPLAY "ANSWER IS YES"
           END-IF.
           SET IS-NO TO TRUE.
           IF IS-NO
               DISPLAY "ANSWER IS NO"
           END-IF.
           DISPLAY WS-YESNO.
      * Multiple values (MONTH-WINTER = 12, 1, 2)
           MOVE 1 TO WS-MONTH.
           IF MONTH-WINTER
               DISPLAY "JANUARY IS WINTER"
           END-IF.
           MOVE 12 TO WS-MONTH.
           IF MONTH-WINTER
               DISPLAY "DECEMBER IS WINTER"
           END-IF.
           MOVE 6 TO WS-MONTH.
           IF MONTH-SUMMER
               DISPLAY "JUNE IS SUMMER"
           END-IF.
      * Error code ranges
           MOVE 0 TO WS-ERROR-CODE.
           IF NO-ERROR
               DISPLAY "NO ERROR"
           END-IF.
           MOVE 50 TO WS-ERROR-CODE.
           IF WARN-LEVEL
               DISPLAY "WARNING LEVEL"
           END-IF.
           MOVE 250 TO WS-ERROR-CODE.
           IF ERROR-LEVEL
               DISPLAY "ERROR LEVEL"
           END-IF.
           MOVE 750 TO WS-ERROR-CODE.
           IF FATAL-LEVEL
               DISPLAY "FATAL LEVEL"
           END-IF.
      * Combined conditions with 88
           SET STATUS-ACTIVE TO TRUE.
           MOVE 1 TO WS-GRADE.
           IF STATUS-ACTIVE AND GRADE-EXCELLENT
               DISPLAY "ACTIVE AND EXCELLENT"
           END-IF.
           STOP RUN.
