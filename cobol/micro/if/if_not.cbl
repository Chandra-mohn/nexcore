      * Test: IF with NOT condition. Expected output: NOT BIG
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-NOT-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 10.
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF NOT WS-A > 100
               DISPLAY "NOT BIG"
           END-IF
           STOP RUN.
