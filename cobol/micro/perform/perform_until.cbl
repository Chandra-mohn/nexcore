      * Tests PERFORM UNTIL (while loop). Expected output: 00015
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-UNTIL-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-I   PIC 9(3) VALUE 1.
           01 WS-SUM PIC 9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM UNTIL WS-I > 5
               ADD WS-I TO WS-SUM
               ADD 1 TO WS-I
           END-PERFORM
           DISPLAY WS-SUM
           STOP RUN.
