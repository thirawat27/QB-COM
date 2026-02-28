' LOOP.BAS
' Demonstrates various loop constructs

PRINT "Loop Demonstrations"
PRINT "==================="
PRINT

' FOR loop
PRINT "FOR loop (1 to 5):"
FOR i = 1 TO 5
    PRINT "  i ="; i
NEXT i
PRINT

' FOR loop with STEP
PRINT "FOR loop with STEP 2 (1 to 10):"
FOR i = 1 TO 10 STEP 2
    PRINT "  i ="; i
NEXT i
PRINT

' WHILE loop
PRINT "WHILE loop (countdown from 5):"
DIM n AS INTEGER
n = 5
WHILE n > 0
    PRINT "  n ="; n
    n = n - 1
WEND
PRINT

' DO WHILE loop
PRINT "DO WHILE loop (count to 3):"
DIM m AS INTEGER
m = 1
DO WHILE m <= 3
    PRINT "  m ="; m
    m = m + 1
LOOP
PRINT

PRINT "All loops completed!"

END
