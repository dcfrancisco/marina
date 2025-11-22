// Tower of Hanoi - Enhanced ASCII Animation
// Demonstrates recursion and console positioning
// Enhanced version with colors and support for up to 13 disks
// Optimized for modern terminal sizes (120x40+)

ClearScreen()

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
numInput := GetInput(numInput)
local diskCount := Val(Trim(numInput))

// Validate input
if diskCount < 1
    diskCount := 1
endif
if diskCount > 13
    diskCount := 13
endif

// Display confirmation
SetPos(6, 30)
OutStd("Solving Tower of Hanoi with ")
? diskCount
OutStd(" disks...")

// Global variables for disk tracking (no LOCAL so they're accessible in functions)
// Support up to 13 disks
peg1 := {13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1}
peg2 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
peg3 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

// Initialize only the requested number of disks
len1 := diskCount
len2 := 0
len3 := 0

// Clear unused disk positions
local i := 0
while i < 13
    if i < diskCount
        peg1[i] := diskCount - i
    else
        peg1[i] := 0
    endif
    i := i + 1
enddo

moveCount := 0

// Display initial state
DrawTowers()

// Wait a moment for visual effect
local delay := 0
while delay < 100000
    delay := delay + 1
enddo

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
? moveCount
OutStd(" moves (minimum: ")
? minMoves
OutStd(")")
SetPos(39, 0)
OutStd("")

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
    
    // Get disk from source peg and update length
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
    
    // Place disk on destination peg and update length
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
    
    // Update move counter and display
    moveCount := moveCount + 1
    DrawTowers()
    
    // Small delay for animation effect
    local pause := 0
    while pause < 50000
        pause := pause + 1
    enddo
    
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
    ? moveCount
    
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
    local diskNum := 0
    local arrayLen := 0
    
    // Determine which peg to get length for
    if column == 25
        arrayLen := len1
    else
        if column == 55
            arrayLen := len2
        else
            arrayLen := len3
        endif
    endif
    
    // Only draw if there are disks on this peg
    if arrayLen == 0
        return nil
    endif
    
    while diskNum < arrayLen
        local diskSize := pegArray[diskNum]
        
        // Skip empty slots (disks with value 0)
        if diskSize > 0
            local diskRow := baseRow - diskNum - 1
            local diskWidth := diskSize * 2 - 1
            local leftPos := column - diskSize + 1
            
            // Set color based on disk size (cycle through colors 1-14)
            local colorCode := diskSize % 14 + 1
            SetColor(colorCode)
            
            SetPos(diskRow, leftPos)
            // Draw disk dynamically based on size
            OutStd(Replicate("█", diskWidth))
            
            // Reset color to default
            SetColor(7)
        endif
        
        diskNum := diskNum + 1
    enddo
    
return nil

// Set console color (simplified - assumes ANSI color support)
// Colors 1-14 cycle through different foreground colors
function SetColor(colorNum)
    // Note: This is a placeholder for color functionality
    // In a real implementation, this would use ANSI escape codes
    // or console API calls to set colors
    // For now, it's a no-op but maintains API compatibility
return nil

// Power function helper
function Power(base, exp)
    local result := 1
    local i := 0
    while i < exp
        result := result * base
        i := i + 1
    enddo
return result

// String conversion helper
function Str(num)
    // Simple integer to string conversion
    if num == 0
        return "0"
    endif
    
    local result := ""
    local temp := num
    local digits := {}
    local digitCount := 0
    
    while temp > 0
        digits[digitCount] := temp % 10
        temp := temp / 10
        digitCount := digitCount + 1
    enddo
    
    // Build string from digits (reverse order)
    local i := digitCount - 1
    while i >= 0
        local digit := digits[i]
        if digit == 0
            result := result + "0"
        else
            if digit == 1
                result := result + "1"
            else
                if digit == 2
                    result := result + "2"
                else
                    if digit == 3
                        result := result + "3"
                    else
                        if digit == 4
                            result := result + "4"
                        else
                            if digit == 5
                                result := result + "5"
                            else
                                if digit == 6
                                    result := result + "6"
                                else
                                    if digit == 7
                                        result := result + "7"
                                    else
                                        if digit == 8
                                            result := result + "8"
                                        else
                                            result := result + "9"
                                        endif
                                    endif
                                endif
                            endif
                        endif
                    endif
                endif
            endif
        endif
        i := i - 1
    enddo
    
return result
