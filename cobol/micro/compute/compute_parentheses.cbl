      * Test: COMPUTE with parentheses overriding precedence ((10 + 5) * 6). Expected output: 00090
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-PARENTHESES-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-R PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE WS-R = (10 + 5) * 6.
           DISPLAY WS-R.
           STOP RUN.
