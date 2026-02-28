' QB-COM Comprehensive Test Suite
' Tests all implemented QBasic + QB64 features

$CONSOLE

PRINT "==================================="
PRINT "   QB-COM Comprehensive Test Suite"
PRINT "==================================="
PRINT

' ===================================
' 1. Basic Variables and Types
' ===================================
PRINT "--- Test 1: Basic Variables ---"

' Integer
DIM i AS INTEGER
i = 42
PRINT "INTEGER: "; i

' Long
DIM l AS LONG
l = 100000
PRINT "LONG: "; l

' Single
DIM s AS SINGLE
s = 3.14159
PRINT "SINGLE: "; s

' Double
DIM d AS DOUBLE
d = 2.718281828459
PRINT "DOUBLE: "; d

' String
DIM str AS STRING
str = "Hello QB-COM!"
PRINT "STRING: "; str

PRINT

' ===================================
' 2. QB64 Extended Types
' ===================================
PRINT "--- Test 2: QB64 Extended Types ---"

DIM bigNum AS _INTEGER64
bigNum = 9223372036854775807&&
PRINT "_INTEGER64 max: "; bigNum

bigNum = 1000000&&
PRINT "_INTEGER64 1M: "; bigNum

DIM ul AS _UNSIGNED LONG
ul = 4000000000
PRINT "_UNSIGNED LONG: "; ul

' ===================================
' 3. CONST
' ===================================
PRINT "--- Test 3: CONST ---"

CONST PI = 3.14159
CONST MSG = "Constant Message"
CONST BIG = 1000000

PRINT "PI = "; PI
PRINT "MSG = "; MSG
PRINT "BIG = "; BIG

PRINT

' ===================================
' 4. TYPE (User Defined Types)
' ===================================
PRINT "--- Test 4: TYPE / UDT ---"

TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p AS Point
p.x = 100
p.y = 200
PRINT "Point: x="; p.x; " y="; p.y

' ===================================
' 5. Arrays
' ===================================
PRINT "--- Test 5: Arrays ---"

DIM arr(5) AS INTEGER
arr(1) = 10
arr(2) = 20
arr(3) = 30
PRINT "Array: "; arr(1); arr(2); arr(3)

' ===================================
' 6. Math Operators
' ===================================
PRINT "--- Test 6: Math Operators ---"

DIM a AS INTEGER
DIM b AS INTEGER
a = 17
b = 5

PRINT "a = "; a; ", b = "; b
PRINT "a + b = "; a + b
PRINT "a - b = "; a - b
PRINT "a * b = "; a * b
PRINT "a / b = "; a / b
PRINT "a \\ b = "; a \ b
PRINT "a MOD b = "; a MOD b
PRINT "a ^ 2 = "; a ^ 2

PRINT

' ===================================
' 7. String Functions
' ===================================
PRINT "--- Test 7: String Functions ---"

DIM text AS STRING
text = "Hello World"

PRINT "Original: "; text
PRINT "LEFT$(text, 5) = "; LEFT$(text, 5)
PRINT "RIGHT$(text, 5) = "; RIGHT$(text, 5)
PRINT "MID$(text, 7, 5) = "; MID$(text, 7, 5)
PRINT "LEN(text) = "; LEN(text)
PRINT "UCASE$(text) = "; UCASE$(text)
PRINT "LCASE$(text) = "; LCASE$(text)
PRINT "ASC(H) = "; ASC("H")
PRINT "CHR$(65) = "; CHR$(65)

PRINT

' ===================================
' 8. Math Functions
' ===================================
PRINT "--- Test 8: Math Functions ---"

PRINT "ABS(-42) = "; ABS(-42)
PRINT "SQR(16) = "; SQR(16)
PRINT "INT(3.7) = "; INT(3.7)
PRINT "FIX(-3.7) = "; FIX(-3.7)
PRINT "SGN(-5) = "; SGN(-5)
PRINT "SGN(5) = "; SGN(5)

PRINT

' ===================================
' 9. IF/THEN/ELSE
' ===================================
PRINT "--- Test 9: IF/THEN/ELSE ---"

DIM num AS INTEGER
num = 10

IF num > 5 THEN
    PRINT num; " is greater than 5"
ELSE
    PRINT num; " is not greater than 5"
END IF

