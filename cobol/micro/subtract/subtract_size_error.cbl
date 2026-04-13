      * Tests SUBTRACT underflow into unsigned triggers ON SIZE ERROR. Expected output: UNDERFLOW
       IDENTIFICATION DIVISION.
       PROGRAM-ID. SUBTRACT-SIZE-ERROR-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-A PIC 9(3) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           SUBTRACT 1 FROM WS-A
               ON SIZE ERROR DISPLAY "UNDERFLOW"
           END-SUBTRACT.
           STOP RUN.
