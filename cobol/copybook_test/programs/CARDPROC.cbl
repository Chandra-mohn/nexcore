      *---------------------------------------------------------------
      * CARDPROC.cbl -- Card processing program
      * Uses all 4 copybooks in one 01-level KSDS record.
      * Tests: copybook boundary detection, OCCURS child table,
      *        REDEFINES inside OCCURS, RECORD KEY IS for PK.
      *---------------------------------------------------------------
       IDENTIFICATION DIVISION.
       PROGRAM-ID. CARDPROC.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT CARD-MASTER-FILE
               ASSIGN TO 'CARDMAST'
               ORGANIZATION IS INDEXED
               ACCESS MODE IS DYNAMIC
               RECORD KEY IS ACCT-NUMBER
               FILE STATUS IS WS-FILE-STATUS.

       DATA DIVISION.
       FILE SECTION.
       FD  CARD-MASTER-FILE.
       01  CARD-MASTER-RECORD.
           COPY ACCT-MASTER.
           COPY CARDHOLDER-INFO.
           COPY BILLING-ADDRESS.
           COPY TRANS-HISTORY.

       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS          PIC XX.
           88  WS-SUCCESS          VALUE '00'.
           88  WS-EOF              VALUE '10'.
       01  WS-RECORD-COUNT         PIC 9(07) VALUE ZEROS.
       01  WS-ERROR-COUNT          PIC 9(05) VALUE ZEROS.
       01  WS-TOTAL-BALANCE        PIC S9(15)V99 COMP-3
                                   VALUE ZEROS.
       01  WS-CURRENT-DATE         PIC 9(08).

       PROCEDURE DIVISION.
       0000-MAIN.
           OPEN I-O CARD-MASTER-FILE.
           IF NOT WS-SUCCESS
               DISPLAY 'ERROR OPENING FILE: ' WS-FILE-STATUS
               STOP RUN
           END-IF.
           PERFORM 1000-PROCESS-RECORDS
               UNTIL WS-EOF.
           PERFORM 9000-CLOSE-FILES.
           STOP RUN.

       1000-PROCESS-RECORDS.
           READ CARD-MASTER-FILE NEXT
               AT END SET WS-EOF TO TRUE
               NOT AT END
                   ADD 1 TO WS-RECORD-COUNT
                   PERFORM 2000-VALIDATE-ACCOUNT
                   PERFORM 3000-PROCESS-TRANSACTIONS
           END-READ.

       2000-VALIDATE-ACCOUNT.
           IF NOT ACCT-ACTIVE
               ADD 1 TO WS-ERROR-COUNT
           ELSE
               ADD ACCT-BALANCE TO WS-TOTAL-BALANCE
           END-IF.

           EVALUATE ACCT-TYPE
               WHEN 'SA'
                   PERFORM 2100-PROCESS-SAVINGS
               WHEN 'CH'
                   PERFORM 2200-PROCESS-CHECKING
               WHEN 'CR'
                   PERFORM 2300-PROCESS-CREDIT
               WHEN OTHER
                   ADD 1 TO WS-ERROR-COUNT
           END-EVALUATE.

       2100-PROCESS-SAVINGS.
           IF ACCT-BALANCE > ACCT-CREDIT-LIMIT
               DISPLAY 'SAVINGS OVERLIMIT: ' ACCT-NUMBER
           END-IF.

       2200-PROCESS-CHECKING.
           IF ACCT-BALANCE < ZEROS
               DISPLAY 'CHECKING OVERDRAFT: ' ACCT-NUMBER
           END-IF.

       2300-PROCESS-CREDIT.
           IF ACCT-BALANCE > ACCT-CREDIT-LIMIT
               DISPLAY 'CREDIT OVERLIMIT: ' ACCT-NUMBER
           END-IF.

       3000-PROCESS-TRANSACTIONS.
           PERFORM VARYING TH-TRANS-COUNT
               FROM 1 BY 1
               UNTIL TH-TRANS-COUNT > 50
               EVALUATE TH-TRANS-TYPE(TH-TRANS-COUNT)
                   WHEN 'PU'
                       PERFORM 3100-PROCESS-PURCHASE
                   WHEN 'PA'
                       PERFORM 3200-PROCESS-PAYMENT
                   WHEN 'RF'
                       PERFORM 3300-PROCESS-REFUND
               END-EVALUATE
           END-PERFORM.

       3100-PROCESS-PURCHASE.
           DISPLAY 'PURCHASE: '
               TH-PUR-CATEGORY(TH-TRANS-COUNT)
               ' MCC=' TH-PUR-MCC(TH-TRANS-COUNT).

       3200-PROCESS-PAYMENT.
           DISPLAY 'PAYMENT: '
               TH-PAY-METHOD(TH-TRANS-COUNT)
               ' REF=' TH-PAY-REF-NUM(TH-TRANS-COUNT).

       3300-PROCESS-REFUND.
           DISPLAY 'REFUND: '
               TH-TRANS-AMOUNT(TH-TRANS-COUNT).

       9000-CLOSE-FILES.
           CLOSE CARD-MASTER-FILE.
           DISPLAY 'RECORDS: ' WS-RECORD-COUNT.
           DISPLAY 'ERRORS:  ' WS-ERROR-COUNT.
           DISPLAY 'BALANCE: ' WS-TOTAL-BALANCE.
