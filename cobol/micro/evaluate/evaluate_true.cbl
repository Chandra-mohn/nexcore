      * Tests EVALUATE TRUE with conditions. Expected output: MED
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-TRUE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-SCORE PIC 9(3) VALUE 42.
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE TRUE
               WHEN WS-SCORE > 75
                   DISPLAY "HIGH"
               WHEN WS-SCORE > 25
                   DISPLAY "MED"
               WHEN OTHER
                   DISPLAY "LOW"
           END-EVALUATE
           STOP RUN.
