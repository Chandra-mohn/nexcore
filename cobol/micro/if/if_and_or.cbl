      * Test: IF with AND and OR conditions. Expected output: BOTH then EITHER
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-AND-OR-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 50.
       01 WS-B PIC 9(3) VALUE 75.
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A > 25 AND WS-B > 50
               DISPLAY "BOTH"
           END-IF
           IF WS-A > 100 OR WS-B > 50
               DISPLAY "EITHER"
           END-IF
           STOP RUN.
