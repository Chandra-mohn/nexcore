      * Tests ADD with ROUNDED clause. Expected output: 0010.6
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-ROUNDED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(5)V99 VALUE 10.55.
           01 WS-B PIC 9(4)V9  VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD WS-A TO WS-B ROUNDED.
           DISPLAY WS-B.
           STOP RUN.
