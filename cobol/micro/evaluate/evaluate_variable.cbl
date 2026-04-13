      * Tests EVALUATE variable WHEN "value". Expected output: GOOD
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-VARIABLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-GRADE PIC X VALUE "B".
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE WS-GRADE
               WHEN "A"
                   DISPLAY "EXCELLENT"
               WHEN "B"
                   DISPLAY "GOOD"
               WHEN OTHER
                   DISPLAY "OTHER"
           END-EVALUATE
           STOP RUN.
