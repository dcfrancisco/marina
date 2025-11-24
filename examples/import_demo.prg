// Simple Tower of Hanoi demo using imports
import "console"
import "math"
import "str"

// Clear screen
ClearScreen()

// Title
console.setPos(1, 25)
console.setColor(14)
console.print("MARINA IMPORT SYSTEM DEMO")
console.resetColor()

// Draw a simple box
console.setPos(5, 10)
console.print("+")
console.print(str.repeat("-", 30))
console.print("+")

console.setPos(6, 10)
console.print("|")
console.setPos(6, 41)
console.print("|")

console.setPos(7, 10)
console.print("+")
console.print(str.repeat("-", 30))
console.print("+")

// Content
console.setPos(6, 12)
console.setColor(12)
console.print("Console Module: ")
console.setColor(10)
console.print("Working!")
console.resetColor()

// Test math functions
console.setPos(10, 10)
console.print("Math Module Tests:")

local a := 7
local b := 3
console.setPos(11, 12)
console.print("max(7, 3) = ")
console.print(math.max(a, b))

console.setPos(12, 12)
console.print("min(7, 3) = ")
console.print(math.min(a, b))

console.setPos(13, 12)
console.print("int(7.8) = ")
console.print(math.int(7.8))

// Test string functions
console.setPos(15, 10)
console.print("String Module Tests:")

console.setPos(16, 12)
console.print("repeat('*', 5) = ")
console.print(str.repeat("*", 5))

console.setPos(17, 12)
console.print("len('Hello') = ")
console.print(str.len("Hello"))

// Colorful finish
console.setPos(20, 10)
for i := 9 to 15
    console.setColor(i)
    console.print("█")
next

console.resetColor()
console.setPos(22, 0)
console.print("All modules working correctly!")

return nil
