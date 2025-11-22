// Tower of Hanoi - Classic ASCII Animation
// Demonstrates recursion and console positioning
// Similar to Turbo Basic / Quick Basic versions

ClearScreen()

// Title screen
SetPos(0, 25)
OutStd("╔════════════════════════════════╗")
SetPos(1, 25)
OutStd("║     TOWER OF HANOI PUZZLE      ║")
SetPos(2, 25)
OutStd("╚════════════════════════════════╝")

// Global variables for disk tracking (no LOCAL so they're accessible in functions)
peg1 := {4, 3, 2, 1}  // Disks on peg 1 (largest to smallest)
peg2 := {0, 0, 0, 0}  // Empty peg 2 (using 0 as placeholder)
peg3 := {0, 0, 0, 0}  // Empty peg 3 (using 0 as placeholder)

len1 := 4  // Track lengths of each peg
len2 := 0
len3 := 0

diskCount := 4  // Number of disks (3-5 works well visually)
moveCount := 0

// Display initial state
SetPos(4, 10)
OutStd("Moving 4 disks from Peg A to Peg C...")
SetPos(5, 10)
OutStd("Watch the animation...")

DrawTowers()

// Wait a moment for visual effect
local delay := 0
while delay < 100000
    delay := delay + 1
enddo

// Solve the puzzle
SetPos(18, 0)
OutStd("")
SolveHanoi(diskCount, 1, 3, 2)

// Final message
SetPos(19, 10)
OutStd("Puzzle solved!")
SetPos(20, 10)
OutStd("Minimum moves for 4 disks: 15")
SetPos(22, 0)
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
    local baseRow := 16
    local pegA := 15
    local pegB := 35
    local pegC := 55
    
    // Clear the tower area
    local clearRow := 7
    while clearRow <= 17
        SetPos(clearRow, 0)
        OutStd(Replicate(" ", 80))
        clearRow := clearRow + 1
    enddo
    
    // Draw peg labels
    SetPos(baseRow + 1, pegA)
    OutStd("A")
    SetPos(baseRow + 1, pegB)
    OutStd("B")
    SetPos(baseRow + 1, pegC)
    OutStd("C")
    
    // Draw move counter
    SetPos(7, 10)
    OutStd("Moves: ")
    
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
    if column == 15
        arrayLen := len1
    else
        if column == 35
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
        local diskSize := 0
        
        // Get disk size based on index
        if diskNum == 0
            diskSize := pegArray[0]
        else
            if diskNum == 1
                diskSize := pegArray[1]
            else
                if diskNum == 2
                    diskSize := pegArray[2]
                else
                    diskSize := pegArray[3]
                endif
            endif
        endif
        
        // Skip empty slots (disks with value 0)
        if diskSize > 0
            local diskRow := baseRow - diskNum - 1
        
            // Draw disk based on size
            if diskSize == 1
                SetPos(diskRow, column - 1)
                OutStd("═══")
            else
                if diskSize == 2
                    SetPos(diskRow, column - 2)
                    OutStd("═════")
                else
                    if diskSize == 3
                        SetPos(diskRow, column - 3)
                        OutStd("═══════")
                    else
                        if diskSize == 4
                            SetPos(diskRow, column - 4)
                            OutStd("═════════")
                        else
                            // Size 5
                            SetPos(diskRow, column - 5)
                            OutStd("═══════════")
                        endif
                    endif
                endif
            endif
        endif
        
        diskNum := diskNum + 1
    enddo
    
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
