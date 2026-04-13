      * Tests simple 01 group with 05 children. Expected: 042 then BOB
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-GROUP-SIMPLE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-REC.
          05 WS-ID   PIC 9(3) VALUE 42.
          05 WS-NAME PIC X(5) VALUE "BOB".
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-ID.
           DISPLAY WS-NAME.
           STOP RUN.
