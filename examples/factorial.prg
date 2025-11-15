// Factorial function example
FUNCTION Factorial(n)
    LOCAL result
    
    IF n <= 1
        result := 1
    ELSE
        result := n * Factorial(n - 1)
    ENDIF
    
    RETURN result

    // Main program
    LOCAL num, fact

    num := 5
    fact := Factorial(num)

    ? "Factorial of", num, "is", fact
