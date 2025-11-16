# Module System

Legacy Clipper used:

* one global namespace
* no real module system
* `.obj` linking
* RDDs for databases
* PRG files compiled monolithically

Clipper-2025 replaces all that with a **clean, modern module system** inspired by:

* Python modules
* Rust crates
* Elixir applications
* Lua require() model

A module is simply:

* a folder
* containing `.prg` files
* plus optional native modules
* plus optional `.ch` macro files

Modules compile to bytecode, load dynamically, and expose functions.

## Module Anatomy

### Example module folder:

```
math/
  math.prg
  helpers.prg
  math.ch
  module.json
```

### module.json:

```json
{
  "name": "math",
  "version": "1.0.0",
  "exports": ["sin", "cos", "sqrt"]
}
```

### A module must contain:

* **name**
* **version**
* **exported functions**

## Importing Modules

Source code:

```clipper
import "math"

function main()
    Print(math.sqrt(25))
return nil
```

### Under the hood:

1. VM checks module registry
2. If not loaded → loads bytecode → initializes module
3. `math` namespace is exposed to code

## Module Namespace Rules

### Automatic namespacing:

```
math.sqrt()
math.pi
dbx.open()
string.upper()
```

### No global pollution.

### No collisions.

Each module protects its scope.

## Module Initialization

Each module may optionally define:

```clipper
function __init__()
    // initialization code
```

Executed once on first import.

Like Python's `__init__.py`.

## Built-in Modules

Built-in modules come bundled with the VM.

| Module   | Purpose                   |
| -------- | ------------------------- |
| `core`   | runtime essentials        |
| `string` | string utilities          |
| `math`   | math functions            |
| `system` | OS functions              |
| `tui`    | text UI engine (optional) |
| `dbx`    | database engine           |
| `io`     | file access               |

## Custom Runtime Modules

Developers can create:

* pure Clipper modules (.prg → .bc)
* Rust native modules
* C native modules
* database adapters
* macro modules
* DSL modules
* GUI frameworks

Example: A MongoDB module:

```
mongo/
  mongo.rs   ← native-binding
  mongo.prg  ← high-level API
  module.json
```

Your design intentionally allows **any future backend**.

## Module Export Rules

To export:

```clipper
export function add(a, b)
    return a + b
```

To hide:

```clipper
function secret()
```

Preventing old Clipper's global function collisions.

## The RDD Replacement

(DBF/CDX engine + future SQL engines)

Clipper's RDD (Replaceable Database Drivers) were:

* magical
* global state driven
* workarea-based
* full of hidden behavior

Clipper-2025 replaces RDDs with:

### Database Modules

```
dbf.open()
dbf.close()
dbf.index()
dbf.seek()
```

And:

```
postgres.query()
mongo.find()
sqlite.exec()
```

You can load any data layer.

## Module Dependency Resolution

Similar to Rust Cargo:

* each module declares dependencies in `module.json`
* compiler ensures they exist
* VM loads dependencies recursively
* versioning is supported (`>=1.2 <2.0`)

## Standard Library Growth

Standard modules will grow over time:

* crypto
* networking
* GUI
* async
* array/list functions
* HTTP client
* JSON serializer
* XML parser
* DBMS adapters
* ORM

Each is optional and modular.

## Why Modules Instead of RDD or DBASE WORKAREAS?

| Feature       | Clipper RDD | Clipper-2025 Module |
| ------------- | ----------- | ------------------- |
| Multiple DBs  | Hard        | Easy                |
| Networking    | No          | Yes                 |
| Indexing      | CDX-only    | Many options        |
| SQL           | No          | Yes                 |
| Mongo         | No          | Yes                 |
| Versioning    | No          | Yes                 |
| Encapsulation | Weak        | Strong              |
| Testability   | Poor        | Excellent           |

The module system is the absolute foundation for future expansion.
