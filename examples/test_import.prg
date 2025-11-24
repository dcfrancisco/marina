// Test import system
import "console"
import "math"
import "str"

console.setColor(12)
console.print("Testing Marina Import System!")
console.resetColor()
console.print("")

// Test math module
local x := 5
local y := 3
local maxVal := math.max(x, y)
local minVal := math.min(x, y)

console.print("Math tests:")
console.print("max(5, 3) = ")
console.print(maxVal)
console.print("")
console.print("min(5, 3) = ")
console.print(minVal)
console.print("")

// Test string module
local repeated := str.repeat("Hello ", 3)
console.print("String tests:")
console.print(repeated)
console.print("")

local text := "Hello World"
local length := str.len(text)
console.print("Length of 'Hello World': ")
console.print(length)
console.print("")

// Test console positioning
console.setPos(15, 10)
console.setColor(14)
console.print("Positioned text at row 15, col 10")
console.resetColor()

console.setPos(20, 0)
console.print("Done!")

return nil
