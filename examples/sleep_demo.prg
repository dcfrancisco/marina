// Sleep Function Demo
Print("Testing Sleep() function...")
Print("")

Print("Starting countdown...")
local i := 3
while i > 0
    Print(i)
    Sleep(1000) // Sleep for 1 second (1000 milliseconds)
    i := i - 1
enddo

Print("Blast off! 🚀")
Print("")

// Quick animation
Print("Quick dots animation:")
OutStd("Loading")
local dots := 0
while dots < 10
    OutStd(".")
    Sleep(200) // 200ms between dots
    dots := dots + 1
enddo
Print(" Done!")
Print("")
Print("Sleep() is working perfectly!")
