// Factorial function example
ClearScreen()

// Main program
local num, fact

num := 5
fact := Factorial(num)

? "Factorial of", num, "is", fact

function Factorial(n)
    local result
    
    if n <= 1
        result := 1
    else
        result := n * Factorial(n - 1)
    endif
    
return result
