      * Tests PIC Z,ZZ9.99 numeric edited. Expected output: 1,234.56
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-NUMERIC-EDITED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC Z,ZZ9.99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 1234.56 TO WS-A
           DISPLAY WS-A
           STOP RUN.
