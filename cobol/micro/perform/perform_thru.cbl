      * Tests PERFORM paragraph THRU paragraph. Expected output: STEP1 / STEP2 / DONE
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-THRU-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM STEP-1 THRU STEP-2
           DISPLAY "DONE"
           STOP RUN.
       STEP-1.
           DISPLAY "STEP1".
       STEP-2.
           DISPLAY "STEP2".
