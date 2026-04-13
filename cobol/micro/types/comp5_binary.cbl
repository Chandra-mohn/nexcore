      * Tests PIC 9(4) COMP-5 full-binary range (value exceeds PIC 9(4) max). Expected: 50000
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-COMP5-BINARY-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(4) COMP-5 VALUE 50000.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
