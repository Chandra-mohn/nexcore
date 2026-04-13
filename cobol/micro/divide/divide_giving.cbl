      * Tests DIVIDE A BY B GIVING C (integer quotient stored in C). Expected output: 00033
       IDENTIFICATION DIVISION.
       PROGRAM-ID. DIVIDE-GIVING-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(5) VALUE 100.
       01 WS-B PIC 9(3) VALUE 3.
       01 WS-C PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           DIVIDE WS-A BY WS-B GIVING WS-C.
           DISPLAY WS-C.
           STOP RUN.
