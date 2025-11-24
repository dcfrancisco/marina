// Tower of Hanoi (Marina Edition)
// Demonstrates recursion, TUI rendering, colors, and console animation
// Functional approach with parameter passing

import "console"
import "tui"
import "math"
import "str"

function main()
    ClearScreen()
    
    // Title
    tui.box(0, 40, 3, 80, "TOWER OF HANOI PUZZLE")
    console.setPos(4, 30)
    console.print("How many disks? (1–13): ")
    
    // Input
    local diskCount := tui.readInt(1, 13)
    
    // Hide cursor for animation
    console.hideCursor()
    
    // Initialize state
    local pegs := [[], [], []]
    for i := diskCount downto 1
        pegs[1]:append(i)
    end
    
    local moveCount := 0
    local baseRow := 35
    local positions := [25, 55, 85]
    
    // Draw initial
    moveCount := drawTowers(pegs, positions, baseRow, moveCount)
    sleep(300)
    
    // Solve
    moveCount := solve(diskCount, 1, 3, 2, pegs, positions, baseRow, moveCount)
    
    // Finish
    console.setPos(37, 30)
    console.print("Puzzle solved!")
    console.setPos(38, 30)
    console.print("Total moves: " + moveCount)
    console.showCursor()
    
    return nil
end

// --- Render Towers ---
function drawTowers(pegs, positions, baseRow, moveCount)
    // Clear area
    for r := 21 to 36
        console.setPos(r, 0)
        console.print(str.repeat(" ", 120))
    end

    // Poles
    for pegIdx := 1 to 3
        local col := positions[pegIdx]
        for r := baseRow - 14 to baseRow - 1
            console.setPos(r, col)
            console.print("│")
        end
    end

    // Bases
    for pegIdx := 1 to 3
        local col := positions[pegIdx]
        console.setPos(baseRow, col - 7)
        console.print("═══════════════")
    end

    // Labels
    console.setPos(baseRow + 1, positions[1])
    console.print("A")
    console.setPos(baseRow + 1, positions[2])
    console.print("B")
    console.setPos(baseRow + 1, positions[3])
    console.print("C")

    // Move counter
    console.setPos(10, 30)
    console.print("Moves: " + moveCount)

    drawPeg(pegs[1], positions[1], baseRow)
    drawPeg(pegs[2], positions[2], baseRow)
    drawPeg(pegs[3], positions[3], baseRow)
    
    return moveCount
end

// --- Draw Peg ---
function drawPeg(list, column, baseRow)
    local height := len(list)
    for i := 1 to height
        local disk := list[i]
        local width := max(disk*2 - 1, 3)
        local left := column - int(width/2)

        setDiskColor(disk)
        console.setPos(baseRow - i, left)
        console.print(str.repeat("█", width))
        console.resetColor()
    end
    return nil
end

function setDiskColor(n)
    local palette := [12, 14, 10, 11, 9, 13]
    local idx := ((n - 1) % 6) + 1
    console.setColor(palette[idx])
    return nil
end

// --- Move Disk ---
function moveDisk(fromPeg, toPeg, pegs, positions, baseRow, moveCount)
    local disk := pegs[fromPeg]:pop()
    pegs[toPeg]:append(disk)

    moveCount := moveCount + 1
    moveCount := drawTowers(pegs, positions, baseRow, moveCount)
    sleep(50)
    
    return moveCount
end

// --- Recursion ---
function solve(n, fromPeg, toPeg, auxPeg, pegs, positions, baseRow, moveCount)
    if n == 1
        moveCount := moveDisk(fromPeg, toPeg, pegs, positions, baseRow, moveCount)
    else
        moveCount := solve(n - 1, fromPeg, auxPeg, toPeg, pegs, positions, baseRow, moveCount)
        moveCount := moveDisk(fromPeg, toPeg, pegs, positions, baseRow, moveCount)
        moveCount := solve(n - 1, auxPeg, toPeg, fromPeg, pegs, positions, baseRow, moveCount)
    end
    return moveCount
end

// Run the main function
main()
