       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-GROUP-TO-STRUCT-TEST.
      *---------------------------------------------------------------
      * Rustification test: GROUP -> native struct
      * Tier 3a - Data structures. Tests that GROUP records can be
      * promoted to native Rust structs.
      * Edge cases: FILLER omission, flat byte copy (GROUP MOVE
      * blocks transformation), nested groups, mixed types.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple group
       01  WS-PERSON.
           05  WS-PERSON-ID        PIC 9(5) VALUE 0.
           05  WS-PERSON-NAME      PIC X(20) VALUE SPACES.
           05  WS-PERSON-AGE       PIC 9(3) VALUE 0.
      * Group with FILLER
       01  WS-HEADER.
           05  FILLER              PIC X(5) VALUE "HDR: ".
           05  WS-HDR-CODE         PIC 9(3) VALUE 0.
           05  FILLER              PIC X(2) VALUE ": ".
           05  WS-HDR-DESC         PIC X(20) VALUE SPACES.
      * Nested group
       01  WS-ORDER.
           05  WS-ORD-NUM          PIC 9(6) VALUE 0.
           05  WS-ORD-CUSTOMER.
               10  WS-CUST-ID     PIC 9(5) VALUE 0.
               10  WS-CUST-NAME   PIC X(20) VALUE SPACES.
           05  WS-ORD-AMOUNT       PIC S9(7)V99 VALUE 0.
      * Mixed types group
       01  WS-MIXED.
           05  WS-MX-ALPHA         PIC X(10) VALUE SPACES.
           05  WS-MX-INT           PIC 9(5) VALUE 0.
           05  WS-MX-DEC           PIC S9(5)V99 VALUE 0.
           05  WS-MX-PACK          PIC S9(7)V99 COMP-3 VALUE 0.
           05  WS-MX-COMP          PIC S9(9) COMP VALUE 0.
      * Group MOVE target
       01  WS-COPY-PERSON.
           05  WS-CP-ID            PIC 9(5) VALUE 0.
           05  WS-CP-NAME          PIC X(20) VALUE SPACES.
           05  WS-CP-AGE           PIC 9(3) VALUE 0.
      * Flat display target (for GROUP MOVE)
       01  WS-FLAT                 PIC X(80) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Fill and display simple group
           MOVE 10001 TO WS-PERSON-ID.
           MOVE "ALICE SMITH" TO WS-PERSON-NAME.
           MOVE 30 TO WS-PERSON-AGE.
           DISPLAY WS-PERSON-ID.
           DISPLAY WS-PERSON-NAME.
           DISPLAY WS-PERSON-AGE.
      * Group-level display (flat bytes)
           DISPLAY WS-PERSON.
      * FILLER group display
           MOVE 42 TO WS-HDR-CODE.
           MOVE "INVENTORY REPORT" TO WS-HDR-DESC.
           DISPLAY WS-HEADER.
      * Nested group fill and display
           MOVE 100001 TO WS-ORD-NUM.
           MOVE 20001 TO WS-CUST-ID.
           MOVE "BOB JONES" TO WS-CUST-NAME.
           MOVE 1250.75 TO WS-ORD-AMOUNT.
           DISPLAY WS-ORD-NUM.
           DISPLAY WS-CUST-ID.
           DISPLAY WS-CUST-NAME.
           DISPLAY WS-ORD-AMOUNT.
      * GROUP MOVE (flat byte copy - blocks struct promotion)
           MOVE WS-PERSON TO WS-COPY-PERSON.
           DISPLAY WS-CP-ID.
           DISPLAY WS-CP-NAME.
           DISPLAY WS-CP-AGE.
      * GROUP MOVE to flat field
           MOVE WS-PERSON TO WS-FLAT.
           DISPLAY WS-FLAT.
      * Mixed types group
           MOVE "MIXED TEST" TO WS-MX-ALPHA.
           MOVE 12345 TO WS-MX-INT.
           MOVE -678.90 TO WS-MX-DEC.
           MOVE 1111.11 TO WS-MX-PACK.
           MOVE 99999 TO WS-MX-COMP.
           DISPLAY WS-MX-ALPHA.
           DISPLAY WS-MX-INT.
           DISPLAY WS-MX-DEC.
           DISPLAY WS-MX-PACK.
           DISPLAY WS-MX-COMP.
      * Nested group display
           DISPLAY WS-ORD-CUSTOMER.
           STOP RUN.
