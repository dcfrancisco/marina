// Val() Function Demo
// Demonstrates string to number conversion

ClearScreen()

SetPos(1, 20)
OutStd("╔═══════════════════════════════╗")
SetPos(2, 20)
OutStd("║   VAL() FUNCTION DEMO         ║")
SetPos(3, 20)
OutStd("╚═══════════════════════════════╝")

local row := 5

// Example 1: Basic conversion
SetPos(row, 5)
OutStd("Val('123'): ")
? Val("123")
row := row + 1

// Example 2: Decimal numbers
SetPos(row, 5)
OutStd("Val('12.5'): ")
? Val("12.5")
row := row + 1

// Example 3: Invalid string
SetPos(row, 5)
OutStd("Val('abc'): ")
? Val("abc")
row := row + 1

// Example 4: Mixed content
SetPos(row, 5)
OutStd("Val('42abc'): ")
? Val("42abc")
row := row + 2

// Example 5: Math with Val
SetPos(row, 5)
OutStd("Val('10') + Val('20') = ")
? Val("10") + Val("20")
row := row + 2

// Example 6: User input conversion
SetPos(row, 5)
OutStd("Enter a number: ")
local input := Space(10)
input := GetInput(input)
local num := Val(Trim(input))

SetPos(row + 2, 5)
OutStd("You entered: ")
? num
SetPos(row + 3, 5)
OutStd("Doubled: ")
? num * 2

SetPos(row + 5, 0)
OutStd("")
?
