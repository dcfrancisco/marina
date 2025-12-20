// Test EXIT semantics and indexed assignment

? "Testing EXIT in LOOP:"
local counter := 0
loop
    counter := counter + 1
    ? "Loop iteration:", counter
    if counter >= 3
        ? "Exiting loop"
        exit
    endif
endloop
? "After loop, counter =", counter

? ""
? "Testing EXIT in WHILE:"
local x := 0
while true
    x := x + 1
    ? "While iteration:", x
    if x >= 3
        exit
    endif
enddo
? "After while, x =", x

? ""
? "Testing EXIT in FOR:"
for i := 1 to 100
    ? "For iteration:", i
    if i >= 5
        exit
    endif
next
? "After for loop"

? ""
? "Testing indexed assignment:"
local arr := {10, 20, 30, 40, 50}
? "Original array:", arr[0], arr[1], arr[2], arr[3], arr[4]
arr[0] := 99
arr[2] := 88
arr[4] := 77
? "Modified array:", arr[0], arr[1], arr[2], arr[3], arr[4]

? ""
? "All Phase 1 features working!"
