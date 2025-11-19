// Augmented assignment operators demonstration

? "=== Augmented Assignment Operators Demo ==="

// Test += operator
local x := 10
? "x =", x
x += 5
? "After x += 5:", x

// Test -= operator  
local y := 20
? ""
? "y =", y
y -= 7
? "After y -= 7:", y

// Test *= operator
local z := 3
? ""
? "z =", z
z *= 4
? "After z *= 4:", z

// Test /= operator
local w := 100
? ""
? "w =", w
w /= 5
? "After w /= 5:", w

// Test ++ operator (post-increment)
local counter := 0
? ""
? "counter =", counter
counter++
? "After counter++:", counter
counter++
? "After counter++:", counter

// Test -- operator (post-decrement)
local countdown := 10
? ""
? "countdown =", countdown
countdown--
? "After countdown--:", countdown
countdown--
? "After countdown--:", countdown

// Complex expressions
local total := 100
? ""
? "total =", total
total += 50
? "total += 50:", total
total -= 30
? "total -= 30:", total
total *= 2
? "total *= 2:", total
total /= 4
? "total /= 4:", total

// Using in loops
? ""
? "Using ++ in a loop:"
local i := 0
while i < 5
    ? "  i =", i
    i++
enddo

? ""
? "=== Demo Complete ==="
