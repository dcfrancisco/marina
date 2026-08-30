# **Clipper-2025 Macro Engine — Future Design Notes**

> Macro preprocessing and `.ch` compatibility are deferred and are not
> implemented in the current runtime.

*The future macro system that preserves Clipper's power while avoiding its historical flaws.*

Clipper-2025 intentionally **does NOT include macros in v0.1**.
The macro system comes later — clean, safe, AST-level — to support:

* `.ch` include files
* syntactic sugar
* legacy compatibility (optional)
* domain-specific features
* code-generation
* embedded DSLs

This document describes how macros **will** work.

---

# 🌊 **1. Philosophy**

Clipper's old macro operator `&var` was:

❌ dangerous
❌ slow
❌ string-based
❌ runtime-evaluated
❌ unoptimizable
❌ source-dependent
❌ error-prone

But `.ch` include files and `#command` macros were extremely powerful.

Clipper-2025 reimplements macros **the right way**, with three layers:

### 1️⃣ **Textual Macros** (like C's preprocessor)

### 2️⃣ **Pattern Macros** (Clipper-style `#command`)

### 3️⃣ **AST Macros** (Rust/Elixir-level power)

---

# 🟥 **2. Macro Levels**

---

## **2.1 Textual Macros**

These are simple replacements.

### Example:

```clipper
#define MAXROWS 100
```

In code:

```
rows := MAXROWS
```

After macro expansion:

```
rows := 100
```

Uses:

* constants
* small definitions
* compatibility keywords (optional)

---

## **2.2 Pattern Macros (#command, #translate)**

Inspired by Clipper:

```clipper
#command ? <x> => Print(<x>)
```

Allows lightweight syntax sugar.

### Example:

Source:

```clipper
? "Hello"
```

Expanded:

```clipper
Print("Hello")
```

---

## **2.3 AST Macros (Marina-exclusive)**

The most powerful feature — safe code rewriting at AST level.

### Example:

A module could define:

```clipper
@route("/home") function home() ...
```

Which expands to:

```clipper
app.addRoute("/home", {|req| home(req)})
```

### Capabilities:

* rewrite expressions
* add new syntactic constructs
* implement DSLs
* metaprogramming
* build frameworks

This level avoids all weaknesses of Clipper's old macro engine.

---

# 🟦 **3. `.ch` Include Files**

Modules may provide `.ch` files such as:

### `clipper.ch` (Legacy compatibility)

Provides:

* `#command` for `?`, `??`, `@ SAY`
* optional syntactic sugar for classic code
* aliases for Clipper functions

### `dbf.ch` (Database shorthand)

Example:

```clipper
#command USE <f> => db := dbx.open(<f>)
```

### `web.ch` (Web DSL)

Example:

```clipper
#command ROUTE <path> => app.route(<path>)
```

### `gui.ch` (Future GUI)

Example:

```clipper
#command WINDOW <title> => gui.createWindow(<title>)
```

This allows Clipper-2025 to adopt new syntax without modifying the core.

---

# 🟩 **4. Macro Resolution Order**

Macro expansion happens:

1. **Load `#define` macros**
2. **Apply pattern macros**
3. **Transform AST via macro modules**
4. **Compile to bytecode**

Macros do NOT run at runtime — only during compilation.

---

# 🟧 **5. Safety Rules**

Unlike Clipper 5.x, macros:

* cannot break the parser
* generate ASTs that are syntactically valid
* cannot execute arbitrary runtime code
* cannot read unknown files
* cannot modify VM memory
* must declare all outputs

This prevents "macro hell."

---

# 🟫 **6. Example Macro Modules**

---

## **6.1 Legacy Compatibility Module**

`clipper_compat.ch`:

```clipper
#command ? <expr> => Print(<expr>)
#command ?? <expr> => PrintNoCR(<expr>)
#command @ <row>,<col> SAY <expr> => tui.say(<row>, <col>, <expr>)
```

Usage:

```clipper
import "compat"
#include "clipper_compat.ch"
```

---

## **6.2 Modern Syntax Sugar Module**

`syntax.ch`:

```clipper
#command let <var> := <expr> => local <var> := <expr>
#command fn <name>(<args,...>) => function <name>(<args>)
```

Transforms:

```clipper
fn greet(name)
    Print("Hello", name)
```

Into:

```clipper
function greet(name)
    Print("Hello", name)
```

---

## **6.3 Database DSL**

`orm.ch`:

```clipper
#command FIND <table> WHERE <expr> => orm.find(<table>, {|rec| <expr> })
```

Usage:

```clipper
rec := FIND customer WHERE rec["AGE"] > 21
```

---

# 🟪 **7. Macro Engine Internals (Compiler Integration)**

The compiler pipeline becomes:

```
Tokens
  ↓ (textual macros)
Tokens
  ↓ (pattern macros)
Transformed source
  ↓ (parser)
AST
  ↓ (AST macro expansion)
Transformed AST
  ↓ (compiler)
Bytecode
```

---

# 🟨 **8. Future Macro Features**

* Macro hygiene (prevent name collisions)
* Compile-time evaluation for constants
* Procedural macros
* Macro-based code generation
* Documentation macros
* GUI syntax extensions
* Web framework DSLs
* Query builders (DBMS-neutral)

---

# 🎉 **Conclusion**

The Marina macro system brings back Clipper's extensibility while ensuring:

✔ Safety
✔ Performance
✔ Clarity
✔ Modularity
✔ Modern power

This is the macro system Clipper should have had.
