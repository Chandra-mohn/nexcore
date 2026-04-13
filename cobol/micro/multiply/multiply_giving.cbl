      * Tests MULTIPLY A BY B GIVING C (result in C, A and B unchanged). Expected: 00100
       IDENTIFICATION DIVISION.
       PROGRAM-ID. MULTIPLY-GIVING-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 25.
       01 WS-B PIC 9(3) VALUE 4.
       01 WS-C PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           MULTIPLY WS-A BY WS-B GIVING WS-C.
           DISPLAY WS-C.
           STOP RUN.
