// Cursor Visibility Demo
ClearScreen()

Print("Cursor Visibility Demo")
Print("======================")
Print("")

Print("Cursor is currently visible (blinking)")
Sleep(2000)

Print("")
Print("Hiding cursor in 1 second...")
Sleep(1000)

SetCursor(false)
Print("Cursor is now hidden!")
Print("Watch - no blinking cursor...")
Sleep(3000)

Print("")
Print("Showing cursor in 1 second...")
Sleep(1000)

SetCursor(true)
Print("Cursor is back!")
Print("")
Print("Demo complete.")
