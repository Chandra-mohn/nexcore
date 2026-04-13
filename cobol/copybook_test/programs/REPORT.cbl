      *---------------------------------------------------------------
      * REPORT.cbl -- Transaction report generator
      * Uses ACCT-MASTER + TRANS-HISTORY
      * Tests: sequential read of same KSDS file, cross-program key
      *        discovery (BILLING declares key, REPORT reads same file
      *        sequentially).
      *---------------------------------------------------------------
       IDENTIFICATION DIVISION.
       PROGRAM-ID. REPORT.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT CARD-FILE
               ASSIGN TO 'CARDMAST'
               ORGANIZATION IS INDEXED
               ACCESS MODE IS SEQUENTIAL
               RECORD KEY IS ACCT-NUMBER
               FILE STATUS IS WS-FILE-STATUS.

           SELECT REPORT-FILE
               ASSIGN TO 'RPTOUT'
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-RPT-STATUS.

       DATA DIVISION.
       FILE SECTION.
       FD  CARD-FILE.
       01  CARD-RECORD.
           COPY ACCT-MASTER.
           COPY TRANS-HISTORY.

       FD  REPORT-FILE.
       01  REPORT-RECORD           PIC X(132).

       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS          PIC XX.
           88  WS-SUCCESS          VALUE '00'.
           88  WS-EOF              VALUE '10'.
       01  WS-RPT-STATUS           PIC XX.
       01  WS-RECORD-COUNT         PIC 9(07) VALUE ZEROS.
       01  WS-TRANS-TOTAL          PIC S9(15)V99 COMP-3
                                   VALUE ZEROS.
       01  WS-PURCHASE-COUNT       PIC 9(07) VALUE ZEROS.
       01  WS-PAYMENT-COUNT        PIC 9(07) VALUE ZEROS.
       01  WS-REFUND-COUNT         PIC 9(07) VALUE ZEROS.

       01  WS-RPT-LINE.
           05  WS-RPT-ACCT         PIC X(16).
           05  FILLER              PIC X(02) VALUE SPACES.
           05  WS-RPT-TYPE         PIC X(02).
           05  FILLER              PIC X(02) VALUE SPACES.
           05  WS-RPT-DATE         PIC 9(08).
           05  FILLER              PIC X(02) VALUE SPACES.
           05  WS-RPT-AMT          PIC Z(10)9.99-.
           05  FILLER              PIC X(02) VALUE SPACES.
           05  WS-RPT-DESC         PIC X(40).
           05  FILLER              PIC X(33) VALUE SPACES.

       PROCEDURE DIVISION.
       0000-MAIN.
           OPEN INPUT CARD-FILE.
           OPEN OUTPUT REPORT-FILE.
           PERFORM 1000-PROCESS
               UNTIL WS-EOF.
           PERFORM 9000-PRINT-SUMMARY.
           CLOSE CARD-FILE.
           CLOSE REPORT-FILE.
           STOP RUN.

       1000-PROCESS.
           READ CARD-FILE
               AT END SET WS-EOF TO TRUE
               NOT AT END
                   ADD 1 TO WS-RECORD-COUNT
                   PERFORM 2000-PROCESS-TRANS
           END-READ.

       2000-PROCESS-TRANS.
           PERFORM VARYING TH-TRANS-COUNT
               FROM 1 BY 1
               UNTIL TH-TRANS-COUNT > 50
               MOVE ACCT-NUMBER TO WS-RPT-ACCT
               MOVE TH-TRANS-TYPE(TH-TRANS-COUNT)
                   TO WS-RPT-TYPE
               MOVE TH-TRANS-DATE(TH-TRANS-COUNT)
                   TO WS-RPT-DATE
               MOVE TH-TRANS-AMOUNT(TH-TRANS-COUNT)
                   TO WS-RPT-AMT
               MOVE TH-TRANS-DESC(TH-TRANS-COUNT)
                   TO WS-RPT-DESC
               EVALUATE TH-TRANS-TYPE(TH-TRANS-COUNT)
                   WHEN 'PU'
                       ADD 1 TO WS-PURCHASE-COUNT
                   WHEN 'PA'
                       ADD 1 TO WS-PAYMENT-COUNT
                   WHEN 'RF'
                       ADD 1 TO WS-REFUND-COUNT
               END-EVALUATE
               ADD TH-TRANS-AMOUNT(TH-TRANS-COUNT)
                   TO WS-TRANS-TOTAL
               WRITE REPORT-RECORD FROM WS-RPT-LINE
           END-PERFORM.

       9000-PRINT-SUMMARY.
           DISPLAY 'RECORDS:    ' WS-RECORD-COUNT.
           DISPLAY 'PURCHASES:  ' WS-PURCHASE-COUNT.
           DISPLAY 'PAYMENTS:   ' WS-PAYMENT-COUNT.
           DISPLAY 'REFUNDS:    ' WS-REFUND-COUNT.
           DISPLAY 'TOTAL:      ' WS-TRANS-TOTAL.
