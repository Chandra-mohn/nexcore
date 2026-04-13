      * Tests PERFORM VARYING BY 2 (odd numbers 1,3,5,7,9). Expected output: 005
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-VARYING-BY-STEP-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-I     PIC 9(3).
           01 WS-COUNT PIC 9(3) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM VARYING WS-I FROM 1 BY 2 UNTIL WS-I > 10
               ADD 1 TO WS-COUNT
           END-PERFORM
           DISPLAY WS-COUNT
           STOP RUN.
