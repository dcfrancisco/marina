// Comprehensive Clipper Demo
// This program demonstrates most working features

ClearScreen()

? "================================="
? "  Clipper Compiler & VM Demo"
? "================================="
? ""

// Variables and arithmetic
? "1. Variables and Arithmetic"
local x, y, result
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
local greeting, name, message
greeting := "Hello"
name := "Clipper"
message := greeting + ", " + name + "!"
? "  ", message
? ""

// Arrays
? "3. Arrays"
local numbers, total, i
numbers := {10, 20, 30, 40, 50}
? "  Array:", numbers
total := 0
for i := 0 to 4
    total := total + numbers[i]
next
? "  Sum of elements:", total
? ""

// Conditionals
? "4. Conditional Logic"
local score
score := 92
? "  Score:", score
if score >= 90
    ? "  Grade: A (Excellent!)"
else
    if score >= 80
        ? "  Grade: B (Good)"
    else
        ? "  Grade: C or below"
    endif
endif
? ""

// FOR loop
? "5. FOR Loop (counting)"
? "  Counting 1 to 5:"
for i := 1 to 5
    ? "    ", i
next
? ""

// WHILE loop
? "6. WHILE Loop (countdown)"
local count
count := 5
? "  Countdown from 5:"
while count > 0
    ? "    ", count
    count := count - 1
enddo
? "  Blastoff!"
? ""

// DO WHILE loop
? "7. DO WHILE Loop"
local n
n := 1
? "  Powers of 2:"
do
? "    2^", n, "=", 2 ^ n
n := n + 1
while n <= 5
? ""

// Logical operators
? "8. Logical Operators"
local a, b
a := TRUE
b := FALSE
? "  a = TRUE, b = FALSE"
? "  a AND b =", a AND b
? "  a OR b =", a OR b
? "  NOT b =", NOT b
? ""

// Comparisons
? "9. Comparisons"
local num1, num2
num1 := 10
num2 := 20
? "  ", num1, "==", num2, ":", num1 = num2
? "  ", num1, "!=", num2, ":", num1 != num2
? "  ", num1, "<", num2, ":", num1 < num2
? "  ", num1, ">", num2, ":", num1 > num2
? ""

// Complex expression
? "10. Complex Expression"
local formula
formula := (5 + 3) * 2 - 10 / 2
? "  (5 + 3) * 2 - 10 / 2 =", formula
? ""

? "================================="
? "  Demo Complete!"
? "================================="
