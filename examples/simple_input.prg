// Simple GetInput Test
// Tests basic GetInput functionality

ClearScreen()

local name := Space(20)
local age := Space(3)

SetPos(2, 5)
OutStd("Enter your name: ")
name := GetInput(name)

SetPos(4, 5)
OutStd("Enter your age: ")
age := GetInput(age)

SetPos(6, 5)
OutStd("Results:")
SetPos(7, 5)
OutStd("Name: [")
OutStd(Trim(name))
OutStd("]")
SetPos(8, 5)
OutStd("Age: [")
OutStd(Trim(age))
OutStd("]")

SetPos(10, 0)
OutStd("")
?
