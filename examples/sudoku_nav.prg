// Sudoku Puzzle Game with Arrow Key Navigation
// Use arrow keys to move cursor, 1-9 to place numbers, Q to quit

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

// Cursor position
cursorRow := 0
cursorCol := 0

// Draw game title and instructions
function DrawTitle()
    SetPos(0, 20)
    OutStd("╔═══════════════════════════════════════╗")
    SetPos(1, 20)
    OutStd("║        SUDOKU - Arrow Navigation      ║")
    SetPos(2, 20)
    OutStd("╚═══════════════════════════════════════╝")
    
    SetPos(3, 15)
    SetColor(14)
    OutStd("Arrow Keys: Move  |  1-9: Place Number  |  0/Del: Clear  |  Q: Quit")
    SetColor(7)
return nil

// Draw the Sudoku board with cursor highlight
function DrawBoard()
    local row := 0
    local col := 0
    
    SetPos(5, 25)
    OutStd("┌───────┬───────┬───────┐")
    
    while row < 9
        SetPos(6 + row + Int(row / 3), 25)
        OutStd("│ ")
        
        col := 0
        while col < 9
            local idx := row * 9 + col
            local num := board[idx]
            local isFixed := fixed[idx] == 1
            local isCursor := (row == cursorRow && col == cursorCol)
            
            // Highlight cursor position
            if isCursor
                SetColor(0)  // Black text
                if isFixed
                    SetColor(11)  // Cyan background (simulated with bright cyan text on reverse)
                else
                    SetColor(10)  // Green background
                endif
            else
                if num == 0
                    SetColor(8)  // Dark gray for empty
                else
                    if isFixed
                        SetColor(15)  // White for given numbers
                    else
                        SetColor(11)  // Cyan for user entries
                    endif
                endif
            endif
            
            if num == 0
                OutStd("· ")
            else
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
            SetPos(7 + row + Int(row / 3), 25)
            OutStd("├───────┼───────┼───────┤")
        endif
        
        row := row + 1
    enddo
    
    SetPos(15, 25)
    OutStd("└───────┴───────┴───────┘")
return nil

// Check if a number is valid in a specific row
function IsValidRow(row, num)
    local col := 0
    local count := 0
    
    while col < 9
        local idx := row * 9 + col
        if board[idx] == num
            count := count + 1
        endif
        col := col + 1
    enddo
    
    return count <= 1
    
// Check if a number is valid in a specific column
function IsValidCol(col, num)
    local row := 0
    local count := 0
    
    while row < 9
        local idx := row * 9 + col
        if board[idx] == num
            count := count + 1
        endif
        row := row + 1
    enddo
    
    return count <= 1

// Check if a number is valid in a 3x3 box
function IsValidBox(row, col, num)
    local boxRow := Int(row / 3) * 3
    local boxCol := Int(col / 3) * 3
    local r := 0
    local c := 0
    local count := 0
    
    while r < 3
        c := 0
        while c < 3
            local idx := (boxRow + r) * 9 + (boxCol + c)
            if board[idx] == num
                count := count + 1
            endif
            c := c + 1
        enddo
        r := r + 1
    enddo
    
    return count <= 1

// Check if entire board is valid
function IsValidBoard()
    local row := 0
    local col := 0
    
    while row < 9
        col := 0
        while col < 9
            local idx := row * 9 + col
            local num := board[idx]
            
            if num != 0
                if !IsValidRow(row, num) || !IsValidCol(col, num) || !IsValidBox(row, col, num)
                    return .F.
                endif
            endif
            
            col := col + 1
        enddo
        row := row + 1
    enddo
    
    return .T.

// Check if puzzle is completely solved
function IsSolved()
    local idx := 0
    
    // Check if all cells are filled
    while idx < 81
        if board[idx] == 0
            return .F.
        endif
        idx := idx + 1
    enddo
    
    // Check if board is valid
    return IsValidBoard()

// Show message at bottom
function ShowMessage(msg, color)
    SetPos(17, 10)
    OutStd(Replicate(" ", 70))
    SetPos(17, 10)
    SetColor(color)
    OutStd(msg)
    SetColor(7)
return nil

// Main game loop with Inkey navigation
procedure Main()
    local gameRunning := .T.
    local key := 0
    local idx := 0
    local num := 0
    
    ClearScreen()
    SetCursor(.F.)
    
    DrawTitle()
    DrawBoard()
    ShowMessage("Use arrow keys to navigate, 1-9 to place numbers", 11)
    
    while gameRunning
        // Get key press
        key := Inkey()
        
        if key > 0
            // Arrow keys (might vary by terminal)
            // Try common patterns: WASD and arrow escape sequences
            
            if key == 119 || key == 87  // W/w - Up
                if cursorRow > 0
                    cursorRow := cursorRow - 1
                    DrawBoard()
                endif
            endif
            
            if key == 115 || key == 83  // S/s - Down
                if cursorRow < 8
                    cursorRow := cursorRow + 1
                    DrawBoard()
                endif
            endif
            
            if key == 97 || key == 65  // A/a - Left
                if cursorCol > 0
                    cursorCol := cursorCol - 1
                    DrawBoard()
                endif
            endif
            
            if key == 100 || key == 68  // D/d - Right
                if cursorCol < 8
                    cursorCol := cursorCol + 1
                    DrawBoard()
                endif
            endif
            
            // Number keys 0-9
            if key >= 48 && key <= 57
                idx := cursorRow * 9 + cursorCol
                
                if fixed[idx] == 1
                    ShowMessage("Cannot change fixed cells!", 12)
                    Sleep(1000)
                    ShowMessage("Use arrow keys to navigate, 1-9 to place numbers", 11)
                else
                    num := key - 48  // Convert ASCII to number
                    board[idx] := num
                    DrawBoard()
                    
                    if IsSolved()
                        ShowMessage("CONGRATULATIONS! Puzzle solved!", 10)
                        Sleep(3000)
                        gameRunning := .F.
                    else
                        if num > 0 && !IsValidBoard()
                            ShowMessage("Invalid! Duplicate in row/col/box", 12)
                            Sleep(1000)
                            ShowMessage("Use arrow keys to navigate, 1-9 to place numbers", 11)
                        endif
                    endif
                endif
            endif
            
            // Q/q to quit
            if key == 113 || key == 81
                gameRunning := .F.
            endif
        endif
        
        Sleep(50)  // Small delay to reduce CPU usage
    enddo
    
    SetPos(20, 0)
    SetCursor(.T.)
    OutStd("")
return
