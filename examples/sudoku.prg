// Sudoku Puzzle Game
// Classic 9x9 Sudoku solver
// Fill in the missing numbers (1-9) so each row, column, and 3x3 box contains all digits

// Global board (81 cells, 0 = empty)
// Using global so functions can modify it
board := {5,3,0,0,7,0,0,0,0,
          6,0,0,1,9,5,0,0,0,
          0,9,8,0,0,0,0,6,0,
          8,0,0,0,6,0,0,0,3,
          4,0,0,8,0,3,0,0,1,
          7,0,0,0,2,0,0,0,6,
          0,6,0,0,0,0,2,8,0,
          0,0,0,4,1,9,0,0,5,
          0,0,0,0,8,0,0,7,9}

// Track which cells are fixed (can't be changed)
fixed := {1,1,0,0,1,0,0,0,0,
          1,0,0,1,1,1,0,0,0,
          0,1,1,0,0,0,0,1,0,
          1,0,0,0,1,0,0,0,1,
          1,0,0,1,0,1,0,0,1,
          1,0,0,0,1,0,0,0,1,
          0,1,0,0,0,0,1,1,0,
          0,0,0,1,1,1,0,0,1,
          0,0,0,0,1,0,0,1,1}

// Draw game title and instructions
function DrawTitle()
    SetPos(0, 25)
    OutStd("╔═══════════════════════════════════╗")
    SetPos(1, 25)
    OutStd("║          SUDOKU PUZZLE            ║")
    SetPos(2, 25)
    OutStd("╚═══════════════════════════════════╝")
    
    SetPos(3, 20)
    SetColor(14)
    OutStd("Fill each row, column, and 3x3 box with 1-9")
    SetColor(7)
return nil

// Draw the Sudoku board
function DrawBoard()
    local row := 0
    local col := 0
    
    // Clear board area
    local clearRow := 5
    while clearRow < 30
        SetPos(clearRow, 0)
        OutStd(Replicate(" ", 80))
        clearRow := clearRow + 1
    enddo
    
    SetPos(5, 20)
    OutStd("┌───────┬───────┬───────┐")
    
    while row < 9
        SetPos(6 + row + Int(row / 3), 20)
        OutStd("│ ")
        
        col := 0
        while col < 9
            local idx := row * 9 + col
            local num := board[idx]
            local isFixed := fixed[idx] == 1
            
            if num == 0
                SetColor(8)  // Dark gray for empty
                OutStd("· ")
            else
                if isFixed
                    SetColor(15)  // White for given numbers
                else
                    SetColor(11)  // Cyan for user entries
                endif
                OutStd(Str(num))
                OutStd(" ")
            endif
            SetColor(7)
            
            if (col + 1) % 3 == 0 && col < 8
                OutStd("│ ")
            endif
            
            col := col + 1
        enddo
        
        OutStd("│")
        
        // Draw horizontal separator after rows 2, 5
        if (row == 2 || row == 5)
            SetPos(7 + row + Int(row / 3), 20)
            OutStd("├───────┼───────┼───────┤")
        endif
        
        row := row + 1
    enddo
    
    SetPos(15, 20)
    OutStd("└───────┴───────┴───────┘")
return nil

// Check if puzzle is complete and valid
function IsSolved()
    local i := 0
    while i < 81
        if board[i] == 0
            return .F.
        endif
        i := i + 1
    enddo
    
    // Check all rows, columns, and boxes are valid
    return IsValidBoard()

// Check if current board state is valid
function IsValidBoard()
    local i := 0
    
    // Check rows
    while i < 9
        if !IsValidRow(i)
            return .F.
        endif
        i := i + 1
    enddo
    
    // Check columns
    i := 0
    while i < 9
        if !IsValidCol(i)
            return .F.
        endif
        i := i + 1
    enddo
    
    // Check 3x3 boxes
    local boxRow := 0
    while boxRow < 3
        local boxCol := 0
        while boxCol < 3
            if !IsValidBox(boxRow, boxCol)
                return .F.
            endif
            boxCol := boxCol + 1
        enddo
        boxRow := boxRow + 1
    enddo
    
    return .T.

