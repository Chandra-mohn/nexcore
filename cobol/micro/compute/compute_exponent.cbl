      * Test: COMPUTE with ** exponentiation (2 ** 10). Expected output: 01024
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-EXPONENT-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-R PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE WS-R = 2 ** 10.
           DISPLAY WS-R.
           STOP RUN.
