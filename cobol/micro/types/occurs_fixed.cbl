      * Tests fixed OCCURS with 1-based indexing. Expected: 010 then 020 then 030
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-OCCURS-FIXED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-TBL.
          05 WS-ITEM PIC 9(3) OCCURS 3 TIMES.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 10 TO WS-ITEM(1).
           MOVE 20 TO WS-ITEM(2).
           MOVE 30 TO WS-ITEM(3).
           DISPLAY WS-ITEM(1).
           DISPLAY WS-ITEM(2).
           DISPLAY WS-ITEM(3).
           STOP RUN.
