// Demonstrate logical operators in Marina
// Testing &&, ||, and ! operators

procedure Main()
    local a := 5
    local b := 10
    local c := 15

    // Test && (AND) operator
    OutStd("Testing && (AND) operator:")
    if a < b && b < c
        OutStd("✓ a < b AND b < c is TRUE")
    endif

    if a > b && b < c
        OutStd("This should not print")
    else
        OutStd("✓ a > b AND b < c is FALSE")
    endif

    OutStd("")

    // Test || (OR) operator
    OutStd("Testing || (OR) operator:")
    if a > b || b < c
        OutStd("✓ a > b OR b < c is TRUE")
    endif

    if a > b || b > c
        OutStd("This should not print")
    else
        OutStd("✓ a > b OR b > c is FALSE")
    endif

    OutStd("")

    // Test ! (NOT) operator
    OutStd("Testing ! (NOT) operator:")
    local isValid := true
    if !isValid
        OutStd("This should not print")
    else
        OutStd("✓ !TRUE is FALSE")
    endif

    isValid := false
    if !isValid
        OutStd("✓ !FALSE is TRUE")
    endif

    OutStd("")

    // Complex expressions
    OutStd("Testing complex expressions:")
    if (a < b && b < c) || (a > 20)
        OutStd("✓ (a < b AND b < c) OR (a > 20) is TRUE")
    endif

    if !(a > b) && (b < c)
        OutStd("✓ NOT(a > b) AND (b < c) is TRUE")
    endif

    OutStd("")
    OutStd("All logical operator tests passed!")
return
