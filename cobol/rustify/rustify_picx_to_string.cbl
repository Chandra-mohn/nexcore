       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-PICX-TO-STRING-TEST.
      *---------------------------------------------------------------
      * Rustification test: PicX -> String
      * Tier 2a - Type promotion. Tests that PIC X fields can be
      * promoted to Rust String type.
      * Edge cases: trailing space semantics, right-truncation,
      * concatenation via STRING, comparison, SPACES/LOW-VALUES.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-NAME                 PIC X(20) VALUE "JOHN DOE".
       01  WS-SHORT                PIC X(5) VALUE "ABCDE".
       01  WS-LONG                 PIC X(30) VALUE SPACES.
       01  WS-EMPTY                PIC X(10) VALUE SPACES.
       01  WS-ZEROS-STR            PIC X(5) VALUE ZEROS.
       01  WS-PADDED               PIC X(10) VALUE "HI".
       01  WS-TRIMMED              PIC X(10) VALUE SPACES.
       01  WS-COMPARE-A            PIC X(10) VALUE "APPLE".
       01  WS-COMPARE-B            PIC X(10) VALUE "BANANA".
       01  WS-CONCAT-RESULT        PIC X(40) VALUE SPACES.
       01  WS-PTR                  PIC 9(3) VALUE 1.
       01  WS-SEARCH-STR           PIC X(20) VALUE "HELLO WORLD".
       01  WS-FOUND                PIC X(10) VALUE SPACES.
       01  WS-DELIM                PIC X VALUE " ".
       01  WS-PART1                PIC X(10) VALUE SPACES.
       01  WS-PART2                PIC X(10) VALUE SPACES.
       01  WS-MOVE-TARGET          PIC X(5) VALUE SPACES.
       01  WS-LONG-SRC             PIC X(20) VALUE "TRUNCATION TEST!!!!".
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Basic display with trailing spaces
           DISPLAY WS-NAME.
           DISPLAY WS-SHORT.
           DISPLAY WS-PADDED.
      * Comparison (space-padded semantics)
           IF WS-COMPARE-A < WS-COMPARE-B
               DISPLAY "APPLE < BANANA"
           ELSE
               DISPLAY "APPLE >= BANANA"
           END-IF.
      * Check for SPACES
           IF WS-EMPTY = SPACES
               DISPLAY "EMPTY IS SPACES"
           END-IF.
      * MOVE between PIC X fields (right-truncation)
           MOVE WS-LONG-SRC TO WS-MOVE-TARGET.
           DISPLAY WS-MOVE-TARGET.
      * MOVE shorter into longer (space pad)
           MOVE WS-SHORT TO WS-LONG.
           DISPLAY ">" WS-LONG "<".
      * STRING concatenation
           STRING WS-NAME DELIMITED BY SPACES
                  " - " DELIMITED BY SIZE
                  WS-SHORT DELIMITED BY SIZE
                  INTO WS-CONCAT-RESULT
                  WITH POINTER WS-PTR.
           DISPLAY WS-CONCAT-RESULT.
      * UNSTRING (split on delimiter)
           MOVE 0 TO WS-PTR.
           UNSTRING WS-SEARCH-STR
               DELIMITED BY WS-DELIM
               INTO WS-PART1 WS-PART2.
           DISPLAY WS-PART1.
           DISPLAY WS-PART2.
      * Reference modification (substring)
           DISPLAY WS-NAME(1:4).
      * ZEROS in alpha field
           DISPLAY WS-ZEROS-STR.
      * INSPECT REPLACING
           MOVE "AABBAAC" TO WS-SHORT.
           INSPECT WS-SHORT REPLACING ALL "A" BY "X".
           DISPLAY WS-SHORT.
           STOP RUN.
