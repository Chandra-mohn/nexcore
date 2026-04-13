      * Test: IF/ELSE false branch. Expected output: SMALL
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-ELSE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 10.
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A > 25
               DISPLAY "BIG"
           ELSE
               DISPLAY "SMALL"
           END-IF
           STOP RUN.
