      * Tests EVALUATE with ALSO (multi-subject). Expected output: H2-N
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-ALSO-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-MONTH PIC 99 VALUE 7.
       01 WS-REGION PIC X(5) VALUE "NORTH".
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE WS-MONTH ALSO WS-REGION
               WHEN 1 THRU 6 ALSO ANY
                   DISPLAY "H1"
               WHEN 7 THRU 12 ALSO "NORTH"
                   DISPLAY "H2-N"
               WHEN OTHER
                   DISPLAY "OTHER"
           END-EVALUATE
           STOP RUN.
