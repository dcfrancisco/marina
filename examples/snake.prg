// Snake Game Example for Marina
// This is a simple implementation of the classic Snake game.

PROCEDURE main()
    LOCAL width := 20
    LOCAL height := 10
    LOCAL snake := {{5, 5}}
    LOCAL direction := "RIGHT"
    LOCAL food := {RANDOM(width), RANDOM(height)}
    LOCAL score := 0
    LOCAL gameOver := .F.
    LOCAL key
    
    CLS()
    WHILE !gameOver
        // Draw the game
        CLS()
        ? "Score: ", score
        FOR y := 1 TO height
            LOCAL line := ""
            FOR x := 1 TO width
                IF {x, y} == food
                    line += "*"
                ELSEIF ASCAN(snake, {x, y}) > 0
                    line += "O"
                ELSE
                    line += "."
                ENDIF
            NEXT
            ? line
        NEXT
        
        // Input
        IF INKEY(0.1) > 0
            key := UPPER(CHR(LASTKEY()))
            IF key == "W" .AND. direction != "DOWN"
                direction := "UP"
            ELSEIF key == "S" .AND. direction != "UP"
                direction := "DOWN"
            ELSEIF key == "A" .AND. direction != "RIGHT"
                direction := "LEFT"
            ELSEIF key == "D" .AND. direction != "LEFT"
                direction := "RIGHT"
            ENDIF
        ENDIF
        
        // Move snake
        LOCAL head := snake[1]
        LOCAL newHead := head
        IF direction == "UP"
            newHead := {head[1], head[2] - 1}
        ELSEIF direction == "DOWN"
            newHead := {head[1], head[2] + 1}
        ELSEIF direction == "LEFT"
            newHead := {head[1] - 1, head[2]}
        ELSEIF direction == "RIGHT"
            newHead := {head[1] + 1, head[2]}
        ENDIF
        
        // Check collisions
        IF newHead[1] < 1 .OR. newHead[1] > width .OR. newHead[2] < 1 .OR. newHead[2] > height .OR. ASCAN(snake, newHead) > 0
            gameOver := .T.
            LOOP
        ENDIF
        
        // Eat food
        IF newHead == food
            score++
            AADD(snake, {}) // Grow snake
            food := {RANDOM(width), RANDOM(height)}
        ENDIF
        
        // Move snake body
        FOR i := LEN(snake) TO 2 STEP -1
            snake[i] := snake[i-1]
        NEXT
        snake[1] := newHead
        
        // Slow down
        SLEEP(0.1)
    END
    CLS()
    ? "Game Over! Final Score: ", score
ENDPROC


