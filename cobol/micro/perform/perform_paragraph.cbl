      * Tests PERFORM a named paragraph. Expected output: 00042
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-PARAGRAPH-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
           01 WS-R PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM CALC-PARA
           DISPLAY WS-R
           STOP RUN.
       CALC-PARA.
           COMPUTE WS-R = 6 * 7.
