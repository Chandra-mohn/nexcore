      * Tests ADD A B GIVING C. Expected output: 00300
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-GIVING-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(5) VALUE 100.
           01 WS-B PIC 9(5) VALUE 200.
           01 WS-C PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A WS-B GIVING WS-C.
           DISPLAY WS-C.
           STOP RUN.
