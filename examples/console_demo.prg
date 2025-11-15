// Console Positioning Demo
// Demonstrates ANSI terminal cursor control
// Modern replacement for Clipper 5.3 @ SAY commands

ClearScreen()

// Title
SetPos(0, 0)
? "╔════════════════════════════════════════╗"
? "║  Clipper-2025 Console Positioning Demo ║"
? "╚════════════════════════════════════════╝"
? ""

// Basic positioning
SetPos(5, 10)
OutStd("This is at row 5, column 10")

SetPos(7, 20)
OutStd("Moved to row 7, column 20")

SetPos(9, 5)
OutStd("Back to column 5, row 9")

// Draw a simple box using positioning
SetPos(12, 15)
OutStd("+----------------------+")
SetPos(13, 15)
OutStd("|   Positioned Text    |")
SetPos(14, 15)
OutStd("|   Box Demo           |")
SetPos(15, 15)
OutStd("+----------------------+")

// Test SavePos/RestorePos
SetPos(18, 5)
OutStd("Saving position...")
SavePos()

SetPos(20, 20)
OutStd("(This text is elsewhere)")

RestorePos()
OutStd(" and restored!")

// Position at bottom and continue
SetPos(23, 0)
? ""
? "Built-in Console Functions:"
? "  • SetPos(row, col) / DevPos(row, col)"
? "  • GotoXY(col, row)"
? "  • OutStd(text)"
? "  • ClearScreen()"
? "  • SavePos() / RestorePos()"
? ""
? "All use ANSI escape codes - compatible with modern terminals!"
? ""

