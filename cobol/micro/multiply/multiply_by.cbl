      * Tests MULTIPLY A BY B (modifies B in place). Expected output: 00100
       IDENTIFICATION DIVISION.
       PROGRAM-ID. MULTIPLY-BY-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 25.
       01 WS-B PIC 9(5) VALUE 4.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MULTIPLY WS-A BY WS-B.
           DISPLAY WS-B.
           STOP RUN.
