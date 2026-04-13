       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-MOVE-TO-ASSIGN-TEST.
      *---------------------------------------------------------------
      * Rustification test: cobol_move() -> direct assignment
      * Tier 1 - Cosmetic. Tests that MOVE statements between
      * compatible types can become simple Rust assignments.
      * Edge cases: left-truncation (12345 -> PIC 9(3) = 345),
      * space padding, sign preservation, type mismatch.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-SRC-INT              PIC 9(5) VALUE 12345.
       01  WS-DST-SMALL            PIC 9(3) VALUE 0.
       01  WS-DST-LARGE            PIC 9(7) VALUE 0.
       01  WS-SRC-ALPHA            PIC X(10) VALUE "ABCDEFGHIJ".
       01  WS-DST-SHORT            PIC X(5) VALUE SPACES.
       01  WS-DST-LONG             PIC X(15) VALUE SPACES.
       01  WS-SRC-SDEC             PIC S9(5)V99 VALUE -123.45.
       01  WS-DST-SDEC             PIC S9(5)V99 VALUE 0.
       01  WS-DST-UDEC             PIC 9(5)V99 VALUE 0.
       01  WS-SRC-PACK             PIC S9(7)V99 COMP-3 VALUE 500.25.
       01  WS-DST-PACK             PIC S9(7)V99 COMP-3 VALUE 0.
       01  WS-SRC-COMP             PIC S9(9) COMP VALUE 999.
       01  WS-DST-COMP             PIC S9(9) COMP VALUE 0.
       01  WS-DST-TINY             PIC 9(2) VALUE 0.
       01  WS-SRC-SIGN             PIC S9(3) VALUE -50.
       01  WS-DST-USIGN            PIC 9(3) VALUE 0.
       01  WS-SRC-LIT              PIC X(3) VALUE "XYZ".
       01  WS-DST-LIT              PIC X(8) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Same-type same-size MOVE (safe to simplify)
           MOVE WS-SRC-SDEC TO WS-DST-SDEC.
           DISPLAY WS-DST-SDEC.
      * Same-type larger destination
           MOVE WS-SRC-INT TO WS-DST-LARGE.
           DISPLAY WS-DST-LARGE.
      * LEFT-TRUNCATION: 12345 -> PIC 9(3) = 345
           MOVE WS-SRC-INT TO WS-DST-SMALL.
           DISPLAY WS-DST-SMALL.
      * Alpha truncation (right-truncate)
           MOVE WS-SRC-ALPHA TO WS-DST-SHORT.
           DISPLAY WS-DST-SHORT.
      * Alpha padding (space fill right)
           MOVE WS-SRC-LIT TO WS-DST-LIT.
           DISPLAY WS-DST-LIT.
           DISPLAY ">" WS-DST-LIT "<".
      * Alpha into longer field
           MOVE WS-SRC-ALPHA TO WS-DST-LONG.
           DISPLAY WS-DST-LONG.
      * Signed to unsigned (sign lost)
           MOVE WS-SRC-SIGN TO WS-DST-USIGN.
           DISPLAY WS-DST-USIGN.
      * Packed to packed
           MOVE WS-SRC-PACK TO WS-DST-PACK.
           DISPLAY WS-DST-PACK.
      * COMP to COMP
           MOVE WS-SRC-COMP TO WS-DST-COMP.
           DISPLAY WS-DST-COMP.
      * Large into tiny (numeric left-truncation)
           MOVE 9876 TO WS-DST-TINY.
           DISPLAY WS-DST-TINY.
      * Signed decimal to unsigned decimal
           MOVE WS-SRC-SDEC TO WS-DST-UDEC.
           DISPLAY WS-DST-UDEC.
      * Literal MOVE
           MOVE "NEW" TO WS-DST-SHORT.
           DISPLAY WS-DST-SHORT.
           MOVE 0 TO WS-DST-LARGE.
           DISPLAY WS-DST-LARGE.
           STOP RUN.
