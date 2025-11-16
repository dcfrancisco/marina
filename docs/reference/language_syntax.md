# **Clipper-2025 Language Manual**

*The modern successor to Clipper 5.x — redesigned, simplified, extended.*

---

# 🌊 **1. Introduction**

Clipper-2025 is a modern, expression-first, functional-procedural language inspired by Clipper 5.x — preserving the parts that made Clipper elegant while removing legacy xBase baggage.

This language manual describes:

* syntax
* statements
* types
* functions
* blocks
* expressions
* modules
* database APIs
* standard library
* future extensions
* compatibility notes

---

# **2. Design Philosophy**

**Clipper-2025 preserves:**

* FUNCTION / LOCAL / RETURN
* `:=` assignment
* codeblocks `{||}`
* dynamic typing
* simple syntax
* readable programs
* low ceremony

**Clipper-2025 removes:**

* workareas
* all xBase commands
* macros (`&`), except through future macros
* preprocessor hacks
* `@ SAY/GET`
* DBF-only thinking
* linking OBJ+EXE
* Clipper DOS UI

**Clipper-2025 adds:**

* portable bytecode
* VM execution
* modules
* multiple database backends
* modern functions & scoping
* pattern-matching blocks (future)
* optional macro system
* package manager (future)
* better error handling
* modern data types
* deeper extensibility

---

# **3. Lexical Structure**

### Identifiers

```
[a-zA-Z_][a-zA-Z0-9_]*
```

Lowercase is recommended but not enforced.

### Whitespace

Whitespace and newlines separate tokens but carry no semantic meaning.

### Comments

```clipper
// single-line
/* multi
   line */
```

### Literals

```
123        // integer
12.34      // float
"hello"    // string
nil        // null
true       // boolean
false      // boolean
```

---

# **4. Statements**

### Assignment

```
x := 10
name := "Danny"
```

### Expression statement

```
Print("hello")
```

### Return

```
return value
```

### Blocks

Used to group expressions:

```
{
    a := 10
    b := a * 2
    return b
}
```

---

# **5. Variables**

### LOCAL

Block-scoped variables:

```clipper
function example()
    local x := 10
    return x
```

### GLOBAL (future)

Will be introduced via module system.

---

# **6. Functions**

### Function Definition

```clipper
function add(a, b)
    return a + b
```

### Nested Functions (planned)

```clipper
function outer()
    function inner()
        return 10
    return inner()
```

### No Function Hoisting

Functions must be defined before use (Clipper legacy allowed it; 2025 version does not).

---

# **7. Expressions**

### Arithmetic

```
+, -, *, /, %
```

### Comparison

```
==, !=, >, >=, <, <=
```

### Logical

```
and, or, not
```

### Concatenation

```
"hello" + "world"
```

---

# **8. Codeblocks**

Clipper-2025 keeps the legendary feature:

```clipper
b := {|| x + 10}
result := Eval(b)
```

More advanced syntax will be added later:

```clipper
map(values, {|n| n*n })
```

---

# **9. Modules**

Syntax:

```clipper
import "dbx"
import "string"
```

Calling module functions:

```clipper
db := dbx.open("customer.dbf")
print(db.count())
```

---

# **10. Error Handling (Future)**

Planned syntax:

```clipper
try
    risky()
catch e
    print("Error: " + e.message)
end
```

---

# **11. Types**

Clipper-2025 includes:

* number
* string
* boolean
* list (planned)
* map/dict (planned)
* object (future OOP)
* nil

---

# **12. Standard Library Overview**

### Print

```
Print("hello")
```

### Math

```
Abs(), Round(), Floor(), Ceil()
```

### String

```
Upper(), Lower(), Len()
```

### Database (DBF example)

```clipper
db := DB.open("customer.dbf")
rec := db.seek("CRUZ")
```

---

# **13. Differences from Clipper 5.x**

| Clipper 5.x        | Clipper-2025                 |
| ------------------ | ---------------------------- |
| Commands           | No commands                  |
| Workareas          | Removed                      |
| Macro operator `&` | Removed (future safe macros) |
| DOS UI             | Removed                      |
| OBJ→EXE            | Removed                      |
| RDD                | Replaced by module DB engine |
| Codeblocks         | Improved                     |
| Functions          | Same spirit                  |
| Classes            | Coming later                 |
| .CH includes       | Macro system planned         |

---

# **14. Future Syntax Extensions**

* pattern matching
* better lambdas
* OOP
* async I/O
* pipelines
* optional static typing
* module-level constants
* actor-based concurrency

---

# **15. Sample Full Program**

```clipper
function main()
    local a := 10
    local b := 20
    local avg := Average(a, b, 30)

    Print("Average:", avg)
    Print("Double:", Double(avg))
    Print("Square of 5:", Square(5))
return nil

function Average(x, y, z)
    local sum := x + y + z
    return sum / 3

function Double(n)
    return n * 2

function Square(n)
    return n * n
```
