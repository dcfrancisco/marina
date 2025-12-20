// Color Demo - Shows all available ANSI colors
ClearScreen()

SetPos(0, 25)
OutStd("╔════════════════════════════════╗")
SetPos(1, 25)
OutStd("║     ANSI COLOR PALETTE DEMO    ║")
SetPos(2, 25)
OutStd("╚════════════════════════════════╝")

local row := 4

// Standard colors (0-7)
SetPos(row, 5)
OutStd("Standard Colors:")
row := row + 2

local color := 0
while color < 8
    SetPos(row, 5)
    SetColor(color)
    OutStd("████████")
    SetColor(7)
    OutStd(" Color ")
    OutStd(color)
    OutStd(" - ")

    // Color names
    if color == 0
        OutStd("Black")
    else
        if color == 1
            OutStd("Blue")
        else
            if color == 2
                OutStd("Green")
            else
                if color == 3
                    OutStd("Cyan")
                else
                    if color == 4
                        OutStd("Red")
                    else
                        if color == 5
                            OutStd("Magenta")
                        else
                            if color == 6
                                OutStd("Yellow")
                            else
                                OutStd("White")
                            endif
                        endif
                    endif
                endif
            endif
        endif
    endif

    row := row + 1
    color := color + 1
enddo

row := row + 1

// Bright colors (8-15)
SetPos(row, 5)
OutStd("Bright Colors:")
row := row + 2

color := 8
while color < 16
    SetPos(row, 5)
    SetColor(color)
    OutStd("████████")
    SetColor(7)
    OutStd(" Color ")
    OutStd(color)
    OutStd(" - Bright ")

    // Bright color names
    local baseColor := color - 8
    if baseColor == 0
        OutStd("Black (Gray)")
    else
        if baseColor == 1
            OutStd("Blue")
        else
            if baseColor == 2
                OutStd("Green")
            else
                if baseColor == 3
                    OutStd("Cyan")
                else
                    if baseColor == 4
                        OutStd("Red")
                    else
                        if baseColor == 5
                            OutStd("Magenta")
                        else
                            if baseColor == 6
                                OutStd("Yellow")
                            else
                                OutStd("White")
                            endif
                        endif
                    endif
                endif
            endif
        endif
    endif

    row := row + 1
    color := color + 1
enddo

// Reset color
SetColor(7)

row := row + 2
SetPos(row, 20)
OutStd("Colors are working! 🎨")

SetPos(row + 2, 0)
OutStd("")
