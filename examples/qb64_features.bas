' QB64 Features Demo
' ?????????????????? QB64

$CONSOLE

' 1. ?????????? 64-bit
DIM bigNumber AS _INTEGER64
bigNumber = 9223372036854775807
PRINT "_INTEGER64 max value: "; bigNumber

DIM unsignedNum AS _UNSIGNED LONG
unsignedNum = 4294967295
PRINT "_UNSIGNED LONG max: "; unsignedNum

' 2. CONST
CONST PI = 3.14159
CONST MAX_SIZE = 100
PRINT "PI = "; PI

' 3. TYPE...END TYPE
TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p AS Point
p.x = 100
p.y = 200
PRINT "Point: ("; p.x; ", "; p.y; ")"

' 4. SELECT CASE ??????????
DIM choice AS INTEGER
choice = 2
SELECT CASE choice
    CASE 1
        PRINT "Selected: 1"
    CASE 2
        PRINT "Selected: 2"
    CASE 3 TO 5
        PRINT "Selected: 3-5"
    CASE ELSE
        PRINT "Other"
END SELECT

' 5. EXIT FOR
FOR i = 1 TO 100
    IF i > 5 THEN EXIT FOR
    PRINT i;
NEXT i
PRINT

' 6. RANDOMIZE TIMER (QB64 compatible)
RANDOMIZE TIMER
DIM r AS SINGLE
r = RND
PRINT "Random: "; r

PRINT "All QB64 features working!"
END
