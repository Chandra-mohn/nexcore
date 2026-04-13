      * Tests PERFORM VARYING count up 1 to 5. Expected output: 00015
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-VARYING-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-I   PIC 9(3).
           01 WS-SUM PIC 9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 5
               ADD WS-I TO WS-SUM
           END-PERFORM
           DISPLAY WS-SUM
           STOP RUN.
