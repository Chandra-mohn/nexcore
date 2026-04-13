      * Tests COMP-1 single precision float. Expected: implementation-defined float display
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-COMP1-FLOAT-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A COMP-1 VALUE 3.14.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
