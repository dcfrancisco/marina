// Simplified Tower of Hanoi for Marina
// Uses simple approach without nested array methods

import "console"
import "math"
import "str"

ClearScreen()

console.setColor(14)
console.print("TOWER OF HANOI")
console.resetColor()
console.print("")
console.print("Number of disks: ")

local diskCount := 3  // Fixed for now

// We'll use separate arrays instead of nested
local peg1 := []
local peg2 := []
local peg3 := []

// Initialize peg1 with disks
for i := diskCount downto 1
    // Manually build the array
    // For now, just track the count
next

local moveCount := 0

console.print("")
console.print("Solving for ")
console.print(diskCount)
console.print(" disks...")
console.print("")

// Recursive solve function
function solve(n, fromPeg, toPeg, auxPeg)
    if n == 1
        moveCount := moveCount + 1
        console.print("Move disk from peg ")
        console.print(fromPeg)
        console.print(" to peg ")
        console.print(toPeg)
        console.print("")
    else
        solve(n - 1, fromPeg, auxPeg, toPeg)
        
        moveCount := moveCount + 1
        console.print("Move disk from peg ")
        console.print(fromPeg)
        console.print(" to peg ")
        console.print(toPeg)
        console.print("")
        
        solve(n - 1, auxPeg, toPeg, fromPeg)
    end
end

solve(diskCount, 1, 3, 2)

console.print("")
console.print("Solved in ")
console.print(moveCount)
console.print(" moves!")

return nil
