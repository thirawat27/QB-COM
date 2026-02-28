' QB64 Complete Demo

$CONSOLE

' === Advanced Data Types ===
PRINT "=== Advanced Data Types ==="

DIM bigNum AS _INTEGER64
bigNum = 9223372036854775807
PRINT "_INTEGER64 max: "; bigNum

DIM unsigned AS _UNSIGNED LONG
unsigned = 4294967295
PRINT "_UNSIGNED LONG max: "; unsigned

' === Constants ===
PRINT
PRINT "=== Constants ==="

CONST PI = 3.14159
CONST MAX_USERS = 1000

PRINT "PI = "; PI
PRINT "Max Users = "; MAX_USERS

' === User-Defined Types ===
PRINT
PRINT "=== User-Defined Types ==="

TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p1 AS Point
p1.x = 100.5
p1.y = 200.75
PRINT "Point: ("; p1.x; ", "; p1.y; ")"

' === SELECT CASE ===
PRINT
PRINT "=== SELECT CASE ==="

DIM score AS INTEGER
score = 85

SELECT CASE score
    CASE IS >= 90
        PRINT "Grade: A"
    CASE 80 TO 89
        PRINT "Grade: B"
    CASE ELSE
        PRINT "Other"
END SELECT

' === EXIT FOR ===
PRINT
PRINT "=== EXIT FOR ==="
FOR i = 1 TO 100
    IF i > 5 THEN EXIT FOR
    PRINT i;
NEXT i
PRINT

' === Arrays ===
PRINT
PRINT "=== Arrays ==="

DIM arr(10) AS INTEGER
FOR i = 0 TO 10
    arr(i) = i * i
NEXT i
PRINT "Squares: ";
FOR i = 0 TO 10
    PRINT arr(i);
NEXT i
PRINT

' === Random Numbers ===
PRINT
PRINT "=== Random Numbers ==="

RANDOMIZE TIMER
FOR i = 1 TO 3
    PRINT "Random "; i; ": "; RND
NEXT i

' === DATA/READ/RESTORE ===
PRINT
PRINT "=== DATA/READ/RESTORE ==="

DIM d1 AS INTEGER, d2 AS INTEGER
READ d1, d2
PRINT "Data1: "; d1; d2

RESTORE DataLabel2
READ d1, d2
PRINT "Data2: "; d1; d2

' === Math Functions ===
PRINT
PRINT "=== Math Functions ==="
PRINT "ABS(-5) = "; ABS(-5)
PRINT "SQR(16) = "; SQR(16)
PRINT "2^10 = "; 2 ^ 10

' === String Functions ===
PRINT
PRINT "=== String Functions ==="
DIM text AS STRING
text = "Hello World"
PRINT "LEFT$ = "; LEFT$(text, 5)
PRINT "RIGHT$ = "; RIGHT$(text, 5)
PRINT "LEN = "; LEN(text)

PRINT
PRINT "=== QB64 System Ready! ==="
END

DataLabel1:
DATA 10, 20

DataLabel2:
DATA 100, 200
