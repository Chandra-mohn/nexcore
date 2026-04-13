      * Tests ADD A B C GIVING D (multiple operands). Expected output: 00060
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-MULTIPLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(5) VALUE 10.
           01 WS-B PIC 9(5) VALUE 20.
           01 WS-C PIC 9(5) VALUE 30.
           01 WS-D PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A WS-B WS-C GIVING WS-D.
           DISPLAY WS-D.
           STOP RUN.
