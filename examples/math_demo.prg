// Math Functions Demo
// Demonstrates all available math functions in Marina

ClearScreen()
SetPos(0, 30)
OutStd("╔═══════════════════════════════╗")
SetPos(1, 30)
OutStd("║   MARINA MATH FUNCTIONS DEMO  ║")
SetPos(2, 30)
OutStd("╚═══════════════════════════════╝")

local row := 4

// Abs - Absolute value
SetPos(row, 5)
OutStd("Abs(-42) = ")
OutStd(Abs(-42))
row := row + 1

SetPos(row, 5)
OutStd("Abs(42) = ")
OutStd(Abs(42))
row := row + 2

// Sqrt - Square root
SetPos(row, 5)
OutStd("Sqrt(16) = ")
OutStd(Sqrt(16))
row := row + 1

SetPos(row, 5)
OutStd("Sqrt(144) = ")
OutStd(Sqrt(144))
row := row + 2

// Round - Rounding
SetPos(row, 5)
OutStd("Round(3.7) = ")
OutStd(Round(3.7))
row := row + 1

SetPos(row, 5)
OutStd("Round(3.2) = ")
OutStd(Round(3.2))
row := row + 1

SetPos(row, 5)
OutStd("Round(3.14159, 2) = ")
OutStd(Round(3.14159, 2))
row := row + 2

// Int - Integer part
SetPos(row, 5)
OutStd("Int(7.9) = ")
OutStd(Int(7.9))
row := row + 1

SetPos(row, 5)
OutStd("Int(-7.9) = ")
OutStd(Int(-7.9))
row := row + 2

// Min and Max
SetPos(row, 5)
OutStd("Min(5, 3, 9, 1, 7) = ")
OutStd(Min(5, 3, 9, 1, 7))
row := row + 1

SetPos(row, 5)
OutStd("Max(5, 3, 9, 1, 7) = ")
OutStd(Max(5, 3, 9, 1, 7))
row := row + 2

// Trig functions (using common angles)
SetPos(row, 5)
OutStd("Sin(0) = ")
OutStd(Round(Sin(0), 4))
row := row + 1

SetPos(row, 5)
OutStd("Cos(0) = ")
OutStd(Round(Cos(0), 4))
row := row + 1

SetPos(row, 5)
OutStd("Tan(0) = ")
OutStd(Round(Tan(0), 4))
row := row + 2

// Power operator
SetPos(row, 5)
OutStd("2 ^ 8 = ")
OutStd(2 ^ 8)
row := row + 1

SetPos(row, 5)
OutStd("10 ^ 3 = ")
OutStd(10 ^ 3)
row := row + 2

// Modulo operator
SetPos(row, 5)
OutStd("17 % 5 = ")
OutStd(17 % 5)
row := row + 1

SetPos(row, 5)
OutStd("100 % 7 = ")
OutStd(100 % 7)
row := row + 2

SetPos(row, 25)
OutStd("All math functions working! ✓")

SetPos(row + 2, 0)
OutStd("")
