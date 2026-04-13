      * Tests EVALUATE with no matching WHEN, falls to OTHER. Expected output: NONE
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-OTHER-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-CODE PIC 9 VALUE 9.
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE WS-CODE
               WHEN 1
                   DISPLAY "ONE"
               WHEN 2
                   DISPLAY "TWO"
               WHEN OTHER
                   DISPLAY "NONE"
           END-EVALUATE
           STOP RUN.
