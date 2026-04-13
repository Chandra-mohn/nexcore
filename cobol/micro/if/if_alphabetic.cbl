      * Test: IF ALPHABETIC class condition. Expected output: IS ALPHA
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-ALPHABETIC-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC X(5) VALUE "HELLO".
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A IS ALPHABETIC
               DISPLAY "IS ALPHA"
           END-IF
           STOP RUN.
