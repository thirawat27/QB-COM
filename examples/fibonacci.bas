' FIBONACCI.BAS
' Calculate Fibonacci sequence

DIM n, i AS INTEGER
DIM a, b, c AS LONG

INPUT "How many Fibonacci numbers to generate? ", n

IF n < 1 THEN
    PRINT "Please enter a positive number."
    END
END IF

PRINT "Fibonacci Sequence:"
PRINT "=================="

a = 0
b = 1

FOR i = 1 TO n
    PRINT "F("; i; ") ="; a
    c = a + b
    a = b
    b = c
NEXT i

PRINT
PRINT "Done!"

END
