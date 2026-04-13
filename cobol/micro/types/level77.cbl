      * Tests level 77 standalone item. Expected: 00042
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-LEVEL77-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       77 WS-COUNTER PIC 9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           ADD 42 TO WS-COUNTER.
           DISPLAY WS-COUNTER.
           STOP RUN.
