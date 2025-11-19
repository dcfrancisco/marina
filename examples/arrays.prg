// Array operations
ClearScreen()

local arr, i

arr := {10, 20, 30, 40, 50}

? "Array:", arr
? "First element:", arr[0]
? "Third element:", arr[2]

// Loop through array
for i := 0 to 4
    ? "Element", i, ":", arr[i]
next
