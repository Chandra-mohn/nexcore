      * Tests PIC X initialized with SPACES figurative constant. Expected: >     <
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-PICX-SPACES-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-A PIC X(5) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY ">" WS-A "<"
           STOP RUN.
