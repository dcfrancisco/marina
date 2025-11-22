// Sudoku Puzzle Game
// Fill in the missing numbers (1-9) so each row, column, and 3x3 box contains all digits

// Global board (81 cells, 0 = empty)
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

// UI Layout
promptCol := 10
boardCol := 25
titleCol := 28
msgRow := 18

// Draw game title
function DrawTitle()
    SetPos(0, titleCol - 10)
    OutStd("╔═══════════════════════════════════╗")
    SetPos(1, titleCol - 10)
    OutStd("║          SUDOKU PUZZLE            ║")
    SetPos(2, titleCol - 10)
    OutStd("╚═══════════════════════════════════╝")
    SetPos(3, 15)
    SetColor(14)
    OutStd("Fill each row, column, and 3x3 box with 1-9")
    SetColor(7)
return nil

// Draw the Sudoku board
function DrawBoard()
    local row := 0
    local col := 0
    local clearRow := 5
    
    // Clear board area
    while clearRow < 30
        SetPos(clearRow, 0)
        OutStd(Replicate(" ", 80))
        clearRow := clearRow + 1
    enddo
    
    SetPos(5, boardCol)
    OutStd("┌───────┬───────┬───────┐")
    
    while row < 9
        local lineOffset := 0
        if row > 2
            lineOffset := 1
        endif
        if row > 5
            lineOffset := 2
        endif
        
        SetPos(6 + row + lineOffset, boardCol)
        OutStd("│ ")
        
        col := 0
        while col < 9
            local idx := row * 9 + col
            local num := board[idx]
            local isFixed := fixed[idx] == 1
            
            if num == 0
                SetColor(8)
                OutStd("· ")
            else
                if isFixed
                    SetColor(15)
                else
                    SetColor(11)
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
        if row == 2
            SetPos(9, boardCol)
            OutStd("├───────┼───────┼───────┤")
        endif
        if row == 5
            SetPos(13, boardCol)
            OutStd("├───────┼───────┼───────┤")
        endif
        
        row := row + 1
    enddo
    
    SetPos(17, boardCol)
    OutStd("└───────┴───────┴───────┘")
return nil

// Check if puzzle is solved
function IsSolved()
    local i := 0
    while i < 81
        if board[i] == 0
            return .F.
        endif
        i := i + 1
    enddo
    return IsValidBoard()
return nil

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

// Check if row is valid
function IsValidRow(row)
    local used := {0,0,0,0,0,0,0,0,0,0}
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

procedure Main()
    local gameRunning := .T.
    
    ClearScreen()
    SetCursor(.F.)
    DrawTitle()
    DrawBoard()
    
    while gameRunning
        SetPos(19, promptCol)
        OutStd("Enter row (1-9), col (1-9), num (1-9) or 'Q' to quit: ")
        SetPos(19, promptCol + 55)
        OutStd(Replicate(" ", 15))
        SetPos(19, promptCol + 55)
        
        local input := Space(10)
        input := GetInput(input)
        input := Trim(input)
        
        // Check for quit
        if input == "Q" || input == "q"
            gameRunning := .F.
        else
            local row := 0
            local col := 0
            local num := 0
            
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
            
            if row >= 1 && row <= 9 && col >= 1 && col <= 9 && num >= 0 && num <= 9
                local idx := (row - 1) * 9 + (col - 1)
                
                if fixed[idx] == 1
                    SetPos(msgRow, 10)
                    SetColor(12)
                    OutStd("That cell is fixed! Can't change it.")
                    SetColor(7)
                    Sleep(1500)
                else
                    board[idx] := num
                    DrawBoard()
                    if IsSolved()
                        SetPos(msgRow, 10)
                        SetColor(10)
                        OutStd("╔════════════════════════════════════════╗")
                        SetPos(msgRow + 1, 10)
                        OutStd("║  CONGRATULATIONS! PUZZLE SOLVED! 🎉  ║")
                        SetPos(msgRow + 2, 10)
                        OutStd("╚════════════════════════════════════════╝")
                        SetColor(7)
                        Sleep(3000)
                        gameRunning := .F.
                    else
                        if !IsValidBoard()
                            SetPos(msgRow, 10)
                            SetColor(12)
                            OutStd("Invalid! Duplicate number in row/col/box.")
                            SetColor(7)
                            Sleep(1500)
                        endif
                    endif
                endif
            else
                SetPos(msgRow, 10)
                SetColor(12)
                OutStd("Invalid input! Use format: row col num (e.g., 1 3 5)")
                SetColor(7)
                Sleep(1500)
            endif
        endif
        
        SetPos(msgRow, 10)
        OutStd(Replicate(" ", 60))
    enddo
    
    SetPos(22, 0)
    SetCursor(.T.)
    OutStd("")
return
