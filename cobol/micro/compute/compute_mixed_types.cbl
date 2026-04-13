      * Test: COMPUTE with mixed numeric types (9(5) + S9(5)V99). Expected output: 000015025
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-MIXED-TYPES-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(5) VALUE 100.
       01 WS-B PIC S9(5)V99 VALUE 50.25.
       01 WS-R PIC 9(7)V99.
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE WS-R = WS-A + WS-B.
           DISPLAY WS-R.
           STOP RUN.
