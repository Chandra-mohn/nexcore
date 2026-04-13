      * Tests ADD overflow triggers ON SIZE ERROR. Expected output: OVERFLOW
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADD-SIZE-ERROR-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(3) VALUE 999.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD 1 TO WS-A
               ON SIZE ERROR DISPLAY "OVERFLOW"
           END-ADD.
           STOP RUN.
