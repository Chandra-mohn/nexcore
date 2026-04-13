#!/usr/bin/env bash
# count_token_errors.sh -- Count token recognition errors by token value.
# Usage: nexmig check ./cobol/abc.cbl -C ./cobol 2>&1 | ./scripts/count_token_errors.sh
#   or:  ./scripts/count_token_errors.sh < logfile.txt

grep "token recognition error at:" | \
    sed "s/.*token recognition error at: '//; s/'$//" | \
    sort | uniq -c | sort -rn
