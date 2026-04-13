      * Test: Nested IF 3 levels deep. Expected output: ALL PASS
       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-NESTED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC 9(3) VALUE 50.
       01 WS-B PIC 9(3) VALUE 75.
       01 WS-C PIC 9(3) VALUE 100.
       PROCEDURE DIVISION.
       MAIN-PARA.
           IF WS-A > 25
               IF WS-B > 50
                   IF WS-C > 75
                       DISPLAY "ALL PASS"
                   END-IF
               END-IF
           END-IF
           STOP RUN.
