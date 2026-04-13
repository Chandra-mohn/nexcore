      * Tests level 88 condition name with SET. Expected: ACTIVE then AC
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TYPE-LEVEL88-BASIC-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-STATUS PIC X(2) VALUE SPACES.
          88 IS-ACTIVE VALUE "AC".
          88 IS-CLOSED VALUE "CL".
       PROCEDURE DIVISION.
       MAIN-PARA.
           SET IS-ACTIVE TO TRUE.
           IF IS-ACTIVE
               DISPLAY "ACTIVE"
           END-IF.
           DISPLAY WS-STATUS.
           STOP RUN.
