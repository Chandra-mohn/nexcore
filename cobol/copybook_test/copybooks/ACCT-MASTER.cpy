      *---------------------------------------------------------------
      * ACCT-MASTER.cpy -- Account master record layout
      * Shared across: CARDPROC, BILLING, REPORT
      *---------------------------------------------------------------
           05  ACCT-NUMBER          PIC X(16).
           05  ACCT-TYPE            PIC X(02).
               88  ACCT-SAVINGS     VALUE 'SA'.
               88  ACCT-CHECKING    VALUE 'CH'.
               88  ACCT-CREDIT      VALUE 'CR'.
           05  ACCT-STATUS          PIC X(01).
               88  ACCT-ACTIVE      VALUE 'A'.
               88  ACCT-CLOSED      VALUE 'C'.
               88  ACCT-FROZEN      VALUE 'F'.
           05  ACCT-OPEN-DATE       PIC 9(08).
           05  ACCT-BALANCE         PIC S9(13)V99 COMP-3.
           05  ACCT-CREDIT-LIMIT    PIC S9(13)V99 COMP-3.
           05  ACCT-LAST-ACTIVITY   PIC 9(08).
