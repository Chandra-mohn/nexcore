      * Test: Simple IF true branch. Expected output: PASS
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-SIMPLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 50.
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A > 25
               DISPLAY "PASS"
           END-IF
           STOP RUN.
