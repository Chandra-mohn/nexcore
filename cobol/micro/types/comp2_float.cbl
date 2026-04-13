      * Tests COMP-2 double precision float. Expected: implementation-defined float display
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-COMP2-FLOAT-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A COMP-2 VALUE 3.14159265.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
