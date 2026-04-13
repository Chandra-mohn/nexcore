      * Tests nested group 01/05/10. Expected: AUSTIN     then 78701 then 1234
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-GROUP-NESTED-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-ADDR.
          05 WS-CITY PIC X(10) VALUE "AUSTIN".
          05 WS-ZIP.
             10 WS-ZIP5 PIC 9(5) VALUE 78701.
             10 WS-ZIP4 PIC 9(4) VALUE 1234.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY WS-CITY.
           DISPLAY WS-ZIP5.
           DISPLAY WS-ZIP4.
           STOP RUN.
