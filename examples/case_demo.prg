// CASE statement demonstration

? "=== CASE Statement Demo ==="

// Test 1: Simple number matching
LOCAL grade := 85

? "Grade:", grade

CASE grade
CASE 90
? "Grade: A (Excellent!)"
CASE 80
? "Grade: B (Good job!)"
CASE 70
? "Grade: C (Passing)"
OTHERWISE
? "Grade: Need improvement"
ENDCASE

// Test 2: String matching
LOCAL day := "Monday"

? ""
? "Day:", day

CASE day
CASE "Monday"
? "Start of the work week"
CASE "Friday"
? "TGIF!"
CASE "Saturday"
? "Weekend!"
CASE "Sunday"
? "Weekend!"
OTHERWISE
? "Middle of the week"
ENDCASE

// Test 3: Boolean matching
LOCAL status := TRUE

? ""
? "Status:", status

CASE status
CASE TRUE
? "System is active"
CASE FALSE
? "System is inactive"
ENDCASE

// Test 4: Expression evaluation
LOCAL x := 15

? ""
? "Value:", x

CASE x
CASE 10
? "Exactly 10"
CASE 15
? "Exactly 15 - matched!"
CASE 20
? "Exactly 20"
OTHERWISE
? "Some other value"
ENDCASE

? ""
? "=== CASE Demo Complete ==="
