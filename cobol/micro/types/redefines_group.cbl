      * Tests group REDEFINES date -> parts. Expected: 2025 then 03 then 15
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-REDEFINES-GROUP-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-DATE  PIC 9(8) VALUE 20250315.
       01 WS-PARTS REDEFINES WS-DATE.
          05 WS-YR  PIC 9(4).
          05 WS-MO  PIC 9(2).
          05 WS-DY  PIC 9(2).
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-YR.
           DISPLAY WS-MO.
           DISPLAY WS-DY.
           STOP RUN.
