// Tower of Hanoi - Simple ASCII Animation
// Demonstrates recursion and console positioning
// No user input version - just runs automatically

ClearScreen()

// Title screen
SetPos(0, 25)
OutStd("╔════════════════════════════════╗")
SetPos(1, 25)
OutStd("║     TOWER OF HANOI PUZZLE      ║")
SetPos(2, 25)
OutStd("╚════════════════════════════════╝")

// Configuration (change these values to customize)
diskCount := 4
moveCount := 0

// Initialize pegs with 4 disks
peg1 := {4, 3, 2, 1, 0, 0, 0}
peg2 := {0, 0, 0, 0, 0, 0, 0}
peg3 := {0, 0, 0, 0, 0, 0, 0}

len1 := 4
len2 := 0
len3 := 0

// Display initial state
SetPos(4, 10)
OutStd("Solving puzzle with 4 disks...")
SetPos(5, 10)
OutStd("Watch the animation!")

DrawTowers()

// Wait a moment for visual effect
local delay := 0
while delay < 100000
    delay := delay + 1
enddo

// Solve the puzzle
SetPos(20, 0)
OutStd("")
SolveHanoi(diskCount, 1, 3, 2)

// Final message
SetPos(21, 10)
OutStd("*** PUZZLE SOLVED! ***")
SetPos(22, 10)
OutStd("Total moves: 15 (minimum required)")
SetPos(24, 0)
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
    
    // Animation delay
    local pause := 0
    while pause < 50000
        pause := pause + 1
    enddo
    
return nil

// Draw all three towers with disks
function DrawTowers()
    local baseRow := 18
    local pegA := 15
    local pegB := 35
    local pegC := 55
    
    // Clear the tower area
    local clearRow := 7
    while clearRow <= 19
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
    OutStd("Move: ")
    OutStd(IntToStr(moveCount))
    
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
                    if diskNum == 3
                        diskSize := pegArray[3]
                    else
                        if diskNum == 4
                            diskSize := pegArray[4]
                        else
                            if diskNum == 5
                                diskSize := pegArray[5]
                            else
                                diskSize := pegArray[6]
                            endif
                        endif
                    endif
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
                            if diskSize == 5
                                SetPos(diskRow, column - 5)
                                OutStd("═══════════")
                            else
                                if diskSize == 6
                                    SetPos(diskRow, column - 6)
                                    OutStd("═════════════")
                                else
                                    SetPos(diskRow, column - 7)
                                    OutStd("═══════════════")
                                endif
                            endif
                        endif
                    endif
                endif
            endif
        endif
        
        diskNum := diskNum + 1
    enddo
    
return nil

// Integer to string conversion
function IntToStr(num)
    if num == 0
        return "0"
    endif
    
    if num < 0
        return "-"
    endif
    
    local result := ""
    local temp := num
    
    while temp > 0
        local digit := temp % 10
        
        if digit == 0
            result := "0" + result
        else
            if digit == 1
                result := "1" + result
            else
                if digit == 2
                    result := "2" + result
                else
                    if digit == 3
                        result := "3" + result
                    else
                        if digit == 4
                            result := "4" + result
                        else
                            if digit == 5
                                result := "5" + result
                            else
                                if digit == 6
                                    result := "6" + result
                                else
                                    if digit == 7
                                        result := "7" + result
                                    else
                                        if digit == 8
                                            result := "8" + result
                                        else
                                            result := "9" + result
                                        endif
                                    endif
                                endif
                            endif
                        endif
                    endif
                endif
            endif
        endif
        
        temp := temp / 10
    enddo
    
return result
