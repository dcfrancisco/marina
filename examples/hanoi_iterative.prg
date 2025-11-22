// Tower of Hanoi - Iterative Solution
// Non-recursive approach using pattern-based moves
// More efficient - no recursion stack overhead
// Same visual output as recursive version

ClearScreen()
SetCursor(.F.)

// Title screen
SetPos(0, 45)
OutStd("╔════════════════════════════════╗")
SetPos(1, 45)
OutStd("║  TOWER OF HANOI - ITERATIVE    ║")
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
    OutStd("   ")
    SetPos(4, 54)
    numInput := GetInput(numInput)
    diskCount := Val(Trim(numInput))
    
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

SetPos(6, 30)
OutStd("Solving Tower of Hanoi with ")
OutStd(diskCount) 
OutStd(" disks...")
SetPos(7, 30)
OutStd("(Using iterative algorithm)")

// Global variables for disk tracking
peg1 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
peg2 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
peg3 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

len1 := diskCount
len2 := 0
len3 := 0

// Initialize disks on peg1
local i := 0
while i < diskCount
    peg1[i] := diskCount - i
    i := i + 1
enddo

moveCount := 0

// Display initial state
DrawTowers()
Sleep(500)

// Calculate total moves needed (2^n - 1)
local totalMoves := 1
local j := 0
while j < diskCount
    totalMoves := totalMoves * 2
    j := j + 1
enddo
totalMoves := totalMoves - 1

// Iterative solution
// Algorithm: Alternate between moving smallest disk and making the only other legal move
// For odd disk count: smallest cycles 1→3→2→1 to reach peg 3
// For even disk count: smallest cycles 1→2→3→1 to reach peg 3
local isOdd := (diskCount % 2) == 1
local move := 0
local smallDiskPeg := 1  // Track where smallest disk is

while move < totalMoves
    move := move + 1
    
    if (move % 2) == 1
        // ODD moves: move smallest disk in its cycle
        local nextPeg := 0
        if isOdd
            // Odd disk count: 1→3→2→1
            if smallDiskPeg == 1
                nextPeg := 3
            else
                if smallDiskPeg == 3
                    nextPeg := 2
                else
                    nextPeg := 1
                endif
            endif
        else
            // Even disk count: 1→2→3→1
            if smallDiskPeg == 1
                nextPeg := 2
            else
                if smallDiskPeg == 2
                    nextPeg := 3
                else
                    nextPeg := 1
                endif
            endif
        endif
        MoveDiskBetween(smallDiskPeg, nextPeg)
        smallDiskPeg := nextPeg
    else
        // EVEN moves: make the only legal move not involving smallest disk
        MoveLargerDisk()
    endif
enddo

// Final message
SetPos(37, 30)
OutStd("Puzzle solved!")
SetPos(38, 30)
OutStd("Done in ")
OutStd(moveCount)
OutStd(" moves (minimum: ")
OutStd(totalMoves)
OutStd(")")
SetPos(39, 0)
OutStd("")

SetCursor(.T.)

// Get peg length
function GetPegLength(pegNum)
    if pegNum == 1
        return len1
    else
        if pegNum == 2
            return len2
        else
            return len3
        endif
    endif
return 0

// Move a larger disk (the only legal move not involving disk 1)
function MoveLargerDisk()
    // Try all possible moves and make the legal one
    if CanMove(1, 2)
        MoveDiskBetween(1, 2)
    else
        if CanMove(2, 1)
            MoveDiskBetween(2, 1)
        else
            if CanMove(1, 3)
                MoveDiskBetween(1, 3)
            else
                if CanMove(3, 1)
                    MoveDiskBetween(3, 1)
                else
                    if CanMove(2, 3)
                        MoveDiskBetween(2, 3)
                    else
                        MoveDiskBetween(3, 2)
                    endif
                endif
            endif
        endif
    endif
return nil

// Check if a move is legal
function CanMove(fromPeg, toPeg)
    local fromLen := GetPegLength(fromPeg)
    local toLen := GetPegLength(toPeg)
    
    // Can't move from empty peg
    if fromLen == 0
        return .F.
    endif
    
    // Can't move disk 1 (smallest) - handled separately
    // Check global arrays directly
    local fromDisk := 0
    if fromPeg == 1
        fromDisk := peg1[fromLen - 1]
    else
        if fromPeg == 2
            fromDisk := peg2[fromLen - 1]
        else
            fromDisk := peg3[fromLen - 1]
        endif
    endif
    
    if fromDisk == 1
        return .F.
    endif
    
    // Can always move to empty peg
    if toLen == 0
        return .T.
    endif
    
    // Can only move smaller disk onto larger disk
    local toDisk := 0
    if toPeg == 1
        toDisk := peg1[toLen - 1]
    else
        if toPeg == 2
            toDisk := peg2[toLen - 1]
        else
            toDisk := peg3[toLen - 1]
        endif
    endif
    
    return fromDisk < toDisk

// Move disk between pegs
function MoveDiskBetween(fromPeg, toPeg)
    local disk := 0
    
    // Get disk from source
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
    
    // Place on destination
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
    local baseRow := 35
    local pegA := 25
    local pegB := 55
    local pegC := 85
    local poleHeight := 14
    
    // Clear the tower area
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
    
    // Draw disks on all pegs
    DrawPeg(peg1, pegA, baseRow, len1)
    DrawPeg(peg2, pegB, baseRow, len2)
    DrawPeg(peg3, pegC, baseRow, len3)
    
return nil

// Draw disks on a specific peg
function DrawPeg(pegArray, column, baseRow, arrayLen)
    if arrayLen == 0
        return nil
    endif
    
    local diskNum := 0
    while diskNum < arrayLen
        local diskSize := pegArray[diskNum]
        if diskSize > 0
            local diskWidth := Max(diskSize * 2 - 1, 3)
            local leftPos := column - Int(diskWidth / 2)
            
            SetDiskColor(diskSize % 6 + 1)
            SetPos(baseRow - diskNum - 1, leftPos)
            OutStd(Replicate("█", diskWidth))
            SetColor(7)
        endif
        diskNum := diskNum + 1
    enddo
return nil

// Set console color for disks
function SetDiskColor(colorNum)
    local colors := {12, 14, 10, 11, 9, 13}
    local idx := ((colorNum - 1) % 6)
    SetColor(colors[idx])
return nil
