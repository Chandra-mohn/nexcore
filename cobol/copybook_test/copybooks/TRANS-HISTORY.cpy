      *---------------------------------------------------------------
      * TRANS-HISTORY.cpy -- Transaction history (OCCURS group)
      * Shared across: CARDPROC, REPORT
      * Contains OCCURS for child table extraction test
      *---------------------------------------------------------------
           05  TH-TRANS-COUNT       PIC 9(03).
           05  TH-TRANS-ENTRY OCCURS 50 TIMES
               DEPENDING ON TH-TRANS-COUNT.
               10  TH-TRANS-ID      PIC X(12).
               10  TH-TRANS-DATE    PIC 9(08).
               10  TH-TRANS-TYPE    PIC X(02).
                   88  TH-PURCHASE  VALUE 'PU'.
                   88  TH-PAYMENT   VALUE 'PA'.
                   88  TH-REFUND    VALUE 'RF'.
                   88  TH-FEE       VALUE 'FE'.
               10  TH-TRANS-AMOUNT  PIC S9(11)V99 COMP-3.
               10  TH-TRANS-DESC    PIC X(40).
               10  TH-MERCHANT-ID   PIC X(15).
               10  TH-TRANS-DETAIL  PIC X(60).
               10  TH-TRANS-PURCHASE REDEFINES TH-TRANS-DETAIL.
                   15  TH-PUR-CATEGORY  PIC X(10).
                   15  TH-PUR-MCC       PIC 9(04).
                   15  TH-PUR-AUTH-CODE PIC X(06).
                   15  FILLER           PIC X(40).
               10  TH-TRANS-PAYMENT REDEFINES TH-TRANS-DETAIL.
                   15  TH-PAY-METHOD    PIC X(02).
                   15  TH-PAY-REF-NUM   PIC X(20).
                   15  TH-PAY-BANK      PIC X(25).
                   15  FILLER           PIC X(13).
