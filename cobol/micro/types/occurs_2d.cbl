      * Tests 2D array OCCURS within OCCURS. Expected: 011 012 021 022
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-OCCURS-2D-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-MATRIX.
          05 WS-ROW OCCURS 2 TIMES.
             10 WS-COL PIC 9(3) OCCURS 2 TIMES.
       01 WS-I PIC 9(3) VALUE 0.
       01 WS-J PIC 9(3) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 11 TO WS-COL(1, 1).
           MOVE 12 TO WS-COL(1, 2).
           MOVE 21 TO WS-COL(2, 1).
           MOVE 22 TO WS-COL(2, 2).
           PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 2
               PERFORM VARYING WS-J FROM 1 BY 1 UNTIL WS-J > 2
                   DISPLAY WS-COL(WS-I, WS-J)
               END-PERFORM
           END-PERFORM.
           STOP RUN.
