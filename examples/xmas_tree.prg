// Christmas Tree Demo
// Using SetPos() and loops to draw a festive triangle tree with star

ClearScreen()

// Title
SetPos(1, 25)
OutStd("*** MERRY CHRISTMAS ***")
SetPos(2, 20)
OutStd("Clipper-2025 ASCII Art Demo")

// Draw the tree using loops
local row, width, i, spaces

// Tree top - Star
SetPos(4, 39)
OutStd("*")

// Perfect triangle tree - each row adds 2 stars
row := 5
for i := 1 to 12
    width := i * 2 - 1
    spaces := 40 - (width + 1) / 2
    SetPos(row, spaces)

    // Use Replicate to draw stars for this row
    OutStd(Replicate("*", width))

    row := row + 1
next

// Tree trunk
for i := 1 to 3
    SetPos(row, 38)
    OutStd("|||")
    row := row + 1
next

// Ornaments (using positioned numbers/letters)
SetPos(7, 38)
OutStd("o")
SetPos(9, 36)
OutStd("O")
SetPos(10, 42)
OutStd("o")
SetPos(11, 34)
OutStd("O")
SetPos(12, 44)
OutStd("o")
SetPos(13, 33)
OutStd("O")
SetPos(14, 45)
OutStd("o")
SetPos(15, 37)
OutStd("O")

// Presents under tree
SetPos(row + 1, 30)
OutStd("[===]  [___]  {***}")

// Bottom message
SetPos(row + 3, 25)
OutStd("Happy Holidays from Clipper-2025!")

// Move cursor to bottom
SetPos(row + 5, 0)
? ""
