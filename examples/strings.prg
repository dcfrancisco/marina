ClearScreen()

// String operations
LOCAL name, greeting, fullMessage

name := "Clipper"
greeting := "Hello, "
fullMessage := greeting + name

? fullMessage
? "Length check:", name

// Comparison
IF name == "Clipper"
    ? "Name matches!"
ENDIF
? Replicate("=", 40)
