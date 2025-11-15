// Comprehensive Clipper Demo
// This program demonstrates most working features

? "================================="
? "  Clipper Compiler & VM Demo"
? "================================="
? ""

// Variables and arithmetic
? "1. Variables and Arithmetic"
LOCAL x, y, result
x := 15
y := 7
result := x + y
? "  ", x, "+", y, "=", result
? "  ", x, "-", y, "=", x - y
? "  ", x, "*", y, "=", x * y
? "  ", x, "/", y, "=", x / y
? ""

// Strings
? "2. String Operations"
LOCAL greeting, name, message
greeting := "Hello"
name := "Clipper"
message := greeting + ", " + name + "!"
? "  ", message
? ""

// Arrays
? "3. Arrays"
LOCAL numbers, total, i
numbers := {10, 20, 30, 40, 50}
? "  Array:", numbers
total := 0
FOR i := 0 TO 4
    total := total + numbers[i]
NEXT
? "  Sum of elements:", total
? ""

// Conditionals
? "4. Conditional Logic"
LOCAL score
score := 92
? "  Score:", score
IF score >= 90
    ? "  Grade: A (Excellent!)"
ELSE
    IF score >= 80
        ? "  Grade: B (Good)"
    ELSE
        ? "  Grade: C or below"
    ENDIF
ENDIF
? ""

// FOR loop
? "5. FOR Loop (counting)"
? "  Counting 1 to 5:"
FOR i := 1 TO 5
    ? "    ", i
NEXT
? ""

// WHILE loop
? "6. WHILE Loop (countdown)"
LOCAL count
count := 5
? "  Countdown from 5:"
WHILE count > 0
    ? "    ", count
    count := count - 1
ENDDO
? "  Blastoff!"
? ""

// DO WHILE loop
? "7. DO WHILE Loop"
LOCAL n
n := 1
? "  Powers of 2:"
DO
? "    2^", n, "=", 2 ^ n
n := n + 1
WHILE n <= 5
? ""

// Logical operators
? "8. Logical Operators"
LOCAL a, b
a := TRUE
b := FALSE
? "  a = TRUE, b = FALSE"
? "  a AND b =", a AND b
? "  a OR b =", a OR b
? "  NOT b =", NOT b
? ""

// Comparisons
? "9. Comparisons"
LOCAL num1, num2
num1 := 10
num2 := 20
? "  ", num1, "==", num2, ":", num1 = num2
? "  ", num1, "!=", num2, ":", num1 != num2
? "  ", num1, "<", num2, ":", num1 < num2
? "  ", num1, ">", num2, ":", num1 > num2
? ""

// Complex expression
? "10. Complex Expression"
LOCAL formula
formula := (5 + 3) * 2 - 10 / 2
? "  (5 + 3) * 2 - 10 / 2 =", formula
? ""

? "================================="
? "  Demo Complete!"
? "================================="
