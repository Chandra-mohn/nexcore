      * Test: COMPUTE * before + precedence (10 + 5 * 6). Expected output: 00040
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-PRECEDENCE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-R PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE WS-R = 10 + 5 * 6.
           DISPLAY WS-R.
           STOP RUN.
