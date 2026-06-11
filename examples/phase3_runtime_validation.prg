IMPORT "string"
IMPORT "math"

FUNCTION DescribeScore(score)
    IF score >= 90
        RETURN "excellent"
    ELSEIF score >= 75
        RETURN "passing"
    ELSE
        RETURN "needs work"
    ENDIF

FUNCTION PickMessage(category, code)
    CASE category
        CASE "status"
            CASE code
                CASE 1
                    RETURN "ok"
                CASE 2
                    RETURN "warn"
                OTHERWISE
                    RETURN "unknown"
            ENDCASE
        OTHERWISE
            RETURN "invalid"
    ENDCASE

FUNCTION Add(a, b)
    RETURN a + b

FUNCTION Factorial(n)
    IF n <= 1
        RETURN 1
    ENDIF
    RETURN n * Factorial(n - 1)

FUNCTION Main()
    LOCAL score := 82
    LOCAL total := Add(10, 20)
    LOCAL label := DescribeScore(score)
    LOCAL status := PickMessage("status", 2)
    LOCAL fact := Factorial(5)
    LOCAL labelLen := string.len(label)
    LOCAL root := math.sqrt(81)

    ? "score:", score
    ? "label:", label
    ? "labelLen:", labelLen
    ? "status:", status
    ? "total:", total
    ? "root:", root
    ? "factorial:", fact

RETURN NIL
