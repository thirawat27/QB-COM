' QB64 Simple Test

$CONSOLE

' Test _INTEGER64
DIM bigNum AS _INTEGER64
bigNum = 1000000
PRINT "_INTEGER64: "; bigNum

' Test CONST
CONST PI = 3.14159
PRINT "PI = "; PI

' Test TYPE
TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p AS Point
p.x = 10
p.y = 20
PRINT "Point: "; p.x; p.y

' Test SELECT CASE
DIM n AS INTEGER
n = 2
SELECT CASE n
    CASE 1
        PRINT "One"
    CASE 2
        PRINT "Two"
    CASE ELSE
        PRINT "Other"
END SELECT

PRINT "QB64 OK!"
END
