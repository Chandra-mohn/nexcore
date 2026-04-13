      * Tests ADD with COMP-3 (packed decimal) fields. Expected output: +00300
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-PACKED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC S9(5) COMP-3 VALUE 100.
           01 WS-B PIC S9(5) COMP-3 VALUE 200.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A TO WS-B.
           DISPLAY WS-B.
           STOP RUN.
