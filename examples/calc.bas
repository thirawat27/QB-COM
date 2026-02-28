' CALC.BAS
' A simple calculator program

PRINT "Simple Calculator"
PRINT "================="
PRINT

DIM a, b AS SINGLE
DIM op AS STRING

INPUT "Enter first number: ", a
INPUT "Enter operator (+, -, *, /): ", op
INPUT "Enter second number: ", b

DIM result AS SINGLE

IF op = "+" THEN
    result = a + b
ELSEIF op = "-" THEN
    result = a - b
ELSEIF op = "*" THEN
    result = a * b
ELSEIF op = "/" THEN
    IF b <> 0 THEN
        result = a / b
    ELSE
        PRINT "Error: Division by zero!"
        END
    END IF
ELSE
    PRINT "Error: Unknown operator"
    END
END IF

PRINT "Result: "; result

END
