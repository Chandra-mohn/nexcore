      * Tests nested PERFORM VARYING (3x3 = 9 iterations). Expected output: 00009
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-NESTED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-I   PIC 9(3).
           01 WS-J   PIC 9(3).
           01 WS-SUM PIC 9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 3
               PERFORM VARYING WS-J FROM 1 BY 1 UNTIL WS-J > 3
                   ADD 1 TO WS-SUM
               END-PERFORM
           END-PERFORM
           DISPLAY WS-SUM
           STOP RUN.
