// Multiple functions example
ClearScreen()

local a, b, c, avg

a := 10
b := 20
c := 30

avg := Average(a, b, c)
? "Average of", a, b, c, "is", avg

? "Double of average:", Double(avg)
? "Square of 5:", Square(5)

function Average(x, y, z)
    local sum
    sum := Add(x, Add(y, z))
return sum / 3

function Add(a, b)
return a + b

function Double(n)
return n * 2

function Square(n)
return n * n
