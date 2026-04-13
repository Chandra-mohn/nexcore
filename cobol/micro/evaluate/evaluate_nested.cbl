      * Tests nested EVALUATE statements. Expected output: OK-X
       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVALUATE-NESTED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC X(2) VALUE "OK".
       01 WS-B PIC X VALUE "X".
       PROCEDURE DIVISION.
       MAIN-PARA.
           EVALUATE WS-A
               WHEN "OK"
                   EVALUATE WS-B
                       WHEN "X"
                           DISPLAY "OK-X"
                       WHEN OTHER
                           DISPLAY "OK-?"
                   END-EVALUATE
               WHEN OTHER
                   DISPLAY "ERR"
           END-EVALUATE
           STOP RUN.
