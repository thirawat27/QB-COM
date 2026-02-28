' GUESS.BAS
' A number guessing game

RANDOMIZE TIMER

DIM secret, guess, tries AS INTEGER
secret = INT(RND * 100) + 1
tries = 0

PRINT "Number Guessing Game"
PRINT "===================="
PRINT "I'm thinking of a number between 1 and 100."
PRINT

DO
    INPUT "Enter your guess: ", guess
    tries = tries + 1
    
    IF guess < secret THEN
        PRINT "Too low!"
    ELSEIF guess > secret THEN
        PRINT "Too high!"
    ELSE
        PRINT "Congratulations! You guessed it in"; tries; "tries!"
        EXIT DO
    END IF
LOOP

END
