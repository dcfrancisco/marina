// Loop examples
LOCAL i, sum

// FOR loop
sum := 0
FOR i := 1 TO 10
sum := sum + i
NEXT

? "Sum 1 to 10:", sum

// WHILE loop
i := 1
sum := 0
WHILE i <= 5
sum := sum + i
i := i + 1
ENDDO

? "Sum 1 to 5:", sum

// DO WHILE loop
i := 1
DO
? "Count:", i
i := i + 1
WHILE i <= 3
