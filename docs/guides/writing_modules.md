# **Marina Module System — Future Design Notes**

> **Phase 3 status:** This guide describes the target module architecture. The
> current compiler supports only the built-in namespaces listed in the [module
> and scope reference](../reference/modules.md). Filesystem discovery, native
> loading, Clipper-authored modules, types, and macros are future work.

*Making Clipper-2025 extensible, modern, and future-proof.*

The module system is the backbone of Marina's extensibility — replacing Clipper 5.x's limitations and allowing everything from database engines to math libraries to UI frameworks to be implemented cleanly *outside* the core language.

---

# 🌊 **1. Goals of the Module System**

### ✔ Language stays small

Modules provide extra functionality without polluting the core.

### ✔ Extensible

Users can write modules in Rust or Clipper-2025.

### ✔ Replace xBase RDD commands

Database functionality becomes just modules.

### ✔ Safe and predictable

Modules register symbols, types, and functions explicitly.

### ✔ Dynamically loadable

Future support for `.so/.dll/.dylib`-style native modules.

### ✔ Macro-capable

Modules can register macros later when the macro engine is enabled.

---

# 🟥 **2. Import Syntax**

Clipper-2025 code imports modules using:

```clipper
import "moduleName"
```

or multiple imports:

```clipper
import "dbx"
import "string"
import "math"
```

---

# 🟦 **3. How Modules Are Located**

The search order:

1. Project's local modules directory

   ```
   ./modules/moduleName/
   ```

2. Global Marina module store

   ```
   ~/.marina/modules/moduleName/
   ```

3. Built-in modules (compiled into VM)

4. Future: installed modules via `marina mod install`

---

# 🟩 **4. Module Structure**

A module contains:

```
module.yaml        # metadata
lib.rs             # Rust implementation
index.prg          # optional Clipper wrapper module
types/             # optional type definitions
macros/            # .ch files (future)
```

### Example `module.yaml`

```yaml
name: dbx
version: 1.0.0
language: rust
description: DBF/CDX database driver for Marina
exports:
  - open
  - close
  - seek
  - read
```

---

# 🟧 **5. What Modules Can Export**

Modules can export:

### ✔ Functions

Example:

```rust
fn open(path: String) -> DbHandle
```

In Clipper:

```clipper
db := dbx.open("customer.dbf")
```

### ✔ Types

Example:

* DBHandle
* Cursor
* List
* Future Object (for async)

### ✔ Constants

Example:

```
dbx.READONLY
dbx.WRITE
```

### ✔ Macros (future)

`{||}` expansions, sugar syntax, DSLs.

---

# 🟫 **6. Module Initialization**

When a module is imported:

1. It registers its functions in the global symbol table.
2. It registers types with the VM.
3. It optionally registers macros.
4. It loads its internal state (e.g. DB connections).
5. It becomes available in current execution context.

---

# 🟪 **7. Calling a Module Function**

### Example

```clipper
import "math"

result := math.pow(2, 5)
Print(result)
```

This compiles to a bytecode sequence:

```
LOAD_GLOBAL "math.pow"
PUSH_CONST 2
PUSH_CONST 5
CALL_BUILTIN pow 2
```

Marina VM dispatches to the module's Rust function.

---

# 🟦 **8. Database Modules (DBF/CDX, PostgreSQL, MongoDB)**

Modules provide a standardized API:

### Example (DBF/CDX)

```clipper
db := dbx.open("customer.dbf")
db.useIndex("customer.cdx")

rec := db.seek("CRUZ")
Print(rec["name"])
```

### Example (PostgreSQL)

```clipper
import "pg"

conn := pg.connect("postgres://...")
rows := conn.query("SELECT * FROM users WHERE id = 1")
```

### Example (MongoDB)

```clipper
import "mongo"

users := mongo.collection("users")
u := users.findOne({ "name": "Danny" })
```

---

# 🟥 **9. Native Modules (.dll / .so / .dylib)**

Future support:

```
import "native:opencv"
```

The VM dynamically loads the shared library and binds exported C/Rust functions.

---

# 🟨 **10. Pure Clipper Modules**

Example:

`string.prg`:

```clipper
function left(s, n)
    return SubStr(s, 1, n)
```

Imported via:

```clipper
import "string"
```

---

# 🟧 **11. Macro Modules (Clipper Legacy or DSLs)**

Modules can register macros at a later stage.

Examples:

### Clipper legacy:

```
#command USE <file> => dbx.open(<file>)
```

### Modern DSL example:

```
#command route <path> => app.addRoute(<path>)
```

---

# 🟩 **12. Error Handling in Modules**

If a module raises an error:

```
raise("Index damaged")
```

The VM catches it and displays:

```
[dbx] Runtime error: Index damaged
    at dbx:seek()
    at main:12
```

---

# 🟦 **13. Security Model**

Modules run sandboxed within the VM:

* No direct memory access
* No unauthorized file access
* No arbitrary system calls
* Permission-based future system for networking

---

# 🟫 **14. Future Extensions**

### ✔ Hot-reload modules

For REPL or long-running apps.

### ✔ Namespacing

```
import "math" as m
result := m.sqrt(9)
```

### ✔ Module versioning

```
import "dbx@1.2.0"
```

### ✔ Module signing

Security for distributed modules.
