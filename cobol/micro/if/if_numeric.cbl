      * Test: IF NUMERIC class condition. Expected output: IS NUM
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-NUMERIC-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC X(5) VALUE "12345".
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A IS NUMERIC
               DISPLAY "IS NUM"
           END-IF
           STOP RUN.
