// GetSecret() Demo - Password Input
// Demonstrates hidden input with asterisks

ClearScreen()

// Title
SetPos(0, 20)
OutStd("╔════════════════════════════════════╗")
SetPos(1, 20)
OutStd("║      SECURE LOGIN SYSTEM           ║")
SetPos(2, 20)
OutStd("╚════════════════════════════════════╝")

// Example 1: Simple password input
SetPos(5, 10)
OutStd("Example 1: Basic Password Input")
SetPos(6, 10)
OutStd("═══════════════════════════════════")

local password := Space(20)
SetPos(8, 10)
OutStd("Enter password: ")
password := GetSecret(password)

SetPos(10, 10)
OutStd("Password entered (length: ")
? Len(Trim(password))
OutStd(")")

// Example 2: With prompt parameter
SetPos(13, 10)
OutStd("Example 2: With Prompt Parameter")
SetPos(14, 10)
OutStd("═══════════════════════════════════")

local pin := Space(4)
SetPos(16, 10)
pin := GetSecret(pin, 16, 10, false, "Enter PIN: ")

SetPos(18, 10)
OutStd("PIN length: ")
? Len(Trim(pin))

// Example 3: Login form
SetPos(21, 10)
OutStd("Example 3: Login Form")
SetPos(22, 10)
OutStd("═══════════════════════════════════")

local username := Space(20)
local userpass := Space(20)

SetPos(24, 10)
OutStd("Username: ")
username := GetInput(username, 24, 20)

SetPos(26, 10)
OutStd("Password: ")
userpass := GetSecret(userpass, 26, 20)

// Show summary (trimmed values)
SetPos(29, 10)
OutStd("Login Attempt:")
SetPos(30, 12)
OutStd("User: [")
OutStd(Trim(username))
OutStd("]")
SetPos(31, 12)
OutStd("Pass: [")
OutStd(Replicate("*", Len(Trim(userpass))))
OutStd("]")

SetPos(33, 10)
OutStd("(Password remains hidden)")

SetPos(35, 0)
OutStd("")
?
