      *---------------------------------------------------------------
      * WSTEST.cbl -- Test copybook in WORKING-STORAGE (not FILE SECTION)
      *---------------------------------------------------------------
       IDENTIFICATION DIVISION.
       PROGRAM-ID. WSTEST.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-CARD-DATA.
           COPY ACCT-MASTER.
           COPY CARDHOLDER-INFO.
       01  WS-COUNTER             PIC 9(5) VALUE ZEROS.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 'SA' TO ACCT-TYPE.
           MOVE 'John' TO CH-FIRST-NAME.
           DISPLAY ACCT-NUMBER.
           DISPLAY CH-LAST-NAME.
           STOP RUN.
