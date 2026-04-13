      * Tests PIC A(10) alphabetic. Expected output: HELLO     (space-padded to 10)
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-ALPHA-EDITED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC A(10) VALUE "HELLO".
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
