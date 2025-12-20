// Tower of Hanoi - Enhanced ASCII Animation
// Demonstrates recursion and console positioning
// Enhanced version with colors and support for up to 13 disks
// Optimized for modern terminal sizes (120x40+)

ClearScreen()
SetCursor(false) // Hide cursor for cleaner animation

// Layout configuration
leftMargin := 15
maxWidth := 120 // Maximum screen width for tower display
baseRow := 25 // Base row for towers - adjust this to move entire tower up/down
poleHeight := 14 // Height based on max 13 disks

// Calculate peg positions based on maxWidth
pegA := Int(maxWidth / 4)
pegB := Int(maxWidth / 2)
pegC := Int(maxWidth * 3 / 4)

// Title screen
SetColor(12) // Red color
SetPos(1, leftMargin)
OutStd("TOWERS OF HANOI")
SetColor(7) // Reset to default color

// ...existing code...

// Get number of disks from user
SetPos(4, leftMargin)
OutStd("How many disks? (1-13): ")
local numInput := Space(3)
local diskCount := 0

// Validate input with retry loop
local validInput := .F.
while !validInput
    SetPos(4, leftMargin + 24)
    OutStd("   ") // Clear previous input
    SetPos(4, leftMargin + 24)
    numInput := GetInput(numInput)
    diskCount := Val(Trim(numInput))

    // Check if input is valid
    if diskCount >= 1 && diskCount <= 13
        validInput := .T.
    else
        SetPos(5, leftMargin)
        SetColor(12)
        OutStd("Please enter a number between 1 and 13")
        SetColor(7)
        Sleep(1000)
        SetPos(5, leftMargin)
        OutStd(Replicate(" ", 40))
    endif
enddo

// Display confirmation
SetPos(6, leftMargin)
OutStd("Solving Tower of Hanoi with ")
OutStd(diskCount)
OutStd(" disks...")

// Global variables for disk tracking (no LOCAL so they're accessible in functions)
peg1 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
peg2 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
peg3 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

len1 := diskCount
len2 := 0
len3 := 0

// Initialize requested number of disks on peg1
local i := 0
while i < diskCount
    peg1[i] := diskCount - i
    i := i + 1
enddo

moveCount := 0

// Display initial state
DrawTowers()

// Wait a moment for visual effect
Sleep(50)

// Solve the puzzle
SolveHanoi(diskCount, 1, 3, 2)

// Calculate minimum moves (2^n - 1)
local minMoves := 1
local j := 0
while j < diskCount
    minMoves := minMoves * 2
    j := j + 1
enddo
minMoves := minMoves - 1

// Final message
SetPos(8, leftMargin)
OutStd("Puzzle solved!")
SetPos(9, leftMargin)
OutStd("Done in ")
OutStd(moveCount)
OutStd(" moves (minimum: ")
OutStd(minMoves)
OutStd(")")
SetPos(baseRow + 4, 0)
OutStd("")

// Show cursor again
SetCursor(true)

// Recursive function to solve Tower of Hanoi
function SolveHanoi(n, fromPeg, toPeg, auxPeg)
    if n == 1
        MoveDisk(fromPeg, toPeg)
    else
        SolveHanoi(n - 1, fromPeg, auxPeg, toPeg)
        MoveDisk(fromPeg, toPeg)
        SolveHanoi(n - 1, auxPeg, toPeg, fromPeg)
    endif
return nil

// Move a disk from one peg to another
function MoveDisk(fromPeg, toPeg)
    local disk

    // Get disk from source peg
    if fromPeg == 1
        len1 := len1 - 1
        disk := peg1[len1]
        peg1[len1] := 0
    else
        if fromPeg == 2
            len2 := len2 - 1
            disk := peg2[len2]
            peg2[len2] := 0
        else
            len3 := len3 - 1
            disk := peg3[len3]
            peg3[len3] := 0
        endif
    endif

    // Place disk on destination peg
    if toPeg == 1
        peg1[len1] := disk
        len1 := len1 + 1
    else
        if toPeg == 2
            peg2[len2] := disk
            len2 := len2 + 1
        else
            peg3[len3] := disk
            len3 := len3 + 1
        endif
    endif

    moveCount := moveCount + 1
    DrawTowers()
    // Sleep(100)
return nil

// Draw all three towers with disks
function DrawTowers()
    // Clear only disk rows to minimize flicker
    local diskRow := baseRow - poleHeight
    while diskRow < baseRow
        SetPos(diskRow, 0)
        OutStd(Replicate(" ", maxWidth))
        diskRow := diskRow + 1
    enddo

    // Draw vertical poles
    local poleRow := baseRow - poleHeight
    while poleRow < baseRow
        SetPos(poleRow, pegA)
        OutStd("│")
        SetPos(poleRow, pegB)
        OutStd("│")
        SetPos(poleRow, pegC)
        OutStd("│")
        poleRow := poleRow + 1
    enddo

    // Draw peg labels
    SetPos(baseRow + 1, pegA)
    OutStd("A")
    SetPos(baseRow + 1, pegB)
    OutStd("B")
    SetPos(baseRow + 1, pegC)
    OutStd("C")

    // Draw move counter
    SetPos(7, leftMargin)
    OutStd("Moves: ")
    OutStd(moveCount)

    // Draw base platforms
    SetPos(baseRow, pegA - 7)
    OutStd("═══════════════")
    SetPos(baseRow, pegB - 7)
    OutStd("═══════════════")
    SetPos(baseRow, pegC - 7)
    OutStd("═══════════════")

    // Draw disks on peg A
    DrawPeg(peg1, pegA)
    // Draw disks on peg B
    DrawPeg(peg2, pegB)
    // Draw disks on peg C
    DrawPeg(peg3, pegC)

return nil

// Draw disks on a specific peg
function DrawPeg(pegArray, column)
    local arrayLen := 0
    if column == pegA
        arrayLen := len1
    else
        if column == pegB
            arrayLen := len2
        else
            arrayLen := len3
        endif
    endif

    if arrayLen == 0
        return nil
    endif

    local diskNum := 0
    while diskNum < arrayLen
        local diskSize := pegArray[diskNum]
        if diskSize > 0
            local diskWidth := Max(diskSize * 2 - 1, 3)
            local leftPos := column - Int(diskWidth / 2)

            SetDiskColor(diskSize % 14 + 1)
            SetPos(baseRow - diskNum - 1, leftPos)
            OutStd(Replicate("█", diskWidth))
            SetColor(7)
        endif
        diskNum := diskNum + 1
    enddo
return nil

// Set console color for disks (bright colors for dark backgrounds)
function SetDiskColor(colorNum)
    local colors := {12, 14, 10, 11, 9, 13}
    local idx := ((colorNum - 1) % 6)
    SetColor(colors[idx])
return nil
