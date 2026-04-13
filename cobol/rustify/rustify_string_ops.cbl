       IDENTIFICATION DIVISION.
       PROGRAM-ID. RUSTIFY-STRING-OPS-TEST.
      *---------------------------------------------------------------
      * Rustification test: STRING/UNSTRING -> String methods
      * Tier 3b - Compound operations. Tests that STRING and
      * UNSTRING statements can use native Rust String methods.
      * Edge cases: overflow, multiple delimiters, POINTER,
      * TALLYING, ALL delimiter, partial matches.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-FIRST-NAME           PIC X(10) VALUE "JOHN".
       01  WS-LAST-NAME            PIC X(15) VALUE "DOE".
       01  WS-FULL-NAME            PIC X(30) VALUE SPACES.
       01  WS-PTR                  PIC 9(3) VALUE 1.
       01  WS-CSV-LINE             PIC X(50)
               VALUE "ALICE,30,NEW YORK,10001".
       01  WS-FIELD1               PIC X(10) VALUE SPACES.
       01  WS-FIELD2               PIC X(10) VALUE SPACES.
       01  WS-FIELD3               PIC X(15) VALUE SPACES.
       01  WS-FIELD4               PIC X(10) VALUE SPACES.
       01  WS-DELIM                PIC X VALUE ",".
       01  WS-TALLY                PIC 9(3) VALUE 0.
       01  WS-OVERFLOW             PIC X(10) VALUE SPACES.
       01  WS-SHORT-BUF            PIC X(5) VALUE SPACES.
       01  WS-ADDR-LINE            PIC X(40) VALUE SPACES.
       01  WS-CITY                 PIC X(15) VALUE "SPRINGFIELD".
       01  WS-STATE                PIC X(2) VALUE "IL".
       01  WS-ZIP                  PIC X(5) VALUE "62701".
       01  WS-MULTI-DELIM          PIC X(30)
               VALUE "ONE::TWO::THREE".
       01  WS-MD-PART1             PIC X(10) VALUE SPACES.
       01  WS-MD-PART2             PIC X(10) VALUE SPACES.
       01  WS-MD-PART3             PIC X(10) VALUE SPACES.
       01  WS-PADDED               PIC X(20) VALUE SPACES.
       01  WS-COUNT-A              PIC 9(3) VALUE 0.
       01  WS-SRC                  PIC X(20) VALUE "AABBAACAA".
       01  WS-REPLACED             PIC X(20) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Basic STRING concatenation
           MOVE 1 TO WS-PTR.
           STRING WS-FIRST-NAME DELIMITED BY SPACES
                  " " DELIMITED BY SIZE
                  WS-LAST-NAME DELIMITED BY SPACES
                  INTO WS-FULL-NAME
                  WITH POINTER WS-PTR.
           DISPLAY WS-FULL-NAME.
           DISPLAY WS-PTR.
      * STRING with ON OVERFLOW
           MOVE 1 TO WS-PTR.
           STRING "THIS IS A VERY LONG STRING"
                  DELIMITED BY SIZE
                  INTO WS-SHORT-BUF
                  WITH POINTER WS-PTR
                  ON OVERFLOW
                      DISPLAY "STRING OVERFLOW"
           END-STRING.
           DISPLAY WS-SHORT-BUF.
      * UNSTRING CSV line
           UNSTRING WS-CSV-LINE
               DELIMITED BY WS-DELIM
               INTO WS-FIELD1 WS-FIELD2 WS-FIELD3 WS-FIELD4
               TALLYING IN WS-TALLY.
           DISPLAY WS-FIELD1.
           DISPLAY WS-FIELD2.
           DISPLAY WS-FIELD3.
           DISPLAY WS-FIELD4.
           DISPLAY WS-TALLY.
      * STRING address line
           MOVE 1 TO WS-PTR.
           STRING WS-CITY DELIMITED BY SPACES
                  ", " DELIMITED BY SIZE
                  WS-STATE DELIMITED BY SIZE
                  " " DELIMITED BY SIZE
                  WS-ZIP DELIMITED BY SIZE
                  INTO WS-ADDR-LINE
                  WITH POINTER WS-PTR.
           DISPLAY WS-ADDR-LINE.
      * UNSTRING with multi-char delimiter
           UNSTRING WS-MULTI-DELIM
               DELIMITED BY "::"
               INTO WS-MD-PART1 WS-MD-PART2 WS-MD-PART3.
           DISPLAY WS-MD-PART1.
           DISPLAY WS-MD-PART2.
           DISPLAY WS-MD-PART3.
      * INSPECT TALLYING
           MOVE 0 TO WS-COUNT-A.
           INSPECT WS-SRC TALLYING WS-COUNT-A
               FOR ALL "A".
           DISPLAY WS-COUNT-A.
      * INSPECT REPLACING
           MOVE WS-SRC TO WS-REPLACED.
           INSPECT WS-REPLACED REPLACING ALL "A" BY "X".
           DISPLAY WS-REPLACED.
      * STRING multiple literals
           MOVE SPACES TO WS-PADDED.
           MOVE 1 TO WS-PTR.
           STRING "(" DELIMITED BY SIZE
                  "123" DELIMITED BY SIZE
                  ") " DELIMITED BY SIZE
                  "456-7890" DELIMITED BY SIZE
                  INTO WS-PADDED
                  WITH POINTER WS-PTR.
           DISPLAY WS-PADDED.
           STOP RUN.