// Check if row is valid (no duplicates)
function IsValidRow(row)
    local used := {0,0,0,0,0,0,0,0,0,0}  // index 1-9
    local col := 0
    
    while col < 9
        local idx := row * 9 + col
        local num := board[idx]
        
        if num != 0
            if used[num] == 1
                return .F.
            endif
            used[num] := 1
        endif
        
        col := col + 1
    enddo
    
    return .T.

// Check if column is valid
function IsValidCol(col)
    local used := {0,0,0,0,0,0,0,0,0,0}
    local row := 0
    
    while row < 9
        local idx := row * 9 + col
        local num := board[idx]
        
        if num != 0
            if used[num] == 1
                return .F.
            endif
            used[num] := 1
        endif
        
        row := row + 1
    enddo
    
    return .T.

// Check if 3x3 box is valid
function IsValidBox(boxRow, boxCol)
    local used := {0,0,0,0,0,0,0,0,0,0}
    local startRow := boxRow * 3
    local startCol := boxCol * 3
    local r := 0
    
    while r < 3
        local c := 0
        while c < 3
            local idx := (startRow + r) * 9 + (startCol + c)
            local num := board[idx]
            
            if num != 0
                if used[num] == 1
                    return .F.
                endif
                used[num] := 1
            endif
            
            c := c + 1
        enddo
        r := r + 1
    enddo
    
    return .T.

// Main entry point
procedure Main()
    ClearScreen()
    SetCursor(.F.)
    
    DrawTitle()
    DrawBoard()
    
    local gameRunning := .T.
    
    while gameRunning
        // Get user input
        SetPos(17, 10)
        OutStd("Enter row (1-9), col (1-9), num (1-9) or 'Q' to quit: ")
        SetPos(17, 65)
        OutStd(Replicate(" ", 15))
        SetPos(17, 65)
        
        local input := Space(10)
        input := GetInput(input)
        input := Trim(input)
        
        // Check for quit
        if input == "Q" || input == "q"
            gameRunning := .F.
        else
            // Parse input: "row col num" or "row,col,num"
            local row := 0
            local col := 0
            local num := 0
            
            // Try to parse
            if Len(input) >= 5
                row := Val(SubStr(input, 1, 1))
                col := Val(SubStr(input, 3, 1))
                num := Val(SubStr(input, 5, 1))
            else
                if Len(input) >= 3
                    row := Val(SubStr(input, 1, 1))
                    col := Val(SubStr(input, 2, 1))
                    num := Val(SubStr(input, 3, 1))
                endif
            endif
            
            // Validate and make move
            if row >= 1 && row <= 9 && col >= 1 && col <= 9 && num >= 0 && num <= 9
                local idx := (row - 1) * 9 + (col - 1)
                
                if fixed[idx] == 1
                    SetPos(18, 10)
                    SetColor(12)
                    OutStd("That cell is fixed! Can't change it.")
                    SetColor(7)
                    Sleep(1500)
                else
                    board[idx] := num
                    DrawBoard()
                    
                    // Check if solved
                    if IsSolved()
                        SetPos(18, 10)
                        SetColor(10)
                        OutStd("╔════════════════════════════════════════╗")
                        SetPos(19, 10)
                        OutStd("║  CONGRATULATIONS! PUZZLE SOLVED! 🎉  ║")
                        SetPos(20, 10)
                        OutStd("╚════════════════════════════════════════╝")
                        SetColor(7)
                        Sleep(3000)
                        gameRunning := .F.
                    else
                        if !IsValidBoard()
                            SetPos(18, 10)
                            SetColor(12)
                            OutStd("Invalid! Duplicate number in row/col/box.")
                            SetColor(7)
                            Sleep(1500)
                        endif
                    endif
                endif
            else
                SetPos(18, 10)
                SetColor(12)
                OutStd("Invalid input! Use format: row col num (e.g., 1 3 5)")
                SetColor(7)
                Sleep(1500)
            endif
        endif
        
        // Clear message area
        SetPos(18, 10)
        OutStd(Replicate(" ", 60))
    enddo
    
    SetPos(22, 0)
    SetCursor(.T.)
    OutStd("")
return
