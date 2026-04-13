      * Tests PIC S9(5) COMP-3 packed decimal positive. Expected output: +12345
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-COMP3-BASIC-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC S9(5) COMP-3 VALUE 12345.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
