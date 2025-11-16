# Roadmap (2025–2027)

Clipper-2025 (Marina) is not a toy experiment.
It is a **multi-year language renaissance**, paced to bring Clipper into its rightful modern position.

## Versioning Philosophy

Marina uses **semantic versioning**:

```
MAJOR.MINOR.PATCH
```

* MAJOR: breaking language or VM changes
* MINOR: new features, modules, or improvements
* PATCH: bug fixes

No unstable nightly builds are expected — the project progresses cleanly.

## Phase Summary

| Phase    | Goal                                  | Target    | Status      |
| -------- | ------------------------------------- | --------- | ----------- |
| Phase 1  | Core language + VM                    | 2025      | In Progress |
| Phase 2  | Arrays, lists, maps                   | 2025      | Planned     |
| Phase 3  | Tooling (LSP, DAP, formatter)         | 2025-2026 | In Progress |
| Phase 4  | DBF/CDX engine                        | 2026      | Planned     |
| Phase 5  | Standard library expansion            | 2026      | Planned     |
| Phase 6  | Macro system (.ch files)              | 2026      | Planned     |
| Phase 7  | SQL engines (Postgres, SQLite)        | 2026-2027 | Planned     |
| Phase 8  | Native OOP (classes, methods)         | 2027      | Planned     |
| Phase 9  | NoSQL engines (MongoDB, Redis)        | 2027      | Planned     |
| Phase 10 | Async/await & concurrency             | 2027-2028 | Future      |
| Phase 11 | Cross-platform GUI framework          | 2028      | Future      |
| Phase 12 | Package ecosystem & registry          | 2028+     | Future      |
| Phase 13 | JIT compiler & optimizations          | 2029+     | Future      |
| Phase 14 | WASM & embedded targets               | 2029+     | Future      |

## PHASE 1 — Core Language + VM (2025)

**Status: IN PROGRESS** ✅ 80% Complete

### Deliverables:

* ✅ Lexer with full tokenization
* ✅ Recursive descent parser
* ✅ AST representation
* ✅ Bytecode compiler
* ✅ Stack-based VM
* ✅ Basic operators (arithmetic, comparison, logical)
* ✅ Functions with parameters
* ✅ Local variables & scoping
* ✅ Control flow (if/else, while, case)
* ✅ Codeblocks `{||}`
* ✅ Built-in functions (Print, TypeOf, etc.)
* ✅ CLI compiler/interpreter
* ✅ REPL mode
* ⏳ Error recovery improvements
* ⏳ Bytecode serialization (.bc files)

### Milestone Example:

```clipper
function factorial(n)
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)
    endif

function main()
    Print("5! =", factorial(5))  // 120
return nil
```

### Current Status:
All core language features working. 62 integration tests passing. Focus now on polish and tooling.

---

## PHASE 2 — Arrays, Lists, Maps (2025)

**Status: PLANNED** 🎯 Next Priority

### Deliverables:

* Array literals `[1, 2, 3]`
* Array indexing `arr[1]`
* Array methods (push, pop, length, slice)
* List comprehensions (future)
* Hash maps/dictionaries `{"key": value}`
* Map methods (keys, values, has, delete)
* Iteration (for/in loops)
* Nested structures

### Milestone Example:

```clipper
function main()
    local numbers := [1, 2, 3, 4, 5]
    local doubled := Map(numbers, {|n| n * 2})
    
    local person := {
        "name": "Danny",
        "age": 30,
        "email": "danny@example.com"
    }
    
    Print(person["name"])
    Print(doubled[3])  // 6
return nil
```

### Why Priority:
Arrays/maps are essential for real applications. Needed before DBF engine (for result sets).

---

## PHASE 3 — Tooling & Developer Experience (2025-2026)

**Status: IN PROGRESS** ⚙️ 40% Complete

### Deliverables:

