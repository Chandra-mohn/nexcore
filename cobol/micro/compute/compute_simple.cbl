      * Test: COMPUTE R = A + B. Expected output: 00300
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-SIMPLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(5) VALUE 100.
       01 WS-B PIC 9(5) VALUE 200.
       01 WS-R PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE WS-R = WS-A + WS-B.
           DISPLAY WS-R.
           STOP RUN.
