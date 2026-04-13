      * Test: MOVE PIC X(5) to PIC X(5) same size. Expected output: HELLO
       IDENTIFICATION DIVISION.
       PROGRAM-ID. MOVE-ALPHA-TO-ALPHA-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-SRC PIC X(5) VALUE "HELLO".
       01 WS-DST PIC X(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE WS-SRC TO WS-DST
           DISPLAY WS-DST
           STOP RUN.