* ✅ marina-lsp (basic diagnostics, completion)
* ⏳ LSP: go-to-definition
* ⏳ LSP: workspace symbols
* ⏳ LSP: rename refactoring
* ✅ marina-fmt (syntax validation)
* ⏳ Formatter: actual code formatting
* ⏳ marina-dap (debugger protocol)
* ⏳ DAP: breakpoints
* ⏳ DAP: step execution
* ⏳ DAP: variable inspection
* ⏳ VS Code extension
* ⏳ Syntax highlighting definitions

### Milestone:
Full VS Code integration with debugging, formatting, IntelliSense.

### Why Priority:
Developer experience determines adoption. Good tooling = more users.

---

## PHASE 4 — DBF/CDX Database Engine (2026)

**Status: PLANNED**

### Deliverables:

* DBF file format support (dBASE III+, IV, 5)
* Field types (C, N, L, D, M)
* DBT/FPT memo fields
* Cursor-based API (no workareas!)
* CDX structural indexes
* Multiple index tags
* Index operations (seek, softseek, rebuild)
* Record locking (file and record level)
* Soft transactions (begin/commit/rollback)
* Filter/scope support
* Batch operations

### Milestone Example:

```clipper
function main()
    local db := dbf.open("customer.dbf")
    db.index("customer.cdx", "upper(name)", "NAME")
    db.setOrder("NAME")
    
    if db.seek("SMITH")
        Print("Found:", db.field("name"))
        Print("Email:", db.field("email"))
        
        db.fieldPut("lastContact", Date())
        db.save()
    endif
    
    db.close()
return nil
```

### Design Principles:
- No global state (each cursor is independent)
- Explicit operations (no magic)
- Modern error handling
- Thread-safe (future)

---

## PHASE 5 — Standard Library Expansion (2026)

**Status: PLANNED**

### Core Modules to Build:

**string module:**
```clipper
import "string"
string.split("a,b,c", ",")
string.join(["a", "b"], "-")
string.replace("hello", "l", "r")
string.padLeft("5", 3, "0")  // "005"
```

**math module:**
```clipper
import "math"
math.max([1, 5, 3])
math.min([1, 5, 3])
math.random(1, 100)
math.floor(3.7)
math.ceil(3.2)
```

**file module:**
```clipper
import "file"
content := file.read("data.txt")
file.write("output.txt", content)
lines := file.readLines("data.csv")
file.exists("config.json")
```

**date module:**
```clipper
import "date"
now := date.now()
formatted := date.format(now, "YYYY-MM-DD")
diff := date.diff(date1, date2, "days")
```

**json module:**
```clipper
import "json"
obj := json.parse('{"name": "Danny"}')
str := json.stringify(obj)
```

**http module (future):**
```clipper
import "http"
response := http.get("https://api.example.com/users")
data := json.parse(response.body)
```

---

## PHASE 6 — Macro System & .CH Files (2026)

**Status: PLANNED**

### Three-Tier System:

**Tier 1: Textual Macros**
```clipper
#define MAX_USERS 100
#define DEBUG true
```

**Tier 2: Pattern Macros**
```clipper
#command ? <expr> => Print(<expr>)
#command ?? <expr> => PrintNoCR(<expr>)
```

**Tier 3: AST Macros**
```clipper
// Define custom syntax transformations
@validate(nonEmpty)
function setName(name)
    this.name := name
end

// Expands to:
function setName(name)
    if Length(name) == 0
        throw("Name cannot be empty")
    endif
    this.name := name
end
```

### Safety Rules:
- Compile-time only (no runtime `&macro`)
- Must produce valid AST
- Scoped to module
- Explicit imports

### Why Important:
Enables DSLs, legacy compatibility layer, framework development without compromising safety.

---

## PHASE 7 — SQL Database Engines (2026-2027)

**Status: PLANNED**

### Deliverables:

