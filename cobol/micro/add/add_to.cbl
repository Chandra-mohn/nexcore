      * Tests ADD A TO B (B modified in place). Expected output: 00300
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-TO-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(5) VALUE 100.
           01 WS-B PIC 9(5) VALUE 200.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A TO WS-B.
           DISPLAY WS-B.
           STOP RUN.
