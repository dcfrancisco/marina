// Christmas Tree Demo
// Using SetPos() and loops to draw a festive tree

ClearScreen()

// Title
SetPos(1, 25)
OutStd("*** MERRY CHRISTMAS ***")
SetPos(2, 20)
OutStd("Clipper-2025 ASCII Art Demo")

// Draw the tree using loops
LOCAL row, col, width, i, spaces, stars

// Tree top (star)
SetPos(5, 39)
OutStd("*")

// Tree body - 5 sections getting wider
row := 6
FOR i := 1 TO 5
    width := i * 2 + 1
    spaces := 40 - width / 2
    SetPos(row, spaces)
    
    // Use Replicate to draw stars for this row
    OutStd(Replicate("*", width))
    
    row := row + 1
NEXT

// Middle section - wider part
FOR i := 1 TO 4
    width := 9 + i
    spaces := 40 - width / 2
    SetPos(row, spaces)
    
    OutStd(Replicate("*", width))
    
    row := row + 1
NEXT

// Bottom section - widest part
FOR i := 1 TO 4
    width := 13 + i
    spaces := 40 - width / 2
    SetPos(row, spaces)
    
    OutStd(Replicate("*", width))
    
    row := row + 1
NEXT

// Tree trunk
FOR i := 1 TO 3
    SetPos(row, 38)
    OutStd("|||")
    row := row + 1
NEXT

// Ornaments (using positioned numbers/letters)
SetPos(8, 38)
OutStd("o")
SetPos(9, 36)
OutStd("O")
SetPos(10, 42)
OutStd("o")
SetPos(11, 34)
OutStd("O")
SetPos(12, 44)
OutStd("o")
SetPos(13, 37)
OutStd("O")
SetPos(14, 41)
OutStd("o")

// Presents under tree
SetPos(row + 1, 33)
OutStd("[===]  [___]  {***}")

// Bottom message
SetPos(row + 3, 28)
OutStd("Happy Holidays from Clipper-2025!")

// Move cursor to bottom
SetPos(row + 5, 0)
? ""
