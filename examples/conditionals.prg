// Conditional logic example
ClearScreen()

LOCAL score, grade

score := 85

IF score >= 90
    grade := "A"
ELSE
    IF score >= 80
        grade := "B"
    ELSE
        IF score >= 70
            grade := "C"
        ELSE
            IF score >= 60
                grade := "D"
            ELSE
                grade := "F"
            ENDIF
        ENDIF
    ENDIF
ENDIF

? "Score:", score
? "Grade:", grade

// Nested conditionals
LOCAL x, y, max

x := 42
y := 37

IF x > y
    max := x
    ? x, "is greater than", y
ELSE
    max := y
    ? y, "is greater than or equal to", x
ENDIF

? "Maximum value:", max
