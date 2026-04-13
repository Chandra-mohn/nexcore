      * Tests EVALUATE with THRU ranges. Expected output: B
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-THRU-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-SCORE PIC 9(3) VALUE 85.
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE WS-SCORE
               WHEN 90 THRU 100
                   DISPLAY "A"
               WHEN 80 THRU 89
                   DISPLAY "B"
               WHEN 70 THRU 79
                   DISPLAY "C"
               WHEN OTHER
                   DISPLAY "F"
           END-EVALUATE
           STOP RUN.
