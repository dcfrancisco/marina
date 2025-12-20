// Loop examples
ClearScreen()

local i, sum

// FOR loop
sum := 0
for i := 1 to 10
    sum := sum + i
next

? "Sum 1 to 10:", sum

// WHILE loop
i := 1
sum := 0
while i <= 5
    sum := sum + i
    i := i + 1
enddo

? "Sum 1 to 5:", sum

// DO WHILE loop
i := 1
do
    ? "Count:", i
    i := i + 1
while i <= 3
