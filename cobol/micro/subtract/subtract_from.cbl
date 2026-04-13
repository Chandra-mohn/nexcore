      * Tests SUBTRACT A FROM B (B modified in place). Expected output: 00400
       IDENTIFICATION DIVISION.
       PROGRAM-ID. SUBTRACT-FROM-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(5) VALUE 100.
           01 WS-B PIC 9(5) VALUE 500.
       PROCEDURE DIVISION.
       MAIN-PARA.
           SUBTRACT WS-A FROM WS-B.
           DISPLAY WS-B.
           STOP RUN.