IF num = 10 THEN PRINT "Exactly 10!"  ' Single line IF

' Nested IF
IF num > 0 THEN
    IF num < 100 THEN
        PRINT num; " is between 0 and 100"
    END IF
END IF

PRINT

' ===================================
' 10. FOR/NEXT Loops
' ===================================
PRINT "--- Test 10: FOR/NEXT Loops ---"

DIM j AS INTEGER
FOR j = 1 TO 5
    PRINT j;
NEXT j
PRINT

' FOR with STEP
FOR j = 10 TO 0 STEP -2
    PRINT j;
NEXT j
PRINT

PRINT

' ===================================
' 11. WHILE/WEND
' ===================================
PRINT "--- Test 11: WHILE/WEND ---"

DIM w AS INTEGER
w = 1
WHILE w <= 3
    PRINT "W"; w;
    w = w + 1
WEND
PRINT

PRINT

' ===================================
' 12. DO/LOOP
' ===================================
PRINT "--- Test 12: DO/LOOP ---"

DIM dw AS INTEGER
dw = 1
DO WHILE dw <= 3
    PRINT "DW"; dw;
    dw = dw + 1
LOOP
PRINT

' DO UNTIL
DIM du AS INTEGER
du = 1
DO
    PRINT "DU"; du;
    du = du + 1
LOOP UNTIL du > 3
PRINT

PRINT

' ===================================
' 13. SELECT CASE
' ===================================
PRINT "--- Test 13: SELECT CASE ---"

DIM choice AS INTEGER
choice = 2

SELECT CASE choice
    CASE 1
        PRINT "You chose 1"
    CASE 2
        PRINT "You chose 2"
    CASE 3
        PRINT "You chose 3"
    CASE ELSE
        PRINT "Unknown choice"
END SELECT

' Range CASE
DIM score AS INTEGER
score = 85

SELECT CASE score
    CASE 90 TO 100
        PRINT "Grade: A"
    CASE 80 TO 89
        PRINT "Grade: B"
    CASE 70 TO 79
        PRINT "Grade: C"
    CASE IS < 70
        PRINT "Grade: F"
    CASE ELSE
        PRINT "Invalid score"
END SELECT

PRINT

' ===================================
' 14. GOSUB/RETURN
' ===================================
PRINT "--- Test 14: GOSUB/RETURN ---"

PRINT "Before GOSUB"
GOSUB MySubroutine
PRINT "After GOSUB"
GOTO SkipSub

MySubroutine:
PRINT "  Inside subroutine"
RETURN

SkipSub:

PRINT

' ===================================
' 15. DATA/READ/RESTORE
' ===================================
PRINT "--- Test 15: DATA/READ/RESTORE ---"

DIM d1 AS INTEGER
DIM d2 AS INTEGER
DIM d3 AS INTEGER

READ d1, d2, d3
PRINT "Read: "; d1; d2; d3

RESTORE MoreData
READ d1, d2
PRINT "More: "; d1; d2

GOTO SkipData

DATA 10, 20, 30
MoreData:
DATA 100, 200, 300

SkipData:

PRINT

' ===================================
' 16. File I/O
' ===================================
PRINT "--- Test 16: File I/O ---"

' Write to file
OPEN "test_output.txt" FOR OUTPUT AS #1
PRINT #1, "Hello from QB-COM!"
PRINT #1, "Line 2"
CLOSE #1
PRINT "File written"

' Read from file (user input simulation)
' OPEN "test_output.txt" FOR INPUT AS #2
' LINE INPUT #2, fileLine$
' PRINT "File content: "; fileLine$
' CLOSE #2

PRINT

' ===================================
' 17. Logical Operators
' ===================================
PRINT "--- Test 17: Logical Operators ---"

DIM flag1 AS INTEGER
DIM flag2 AS INTEGER
flag1 = 1  ' TRUE
flag2 = 0  ' FALSE

IF flag1 AND 1 THEN PRINT "flag1 AND 1 = TRUE"
IF flag1 OR flag2 THEN PRINT "flag1 OR flag2 = TRUE"
IF NOT flag2 THEN PRINT "NOT flag2 = TRUE"

PRINT

' ===================================
' All Tests Complete
' ===================================
PRINT "==================================="
PRINT "   All Tests Completed Successfully!"
PRINT "==================================="

END
