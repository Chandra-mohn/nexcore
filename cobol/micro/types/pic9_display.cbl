      * Tests PIC 9(5) unsigned display numeric. Expected output: 00042
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-PIC9-DISPLAY-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(5) VALUE 42.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-A
           STOP RUN.