**PostgreSQL Module:**
```clipper
import "postgres"

pg := postgres.connect("postgresql://user:pass@localhost/mydb")
result := pg.query("SELECT * FROM users WHERE age > $1", [21])

for row in result
    Print(row["name"], row["email"])
next

pg.execute("INSERT INTO users (name, email) VALUES ($1, $2)", 
           ["Danny", "danny@example.com"])
pg.commit()
```

**SQLite Module:**
```clipper
import "sqlite"

db := sqlite.open("app.db")
db.execute("""
    CREATE TABLE IF NOT EXISTS customers (
        id INTEGER PRIMARY KEY,
        name TEXT,
        email TEXT
    )
""")

db.insert("customers", {"name": "Alice", "email": "alice@example.com"})
rows := db.query("SELECT * FROM customers")
```

**Unified Database API:**
```clipper
// Abstract over DBF, SQLite, Postgres
local db := database.connect("postgres://...")  // or "dbf://..." or "sqlite://..."

db.insert("users", data)
db.update("users", {"name": "New Name"}, {"id": 123})
rows := db.select("users", {"age >": 21})
```

### Why Priority:
Modern apps need SQL. Marina must be enterprise-ready.

---

## PHASE 8 — Native Object-Oriented Programming (2027)

**Status: PLANNED**

The OOP Clipper should have had in 1997 - done right.

### Deliverables:

**Class Definition:**
```clipper
class Customer
    // Properties (private by default)
    private name
    private email
    private balance
    
    // Constructor
    method init(name, email)
        this.name := name
        this.email := email
        this.balance := 0.0
    end
    
    // Public methods
    method getName()
        return this.name
    end
    
    method addFunds(amount)
        this.balance += amount
    end
    
    method getBalance()
        return this.balance
    end
end

// Usage
local customer := Customer("Danny", "danny@example.com")
customer.addFunds(100.00)
Print("Balance:", customer.getBalance())
```

**Inheritance:**
```clipper
class PremiumCustomer extends Customer
    private discount
    
    method init(name, email, discount)
        super.init(name, email)
        this.discount := discount
    end
    
    method getDiscount()
        return this.discount
    end
end
```

**Interfaces/Traits (future):**
```clipper
interface Serializable
    method toJson()
    method fromJson(data)
end

class Customer implements Serializable
    method toJson()
        return json.stringify({
            "name": this.name,
            "email": this.email
        })
    end
end
```

**Codeblocks as Methods:**
```clipper
class Calculator
    method process(data, operation)
        return Map(data, operation)
    end
end

calc := Calculator()
result := calc.process([1, 2, 3], {|n| n * 2})
```

### Design Principles:
- Simple, not over-engineered (unlike VO)
- No metaclass hacks (unlike Class(y))
- Integrates with VM natively
- Optional (procedural still valid)
- Module-scoped classes

---

## PHASE 9 — NoSQL Database Engines (2027)

**Status: PLANNED**

### MongoDB Module:

```clipper
import "mongodb"

mongo := mongodb.connect("mongodb://localhost:27017/myapp")
customers := mongo.collection("customers")

// Insert
customers.insertOne({
    "name": "Danny",
    "age": 30,
    "tags": ["premium", "verified"]
})

// Query
results := customers.find({"age": {"$gt": 21}})
for doc in results
    Print(doc["name"])
next

// Update
customers.updateOne(
    {"name": "Danny"},
    {"$set": {"age": 31}}
)

// Aggregation pipeline
pipeline := [
    {"$match": {"age": {"$gt": 25}}},
    {"$group": {"_id": "$city", "count": {"$sum": 1}}}
]
results := customers.aggregate(pipeline)
```

### Redis Module:

```clipper
import "redis"

r := redis.connect("redis://localhost:6379")

// Key-value
r.set("user:123", "Danny")
name := r.get("user:123")

// Lists
r.rpush("queue", "job1")
job := r.lpop("queue")

// Hashes
r.hset("user:123", "name", "Danny")
r.hset("user:123", "email", "danny@example.com")
data := r.hgetall("user:123")

// Pub/Sub
r.publish("notifications", "New message!")
```

