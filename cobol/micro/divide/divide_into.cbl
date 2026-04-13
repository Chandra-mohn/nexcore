      * Tests DIVIDE A INTO B (B = B / A, B modified in place). Expected output: 00020
       IDENTIFICATION DIVISION.
       PROGRAM-ID. DIVIDE-INTO-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 5.
       01 WS-B PIC 9(5) VALUE 100.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DIVIDE WS-A INTO WS-B.
           DISPLAY WS-B.
           STOP RUN.
