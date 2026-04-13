      * Tests PIC X(10) alphanumeric. Expected output: HELLO     (trailing spaces)
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-PICX-BASIC-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC X(10) VALUE "HELLO".
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
