      * Tests level 88 with THRU range. Expected: PASS
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-LEVEL88-THRU-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-GRADE PIC 9 VALUE 0.
          88 GRADE-PASS VALUE 1 THRU 3.
          88 GRADE-FAIL VALUE 4 THRU 5.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 2 TO WS-GRADE.
           IF GRADE-PASS
               DISPLAY "PASS"
           END-IF.
           STOP RUN.
