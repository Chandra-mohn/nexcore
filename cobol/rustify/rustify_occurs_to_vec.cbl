       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-OCCURS-TO-VEC-TEST.
      *---------------------------------------------------------------
      * Rustification test: OCCURS -> Vec/array
      * Tier 3a - Data structures. Tests that OCCURS clauses can
      * be converted to Rust Vec or fixed-size arrays.
      * Edge cases: 1-based vs 0-based indexing, OCCURS DEPENDING
      * ON, 2D arrays, SEARCH, group OCCURS.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple fixed array
       01  WS-SCORES.
           05  WS-SCORE            PIC 9(3) OCCURS 5 TIMES.
      * Array with group entries
       01  WS-EMPLOYEES.
           05  WS-EMP              OCCURS 3 TIMES.
               10  WS-EMP-ID      PIC 9(5).
               10  WS-EMP-NAME    PIC X(15).
               10  WS-EMP-SAL     PIC 9(7)V99.
      * 2D array
       01  WS-MATRIX.
           05  WS-ROW              OCCURS 3 TIMES.
               10  WS-COL         PIC 9(3) OCCURS 4 TIMES.
      * OCCURS DEPENDING ON
       01  WS-ITEM-COUNT           PIC 9(2) VALUE 3.
       01  WS-ITEMS.
           05  WS-ITEM             PIC X(10)
                                   OCCURS 1 TO 10 TIMES
                                   DEPENDING ON WS-ITEM-COUNT.
      * Counters
       01  WS-I                    PIC 9(3) VALUE 0.
       01  WS-J                    PIC 9(3) VALUE 0.
       01  WS-SUM                  PIC 9(9) VALUE 0.
       01  WS-TOTAL-SAL            PIC 9(9)V99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Fill fixed array (1-based indexing)
           MOVE 90 TO WS-SCORE(1).
           MOVE 85 TO WS-SCORE(2).
           MOVE 78 TO WS-SCORE(3).
           MOVE 92 TO WS-SCORE(4).
           MOVE 88 TO WS-SCORE(5).
      * Display array elements
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               DISPLAY WS-SCORE(WS-I)
           END-PERFORM.
      * Sum array
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               ADD WS-SCORE(WS-I) TO WS-SUM
           END-PERFORM.
           DISPLAY WS-SUM.
      * Fill group array
           MOVE 10001 TO WS-EMP-ID(1).
           MOVE "ALICE" TO WS-EMP-NAME(1).
           MOVE 75000.00 TO WS-EMP-SAL(1).
           MOVE 10002 TO WS-EMP-ID(2).
           MOVE "BOB" TO WS-EMP-NAME(2).
           MOVE 82000.00 TO WS-EMP-SAL(2).
           MOVE 10003 TO WS-EMP-ID(3).
           MOVE "CHARLIE" TO WS-EMP-NAME(3).
           MOVE 69500.00 TO WS-EMP-SAL(3).
      * Display group array
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               DISPLAY WS-EMP-ID(WS-I) " "
                       WS-EMP-NAME(WS-I) " "
                       WS-EMP-SAL(WS-I)
           END-PERFORM.
      * Total salary
           MOVE 0 TO WS-TOTAL-SAL.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               ADD WS-EMP-SAL(WS-I) TO WS-TOTAL-SAL
           END-PERFORM.
           DISPLAY WS-TOTAL-SAL.
      * Fill 2D array
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               PERFORM VARYING WS-J FROM 1 BY 1
                   UNTIL WS-J > 4
                   COMPUTE WS-COL(WS-I, WS-J) =
                       WS-I * 10 + WS-J
               END-PERFORM
           END-PERFORM.
      * Display 2D array (one element per line)
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               PERFORM VARYING WS-J FROM 1 BY 1
                   UNTIL WS-J > 4
                   DISPLAY WS-COL(WS-I, WS-J)
               END-PERFORM
           END-PERFORM.
      * OCCURS DEPENDING ON
           MOVE "FIRST" TO WS-ITEM(1).
           MOVE "SECOND" TO WS-ITEM(2).
           MOVE "THIRD" TO WS-ITEM(3).
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > WS-ITEM-COUNT
               DISPLAY WS-ITEM(WS-I)
           END-PERFORM.
      * Change count and add item
           MOVE 4 TO WS-ITEM-COUNT.
           MOVE "FOURTH" TO WS-ITEM(4).
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > WS-ITEM-COUNT
               DISPLAY WS-ITEM(WS-I)
           END-PERFORM.
           STOP RUN.
