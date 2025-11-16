// Test EXIT semantics and indexed assignment

? "Testing EXIT in LOOP:"
LOCAL counter := 0
LOOP
counter := counter + 1
? "Loop iteration:", counter
IF counter >= 3
? "Exiting loop"
EXIT
ENDIF
ENDLOOP
? "After loop, counter =", counter

? ""
? "Testing EXIT in WHILE:"
LOCAL x := 0
WHILE TRUE
x := x + 1
? "While iteration:", x
IF x >= 3
EXIT
ENDIF
ENDDO
? "After while, x =", x

? ""
? "Testing EXIT in FOR:"
FOR i := 1 TO 100
? "For iteration:", i
IF i >= 5
EXIT
ENDIF
NEXT
? "After for loop"

? ""
? "Testing indexed assignment:"
LOCAL arr := {10, 20, 30, 40, 50}
? "Original array:", arr[0], arr[1], arr[2], arr[3], arr[4]
arr[0] := 99
arr[2] := 88
arr[4] := 77
? "Modified array:", arr[0], arr[1], arr[2], arr[3], arr[4]

? ""
? "All Phase 1 features working!"