### Why NoSQL:
Modern data models (documents, graphs, caching) require NoSQL. Marina should excel at data manipulation regardless of backend.

---

## PHASE 10 — Async/Await & Concurrency (2027-2028)

**Status: FUTURE**

### Async Functions:

```clipper
async function fetchUser(id)
    local response := await http.get("https://api.example.com/users/" + id)
    return json.parse(response.body)
end

async function main()
    local user := await fetchUser(123)
    Print("User:", user["name"])
return nil
```

### Parallel Execution:

```clipper
// Spawn concurrent tasks
local t1 := spawn {|| heavyComputation1() }
local t2 := spawn {|| heavyComputation2() }

// Wait for completion
local result1 := await t1
local result2 := await t2
```

### Actor Model:

```clipper
actor Worker
    method process(data)
        // Runs in isolated thread
        return transform(data)
    end
end

worker := Worker.spawn()
result := await worker.send("process", myData)
```

### Channels:

```clipper
channel := Channel.create()

spawn {||
    for i := 1 to 10
        channel.send(i)
    next
    channel.close()
}

while channel.isOpen()
    value := channel.receive()
    Print("Received:", value)
endwhile
```

---

## PHASE 11 — Cross-Platform GUI Framework (2028)

**Status: FUTURE**

Modern VO - done right.

### Declarative GUI DSL:

```clipper
import "gui"

window "Customer Manager"
    title "Marina CRM"
    size 1024, 768
    
    menubar
        menu "File"
            item "New Customer" shortcut "Ctrl+N" action {|| newCustomer() }
            item "Open..." shortcut "Ctrl+O" action {|| openFile() }
            separator
            item "Exit" action {|| app.quit() }
        end
        
        menu "Edit"
            item "Cut" shortcut "Ctrl+X" action {|| edit.cut() }
            item "Copy" shortcut "Ctrl+C" action {|| edit.copy() }
            item "Paste" shortcut "Ctrl+V" action {|| edit.paste() }
        end
    end
    
    layout vertical
        toolbar
            button "New" icon "new.png" action {|| newCustomer() }
            button "Save" icon "save.png" action {|| saveCustomer() }
            separator
            button "Delete" icon "delete.png" action {|| deleteCustomer() }
        end
        
        splitview horizontal ratio 0.3
            // Left panel - customer list
            table id "customerList" source customersDb
                column "Name" field "name" width 200
                column "Email" field "email" width 250
                column "Phone" field "phone" width 150
                
                onSelect {|row| displayCustomer(row) }
                onDoubleClick {|row| editCustomer(row) }
            end
            
            // Right panel - customer detail
            form id "customerDetail"
                field "Name" binding "customer.name" required
                field "Email" binding "customer.email" type "email"
                field "Phone" binding "customer.phone"
                field "Notes" binding "customer.notes" type "multiline"
                
                buttons
                    button "Save" action {|| saveCustomer() }
                    button "Cancel" action {|| cancelEdit() }
                end
            end
        end
        
        statusbar
            label id "status" text "Ready"
            label id "recordCount" text "0 customers"
        end
    end
end
```

### Backend Support:
- **Native** - Platform widgets (GTK on Linux, Cocoa on macOS, Win32 on Windows)
- **WebView** - Embedded browser with HTML/CSS
- **Qt** - Cross-platform Qt bindings
- **Immediate Mode** - Dear ImGui for tools/editors

### Reactive Updates:
```clipper
// Data binding - UI updates automatically
local customer := reactive({
    "name": "Danny",
    "email": "danny@example.com"
})

// When customer changes, bound UI updates
customer.name := "Daniel"  // UI field updates automatically
```

### Themes:
```clipper
gui.setTheme("dark")
gui.setTheme("light")
gui.setTheme("custom", {
    "backgroundColor": "#1e1e1e",
    "textColor": "#ffffff",
    "accentColor": "#007acc"
})
```

---

## PHASE 12 — Package Ecosystem & Registry (2028+)

