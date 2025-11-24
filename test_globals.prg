// Test global variables with array methods
import "console"

testArray := [1, 2, 3]

function printArray()
    console.print("Array: ")
    console.print(testArray[1])
    console.print(", ")
    console.print(testArray[2])
    console.print(", ")
    console.print(testArray[3])
end

printArray()

return nil
