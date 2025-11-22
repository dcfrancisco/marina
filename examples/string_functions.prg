// String Functions Demo
// Demonstrates all string manipulation functions

ClearScreen()

// Header
SetPos(0, 25)
OutStd("╔═══════════════════════════════╗")
SetPos(1, 25)
OutStd("║  STRING FUNCTIONS DEMO        ║")
SetPos(2, 25)
OutStd("╚═══════════════════════════════╝")

local row := 4

// Space() function
SetPos(row, 5)
OutStd("Space(10): [")
OutStd(Space(10))
OutStd("]")
row := row + 1

// Replicate() function
SetPos(row, 5)
OutStd("Replicate('*', 8): [")
OutStd(Replicate("*", 8))
OutStd("]")
row := row + 1

// Len() function
local text := "Hello World"
SetPos(row, 5)
OutStd("Len('Hello World'): 11")
row := row + 1

// SubStr() function
SetPos(row, 5)
OutStd("SubStr('Hello World', 7, 5): [")
OutStd(SubStr(text, 7, 5))
OutStd("]")
row := row + 1

// Trim functions
local padded := "  trim me  "
SetPos(row, 5)
OutStd("Original: [")
OutStd(padded)
OutStd("]")
row := row + 1

SetPos(row, 5)
OutStd("Trim(): [")
OutStd(Trim(padded))
OutStd("]")
row := row + 1

SetPos(row, 5)
OutStd("LTrim(): [")
OutStd(LTrim(padded))
OutStd("]")
row := row + 1

SetPos(row, 5)
OutStd("RTrim(): [")
OutStd(RTrim(padded))
OutStd("]")
row := row + 1

SetPos(row, 5)
OutStd("AllTrim(): [")
OutStd(AllTrim(padded))
OutStd("]")
row := row + 2

// Chr() and Asc() functions
SetPos(row, 5)
OutStd("Chr(65): [")
OutStd(Chr(65))
OutStd("]")
row := row + 1

SetPos(row, 5)
OutStd("Asc('A'): 65")
row := row + 1

SetPos(row, 5)
OutStd("Chr(42): [")
OutStd(Chr(42))
OutStd("]")
row := row + 2

// Building strings
local name := "Marina"
local version := "1.0"
local message := name + Space(1) + "v" + version

SetPos(row, 5)
OutStd("Built string: [")
OutStd(message)
OutStd("]")

SetPos(row + 2, 0)
OutStd("")
?
