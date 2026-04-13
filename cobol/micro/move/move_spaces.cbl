      * Test: MOVE SPACES to PIC X(5). Expected output: "     " (5 spaces)
       IDENTIFICATION DIVISION.
       PROGRAM-ID. MOVE-SPACES-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-DST PIC X(5) VALUE "HELLO".
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE SPACES TO WS-DST
           DISPLAY WS-DST
           STOP RUN.
