# Procedure Main() - Entry Point

Marina supports Clipper-style `procedure Main()` as an automatic entry point for programs.

## How It Works

When you compile and run a Marina program:

1. **Top-level code executes first** - This includes global variable initialization and function/procedure definitions
2. **If `Main()` or `main()` exists, it's called automatically** - After all top-level code completes
3. **If no `Main()` exists, execution stops** - After top-level code completes (traditional script-style execution)

## Example with Main()

```clipper
// Global initialization runs first
myCounter := 0
myName := "Marina"

// Functions and procedures are defined
procedure Greet(name)
    OutStd("Hello, ")
    OutStd(name)
    OutStd("!")
    OutStd("")
return

// Main() is called automatically after initialization
procedure Main()
    local input := Space(50)
    
    Greet(myName)
    
    OutStd("Enter your name: ")
    input := GetInput(input)
    
    Greet(Trim(input))
    
    myCounter := myCounter + 1
    OutStd("Counter: ")
    OutStd(Str(myCounter))
    OutStd("")
return
```

**Execution order:**
1. `myCounter := 0` (global init)
2. `myName := "Marina"` (global init)
3. `Greet()` procedure definition stored
4. `Main()` procedure definition stored
5. `Main()` automatically called
6. Program exits when `Main()` returns

## Example without Main() (Traditional)

```clipper
// Top-level code executes directly
local x := 10
local y := 20

OutStd("Sum: ")
OutStd(Str(x + y))
OutStd("")
```

**Execution order:**
1. All statements execute in order
2. Program exits at end of file

## Benefits

### With `procedure Main()`:
- ✅ Clear entry point (like C, Pascal, etc.)
- ✅ Better organization for larger programs
- ✅ Easier to read and maintain
- ✅ Local variables in `Main()` instead of globals
- ✅ Matches classic Clipper style

### Without `Main()` (Script style):
- ✅ Quick scripts and simple programs
- ✅ Less boilerplate
- ✅ Good for testing and demos

## Global Variables

Global variables must be initialized **before** `Main()` (at the top level):

```clipper
// ✅ CORRECT - globals initialized before Main()
board := {1,2,3,4,5,6,7,8,9}
score := 0

procedure Main()
    // Can access and modify globals
    board[1] := 99
    score := score + 10
return
```

```clipper
// ❌ INCORRECT - can't initialize globals inside Main()
procedure Main()
    board := {1,2,3,4,5,6,7,8,9}  // This creates a LOCAL, not global!
    score := 0  // This creates a LOCAL, not global!
return
```

To use globals across functions, initialize them at the top level:

```clipper
// Global state
playerX := 5
playerY := 5

function MovePlayer(dx, dy)
    playerX := playerX + dx
    playerY := playerY + dy
return

procedure Main()
    MovePlayer(1, 0)  // Move right
    MovePlayer(0, 1)  // Move down
    
    OutStd("Player at: ")
    OutStd(Str(playerX))
    OutStd(", ")
    OutStd(Str(playerY))
    OutStd("")
return
```

## Case Sensitivity

Both `Main()` and `main()` work (checked in that order):

```clipper
procedure Main()  // ✅ Works
    OutStd("Hello")
return
```

```clipper
procedure main()  // ✅ Also works
    OutStd("Hello")
return
```

## Best Practices

1. **Use `procedure Main()` for complete programs** - Games, utilities, applications
2. **Use script style for simple tests** - Quick calculations, demos, experiments
3. **Initialize globals at top level** - Before any procedure definitions
4. **Keep `Main()` focused** - Call other procedures/functions for actual logic
5. **Use local variables in `Main()`** - Don't pollute global namespace

## Migration from Top-Level to Main()

If you have existing code without `Main()`, you can wrap it:

**Before:**
```clipper
// Top-level script
local name := Space(50)
OutStd("Name: ")
name := GetInput(name)
OutStd("Hello, ")
OutStd(Trim(name))
OutStd("")
```

**After:**
```clipper
// With Main() entry point
procedure Main()
    local name := Space(50)
    
    OutStd("Name: ")
    name := GetInput(name)
    
    OutStd("Hello, ")
    OutStd(Trim(name))
    OutStd("")
return
```

Both styles work! Choose what fits your project best.
