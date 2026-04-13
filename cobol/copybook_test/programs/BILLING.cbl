      *---------------------------------------------------------------
      * BILLING.cbl -- Billing statement generator
      * Uses ACCT-MASTER + CARDHOLDER-INFO + BILLING-ADDRESS
      * Tests: subset of copybooks, sequential output file.
      *---------------------------------------------------------------
       IDENTIFICATION DIVISION.
       PROGRAM-ID. BILLING.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT CARD-FILE
               ASSIGN TO 'CARDMAST'
               ORGANIZATION IS INDEXED
               ACCESS MODE IS SEQUENTIAL
               RECORD KEY IS ACCT-NUMBER
               FILE STATUS IS WS-FILE-STATUS.

           SELECT STATEMENT-FILE
               ASSIGN TO 'STMTOUT'
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-STMT-STAT.

       DATA DIVISION.
       FILE SECTION.
       FD  CARD-FILE.
       01  CARD-RECORD.
           COPY ACCT-MASTER.
           COPY CARDHOLDER-INFO.
           COPY BILLING-ADDRESS.

       FD  STATEMENT-FILE.
       01  STATEMENT-RECORD        PIC X(132).

       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS          PIC XX.
           88  WS-SUCCESS          VALUE '00'.
           88  WS-EOF              VALUE '10'.
       01  WS-STMT-STAT          PIC XX.
       01  WS-STMT-COUNT           PIC 9(07) VALUE ZEROS.

       01  WS-STMT-LINE.
           05  WS-STMT-NAME        PIC X(56).
           05  WS-STMT-ACCT        PIC X(16).
           05  WS-STMT-BAL         PIC Z(12)9.99-.
           05  WS-STMT-LIMIT       PIC Z(12)9.99.
           05  WS-STMT-ACCT-STAT  PIC X(10).
           05  FILLER              PIC X(15).

       PROCEDURE DIVISION.
       0000-MAIN.
           OPEN INPUT CARD-FILE.
           OPEN OUTPUT STATEMENT-FILE.
           PERFORM 1000-PROCESS
               UNTIL WS-EOF.
           CLOSE CARD-FILE.
           CLOSE STATEMENT-FILE.
           DISPLAY 'STATEMENTS: ' WS-STMT-COUNT.
           STOP RUN.

       1000-PROCESS.
           READ CARD-FILE
               AT END SET WS-EOF TO TRUE
               NOT AT END
                   PERFORM 2000-BUILD-STATEMENT
           END-READ.

       2000-BUILD-STATEMENT.
           IF ACCT-ACTIVE
               STRING CH-FIRST-NAME DELIMITED SPACES
                      ' ' DELIMITED SIZE
                      CH-LAST-NAME DELIMITED SPACES
                   INTO WS-STMT-NAME
               END-STRING
               MOVE ACCT-NUMBER TO WS-STMT-ACCT
               MOVE ACCT-BALANCE TO WS-STMT-BAL
               MOVE ACCT-CREDIT-LIMIT TO WS-STMT-LIMIT
               EVALUATE TRUE
                   WHEN ACCT-ACTIVE
                       MOVE 'ACTIVE' TO WS-STMT-ACCT-STAT
                   WHEN ACCT-FROZEN
                       MOVE 'FROZEN' TO WS-STMT-ACCT-STAT
               END-EVALUATE
               WRITE STATEMENT-RECORD FROM WS-STMT-LINE
               ADD 1 TO WS-STMT-COUNT
           END-IF.
