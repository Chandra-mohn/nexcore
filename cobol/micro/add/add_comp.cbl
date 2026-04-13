      * Tests ADD with COMP (binary) fields. Expected output: +000003000
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-COMP-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC S9(9) COMP VALUE 1000.
           01 WS-B PIC S9(9) COMP VALUE 2000.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A TO WS-B.
           DISPLAY WS-B.
           STOP RUN.
