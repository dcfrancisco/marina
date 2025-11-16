# Macro Engine & `.CH` System

Clipper's original macro system (`#command`, `#translate`, `#include`, and `&var`) was both powerful and dangerous.

Clipper-2025 keeps the power, removes the danger, and evolves macros into a **safe, compiler-driven system**.

This is one of Marina's most important innovations.

## Overview

Clipper-2025 includes a **three-tier macro system**:

### 1) Textual Macros

(C-like replacements)

### 2) Pattern Macros

(Clipper-style `#command` / `#translate`)

### 3) AST Macros

(Modern, safe metaprogramming similar to Rust/Elixir)

This gives Marina:

* the expressive power of Clipper
* the safety and modernity of modern languages
* the ability to build DSLs and frameworks directly in Clipper

## Why Macros Matter

Clipper developers depended heavily on:

* `.ch` include files
* syntactic sugar like `?`, `??`, `@`, `@ SAY`
* DB query DSLs
* UI layout shortcuts
* domain-specific languages

Clipper-2025 reintroduces all of this with a **clean architecture**.

## Macro Layers Explained

### Tier 1 — Textual Macros

Simple replacements.

Example:

```clipper
#define MAX 100
```

Usage:

```clipper
local x := MAX
```

Expansion:

```clipper
local x := 100
```

This layer exists mainly for:

* constants
* simple legacy support
* shared definitions

### Tier 2 — Pattern Macros (Clipper-style)

Example:

```clipper
#command ? <expr> => Print(<expr>)
```

Transforms:

```clipper
? "Hello"
```

Into:

```clipper
Print("Hello")
```

These are used to recreate:

* `?` and `??`
* `@ row,col SAY`
* DBF shorthands
* legacy syntactic sugar
* optional compatibility layers

### Tier 3 — AST Macros

This is where Clipper-2025 surpasses Clipper 5, Harbour, and Xbase++.

AST Macros allow DSL-like patterns:

#### Example: Web routing DSL

```clipper
@route("/home") function home(req)
    Print("Hello!")
```

Expands to:

```clipper
function home(req)
    app.addRoute("/home", {|req| home(req)})
```

#### Example: ORM

```clipper
customer := FIND customer WHERE age > 21
```

Expands to:

```clipper
customer := orm.find("customer", {|rec| rec["age"] > 21 })
```

#### Example: GUI

```clipper
WINDOW "Main" WIDTH 400 HEIGHT 300
```

Expands to the GUI module's AST.

## `.CH` Files

A `.ch` file may contain:

* `#define` macros
* `#command` macros
* `#translate` macros
* AST macro definitions

### Example file `clipper.ch`:

```clipper
#command ? <x> => Print(<x>)
#command ?? <x> => PrintNoCR(<x>)
#command @ <r>,<c> SAY <expr> => tui.say(<r>, <c>, <expr>)
```

### Importing `.ch` in Clipper-2025:

```clipper
#include "clipper.ch"
```

## Macro Expansion Pipeline

Macro expansion happens **before parsing**, unless AST macro is used.

### Order:

1. Load textual `#define`
2. Expand pattern macros (`#command`, `#translate`)
3. Parse source into AST
4. Apply AST macros
5. Compile to bytecode

This is identical to modern compilers like Rust.

## Safety Rules

Clipper-2025 *removes everything unsafe* from Clipper 5.x macro engine:

❌ No runtime `&macro`
❌ No string-eval
❌ No unpredictable runtime effects
❌ No ability to produce invalid syntax

Macros must:

* produce valid AST
* declare their output type
* not modify VM memory
* not execute runtime code

## Example Macro Modules

### Legacy Compatibility Module

`clipper_compat.ch`

```clipper
#command ? <expr> => Print(<expr>)
#command ?? <expr> => PrintNoCR(<expr>)
#command @ <r>,<c> SAY <e> => tui.say(<r>, <c>, <e>)
```

This allows someone to use Clipper-2025 *almost like Clipper 5* — without bringing in xBase.

### Syntax Sugar Module

`syntax.ch`

```clipper
#command let <var> := <expr> => local <var> := <expr>
```

### Database Macro Module

`dbx.ch`

```clipper
#command FIND <tbl> WHERE <expr> => dbx.find(<tbl>, {|rec| <expr>})
```

### GUI DSL Module

`gui.ch`

```clipper
#command WINDOW <name> => gui.createWindow(<name>)
```

## Why AST Macros Matter for the Future

This enables you to create:

* DSLs
* GUI frameworks
* ORM syntaxes
* Routing frameworks
* Template languages
* Task schedulers
* Reactive programming structures
* Mini-languages inside Clipper

Clipper grows beyond xBase — without losing clarity.

## Macro Vision Summary

Clipper-2025 macro system:

✔ preserves Clipper's soul
✔ supports `.ch` files
✔ supports legacy shortcuts
✔ enables future frameworks
✔ is completely safe
✔ is fully compiler-based
✔ prevents runtime chaos

Clipper finally has the macro system it deserved in 1995.
