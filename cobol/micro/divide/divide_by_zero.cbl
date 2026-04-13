      * Tests DIVIDE by zero triggers ON SIZE ERROR handler. Expected output: DIV-BY-ZERO
       IDENTIFICATION DIVISION.
       PROGRAM-ID. DIVIDE-BY-ZERO-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(5) VALUE 100.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DIVIDE 0 INTO WS-A
               ON SIZE ERROR DISPLAY "DIV-BY-ZERO"
           END-DIVIDE.
           STOP RUN.
