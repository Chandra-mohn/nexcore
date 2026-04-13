      * Tests PIC S9(5) signed display numeric. Expected output: -00123
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-PIC9-SIGNED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC S9(5) VALUE -123.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
