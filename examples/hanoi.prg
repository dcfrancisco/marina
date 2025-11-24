// Tower of Hanoi (Marina Edition)
// Demonstrates recursion, TUI rendering, colors, and console animation
// Supports 1–13 disks, optimized for 120x40 terminals

import "console"
import "tui"
import "math"
import "str"

ClearScreen()
console.hideCursor()

// Title
tui.box(0, 40, 3, 80, "TOWER OF HANOI PUZZLE")
console.setPos(4, 30)
console.print("How many disks? (1–13): ")

// Input
local diskCount := tui.readInt(1, 13)

// State
local pegs := [
    [],  // Peg A
    [],  // Peg B
    []   // Peg C
]

for i := diskCount downto 1
    pegs[1]:append(i)
end

local moveCount := 0

local baseRow := 35
local positions := [25, 55, 85]

// --- Render Towers ---
function drawTowers()
    // Clear area
    for r := 21 to 36
        console.setPos(r, 0)
        console.print(str.repeat(" ", 120))
    end

    // Poles
    for col in positions
        for r := baseRow - 14 to baseRow - 1
            console.setPos(r, col)
            console.print("│")
        end
    end

    // Bases
    for col in positions
        console.setPos(baseRow, col - 7)
        console.print("═══════════════")
    end

    // Labels
    console.setPos(baseRow + 1, positions[1]); console.print("A")
    console.setPos(baseRow + 1, positions[2]); console.print("B")
    console.setPos(baseRow + 1, positions[3]); console.print("C")

    // Move counter
    console.setPos(10, 30)
    console.print("Moves: " + moveCount)

    drawPeg(pegs[1], positions[1])
    drawPeg(pegs[2], positions[2])
    drawPeg(pegs[3], positions[3])
end

// --- Draw Peg ---
function drawPeg(list, column)
    local height := len(list)
    for i := 1 to height
        local disk      := list[i]
        local width     := max(disk*2 - 1, 3)
        local left      := column - int(width/2)

        setDiskColor(disk)
        console.setPos(baseRow - i, left)
        console.print(str.repeat("█", width))
        console.resetColor()
    end
end

function setDiskColor(n)
    local palette := [12, 14, 10, 11, 9, 13]
    local idx := ((n - 1) % 6) + 1
    console.setColor(palette[idx])
end

// --- Move Disk ---
function moveDisk(fromPeg, toPeg)
    local disk := pegs[fromPeg]:pop()
    pegs[toPeg]:append(disk)

    moveCount := moveCount + 1
    drawTowers()
    sleep(50)
end

// --- Recursion ---
function solve(n, fromPeg, toPeg, auxPeg)
    if n == 1
        moveDisk(fromPeg, toPeg)
    else
        solve(n - 1, fromPeg, auxPeg, toPeg)
        moveDisk(fromPeg, toPeg)
        solve(n - 1, auxPeg, toPeg, fromPeg)
    end
end

// Draw initial
drawTowers()
sleep(300)

// Solve
solve(diskCount, 1, 3, 2)

// Finish
console.setPos(37, 30)
console.print("Puzzle solved!")
console.setPos(38, 30)
console.print("Total moves: " + moveCount)
console.showCursor()
return nil