**Status: FUTURE**

### Package Manager:

```bash
# Install packages
marina pkg install http
marina pkg install json
marina pkg install postgres

# Publish packages
marina pkg publish mylib

# Search
marina pkg search database
```

### Package Definition:

```toml
# marina.toml
[package]
name = "myapp"
version = "1.0.0"
authors = ["Danny <danny@example.com>"]
license = "MIT"

[dependencies]
http = "^2.0"
json = "^1.5"
postgres = "^0.9"

[dev-dependencies]
test-framework = "^1.0"
```

### The "Dockyard" Registry:
Central package repository (like npm, crates.io, PyPI)

---

## PHASE 13 — JIT Compiler & Optimizations (2029+)

**Status: FUTURE**

### Just-In-Time Compilation:
- Hot path detection
- Inline expansion
- Constant folding
- Dead code elimination
- Register allocation
- Native code generation

### Performance Goals:
- Interpreted: 0.5x Python speed
- JIT: 2-5x Python speed
- Future AOT: Near C speed

---

## PHASE 14 — WASM & Embedded Targets (2029+)

**Status: FUTURE**

### WebAssembly:
```bash
# Compile to WASM
marina build --target wasm32 app.prg

# Run in browser
<script src="marina-runtime.js"></script>
<script>
  Marina.loadModule('app.wasm').then(app => {
    app.main();
  });
</script>
```

### Embedded Targets:
- Raspberry Pi
- ESP32 microcontrollers
- Mobile (iOS/Android via WASM or native)
- Game engines (Godot, Unity plugins)

---

## Long-Term Vision (2030 and beyond)

### AI Integration
- LLM bindings (OpenAI, Anthropic, local models)
- Vector database support
- Embeddings API
- RAG (Retrieval Augmented Generation) helpers

### Game Development
- Game engine embedding
- Scripting for Godot/Unity
- ECS (Entity Component System) framework
- Hot reload for rapid iteration

### Cloud Native
- Container-first deployment
- Kubernetes operators
- Serverless functions
- Distributed tracing

### The 30-Year Language

Marina is designed with longevity in mind:
- Clean core (small, stable)
- Module-based extension (grow without bloat)
- Backward compatibility (within major versions)
- Forward thinking (room for future paradigms)

---

## Success Metrics

### Phase 1-3 (2025-2026): Foundation
- ✅ Language works
- ✅ Developer tools exist
- ✅ Early adopters build apps

### Phase 4-6 (2026-2027): Adoption
- ✅ Real applications in production
- ✅ Community contributes packages
- ✅ Marina chosen for new projects

### Phase 7-9 (2027-2028): Maturity
- ✅ Enterprise adoption
- ✅ Teaching in universities
- ✅ Multiple database backends used
- ✅ GUI apps shipped

### Phase 10+ (2028+): Ecosystem
- ✅ Package ecosystem thrives
- ✅ Marina used across industries
- ✅ New paradigms integrated
- ✅ Next generation discovers Marina

The language is designed to last 30–50 years.

---

## What Makes Marina Different From Xbase++

| Feature        | Xbase++         | Marina               |
| -------------- | --------------- | -------------------- |
| xBase commands | Yes             | No                   |
| Workareas      | Yes             | No                   |
| RDD            | Yes             | Modular engines      |
| VM             | Partial         | Yes                  |
| Bytecode       | Partial         | Yes                  |
| Macro system   | Same as Clipper | Modern, safe         |
| Syntax         | xBase + OOP     | Clipper-style modern |
| Memory model   | Proprietary     | Rust safe            |
| SQL            | Optional        | Full                 |
| NoSQL          | No              | Yes                  |
| Async          | No              | Yes                  |
| Future-proof   | No              | Yes                  |

## The Big Promise

> **Clipper-2025 is not nostalgia.
> It is a second life for a language that was ahead of its time —
> rebuilt with modern engineering —
> and aimed at the next 30 years of computing.**
