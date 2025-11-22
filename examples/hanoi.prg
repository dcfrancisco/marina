// Tower of Hanoi - Enhanced ASCII Animation
// Demonstrates recursion and console positioning
// Enhanced version with colors and support for up to 13 disks
// Optimized for modern terminal sizes (120x40+)

ClearScreen()
SetCursor(false)  // Hide cursor for cleaner animation

// Title screen
SetPos(0, 45)
OutStd("╔════════════════════════════════╗")
SetPos(1, 45)
OutStd("║     TOWER OF HANOI PUZZLE      ║")
SetPos(2, 45)
OutStd("╚════════════════════════════════╝")

// Get number of disks from user
SetPos(4, 30)
OutStd("How many disks? (1-13): ")
local numInput := Space(3)
local diskCount := 0

// Validate input with retry loop
local validInput := .F.
while !validInput
    SetPos(4, 54)
    OutStd("   ")  // Clear previous input
    SetPos(4, 54)
    numInput := GetInput(numInput)
    diskCount := Val(Trim(numInput))
    
    // Check if input is valid
    if diskCount >= 1 && diskCount <= 13
        validInput := .T.
    else
        SetPos(5, 30)
        SetColor(12)
        OutStd("Please enter a number between 1 and 13")
        SetColor(7)
        Sleep(1000)
        SetPos(5, 30)
        OutStd(Replicate(" ", 40))
    endif
enddo

// Display confirmation
SetPos(6, 30)
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
Sleep(500)

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
SetPos(37, 30)
OutStd("Puzzle solved!")
SetPos(38, 30)
OutStd("Done in ")
OutStd(moveCount)
OutStd(" moves (minimum: ")
OutStd(minMoves)
OutStd(")")
SetPos(39, 0)
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
    Sleep(100)
return nil

// Draw all three towers with disks
function DrawTowers()
    local baseRow := 35       // Modern terminals have 40+ rows
    local pegA := 25
    local pegB := 55
    local pegC := 85
    local poleHeight := 14    // Height based on max 13 disks
    
    // Clear the tower area (wider for modern screens)
    local clearRow := 21
    while clearRow <= 36
        SetPos(clearRow, 0)
        OutStd(Replicate(" ", 120))
        clearRow := clearRow + 1
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
    SetPos(10, 30)
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
    DrawPeg(peg1, pegA, baseRow)
    
    // Draw disks on peg B
    DrawPeg(peg2, pegB, baseRow)
    
    // Draw disks on peg C
    DrawPeg(peg3, pegC, baseRow)
    
return nil

// Draw disks on a specific peg
function DrawPeg(pegArray, column, baseRow)
    local arrayLen := 0
    if column == 25
        arrayLen := len1
    else
        if column == 55
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
