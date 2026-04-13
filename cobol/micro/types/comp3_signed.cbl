      * Tests PIC S9(5) COMP-3 packed decimal negative. Expected output: -00999
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-COMP3-SIGNED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC S9(5) COMP-3 VALUE -999.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
