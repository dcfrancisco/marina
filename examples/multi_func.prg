// Multiple functions example
ClearScreen()

LOCAL a, b, c, avg

a := 10
b := 20
c := 30

avg := Average(a, b, c)
? "Average of", a, b, c, "is", avg

? "Double of average:", Double(avg)
? "Square of 5:", Square(5)

FUNCTION Average(x, y, z)
    LOCAL sum
    sum := Add(x, Add(y, z))
RETURN sum / 3

FUNCTION Add(a, b)
RETURN a + b

FUNCTION Double(n)
RETURN n * 2

FUNCTION Square(n)
RETURN n * n
