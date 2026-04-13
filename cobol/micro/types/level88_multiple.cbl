      * Tests level 88 with multiple VALUES. Expected: WEEKEND
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-LEVEL88-MULTIPLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-DAY PIC 9 VALUE 0.
          88 IS-WEEKEND VALUE 6, 7.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 7 TO WS-DAY.
           IF IS-WEEKEND
               DISPLAY "WEEKEND"
           END-IF.
           STOP RUN.
