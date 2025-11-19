// CASE statement demonstration

? "=== CASE Statement Demo ==="

// Test 1: Simple number matching
local grade := 85

? "Grade:", grade

case grade
case 90
? "Grade: A (Excellent!)"
case 80
? "Grade: B (Good job!)"
case 70
? "Grade: C (Passing)"
otherwise
? "Grade: Need improvement"
endcase

// Test 2: String matching
local day := "Monday"

? ""
? "Day:", day

case day
case "Monday"
? "Start of the work week"
case "Friday"
? "TGIF!"
case "Saturday"
? "Weekend!"
case "Sunday"
? "Weekend!"
otherwise
? "Middle of the week"
endcase

// Test 3: Boolean matching
local status := TRUE

? ""
? "Status:", status

case status
case TRUE
? "System is active"
case FALSE
? "System is inactive"
endcase

// Test 4: Expression evaluation
local x := 15

? ""
? "Value:", x

case x
case 10
? "Exactly 10"
case 15
? "Exactly 15 - matched!"
case 20
? "Exactly 20"
otherwise
? "Some other value"
endcase

? ""
? "=== CASE Demo Complete ==="
