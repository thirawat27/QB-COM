' PRIMES.BAS
' Find prime numbers using Sieve of Eratosthenes

DIM limit, i, j AS INTEGER

INPUT "Find primes up to: ", limit

IF limit < 2 THEN
    PRINT "Please enter a number >= 2"
    END
END IF

' Simple prime check
PRINT "Prime numbers up to"; limit; ":"
PRINT

FOR i = 2 TO limit
    DIM isPrime AS INTEGER
    isPrime = 1
    
    FOR j = 2 TO SQR(i)
        IF i MOD j = 0 THEN
            isPrime = 0
            EXIT FOR
        END IF
    NEXT j
    
    IF isPrime = 1 THEN
        PRINT i;
    END IF
NEXT i

PRINT
PRINT
PRINT "Done!"

END
