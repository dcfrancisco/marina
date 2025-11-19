// Conditional logic example
ClearScreen()

local score, grade

score := 85

if score >= 90
    grade := "A"
else
    if score >= 80
        grade := "B"
    else
        if score >= 70
            grade := "C"
        else
            if score >= 60
                grade := "D"
            else
                grade := "F"
            endif
        endif
    endif
endif

? "Score:", score
? "Grade:", grade

// Nested conditionals
local x, y, max

x := 42
y := 37

if x > y
    max := x
    ? x, "is greater than", y
else
    max := y
    ? y, "is greater than or equal to", x
endif

? "Maximum value:", max
