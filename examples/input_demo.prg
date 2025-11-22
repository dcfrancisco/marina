// Input Demo - Using GetInput() function
// Demonstrates simple form input with GetInput()

ClearScreen()

// Title
SetPos(0, 20)
OutStd("╔════════════════════════════════════╗")
SetPos(1, 20)
OutStd("║     CUSTOMER REGISTRATION FORM      ║")
SetPos(2, 20)
OutStd("╚════════════════════════════════════╝")

// Initialize variables with Space() for field lengths
local name := Space(30)
local email := Space(40)
local phone := Space(15)
local city := Space(20)
local age := Space(3)

// Input fields with prompts
SetPos(5, 10)
OutStd("Name....: ")
name := GetInput(name, 5, 20)

SetPos(7, 10)
OutStd("Email...: ")
email := GetInput(email, 7, 20)

SetPos(9, 10)
OutStd("Phone...: ")
phone := GetInput(phone, 9, 20)

SetPos(11, 10)
OutStd("City....: ")
city := GetInput(city, 11, 20)

SetPos(13, 10)
OutStd("Age.....: ")
age := GetInput(age, 13, 20)

// Display summary
SetPos(16, 10)
OutStd("════════════════════════════════════════")
SetPos(17, 10)
OutStd("SUMMARY:")
SetPos(18, 10)
OutStd("Name....: ")
OutStd(Trim(name))
SetPos(19, 10)
OutStd("Email...: ")
OutStd(Trim(email))
SetPos(20, 10)
OutStd("Phone...: ")
OutStd(Trim(phone))
SetPos(21, 10)
OutStd("City....: ")
OutStd(Trim(city))
SetPos(22, 10)
OutStd("Age.....: ")
OutStd(Trim(age))

SetPos(24, 0)
OutStd("")
?
