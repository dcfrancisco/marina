# **Clipper-2025 DBF/CDX Engine Design**

*Modernizing Clipper's powerful database engine without xBase commands or workareas.*

This document details:

* DBF file handling
* CDX index engine
* cursor model
* data buffers
* API design
* locking model
* transaction support
* future enhancements

This engine replaces Clipper's **RDD system** with a clean, modern, module-based design.

---

# 🌊 **1. Background: Why DBF/CDX Still Matters**

Clipper's DBF/CDX engine was:

* fast
* predictable
* lightweight
* single-file indexes
* flexible multi-order indexes
* great for local databases
* excellent for embedded systems
* simple to deploy

Clipper-2025 preserves these strengths but removes:

* workareas
* command syntax
* RDD macros
* legacy preprocessor rules
* implicit relationships

DBF becomes a **first-class module**, not a baked-in command system.

---

# 🟥 **2. Goals of the New DBF/CDX Engine**

### ✔ 100% compatible with Clipper-format DBF/CDX

### ✔ zero xBase commands

### ✔ API-based data manipulation

### ✔ index-aware search

### ✔ stable, safe, predictable

### ✔ supports modern features (UTF-8, transactions, etc.)

### ✔ swappable backend (PostgreSQL, MongoDB, etc.)

---

# 🟦 **3. Database Module API (High-Level)**

### Opening a DBF

```clipper
db := dbx.open("customer.dbf")
```

### Selecting indexes

```clipper
db.useIndex("customer.cdx")
db.order("LASTNAME")
```

### Searching

```clipper
rec := db.seek("CRUZ")
```

### Reading fields

```clipper
name := rec["NAME"]
age  := rec["AGE"]
```

### Scanning

```clipper
while (db.next())
    Print(db.current()["NAME"])
end
```

### Updating (future)

```clipper
db.replace({ "AGE": 40 })
```

### Filtering (future)

```clipper
db.filter({|rec| rec["AGE"] > 21 })
```

All operations are expression-based, not command-based.

---

# 🟩 **4. DBF Engine Internals**

A DBF file is parsed into:

* **header**
* **field descriptors**
* **record data buffer**
* **deleted flags**
* **memo pointers (for DBT/FPT)**

### DBF Reader Flow

1. Open file
2. Read header
3. Read field definitions
4. Position cursor
5. Load/delete flags
6. Memory-map (optional optimization)
7. Provide record buffer to API

---

# 🟧 **5. Record Model**

DBF records are represented as a **map**:

```rust
HashMap<String, Value>
```

Or in Clipper:

```clipper
rec["NAME"]
```

Types supported:

* Character
* Numeric
* Date
* Logical
* Memo (future)
* Float
* Currency (future)

---

# 🟫 **6. CDX Index Engine**

The CDX engine is critical — Clipper's CDX index format is a high-performance B-Tree.

Clipper-2025 CDX engine supports:

* multiple index tags per CDX file
* descending/ascending
* custom expressions (future)
* filtering (future)
* soft-seek and full-seek behavior
* binary search fallback

### CDX Operations

#### Open CDX

```clipper
db.useIndex("customer.cdx")
```

#### Select order

```clipper
db.order("NAME")
```

#### Seek

```clipper
rec := db.seek("CRUZ")
```

#### Next

```clipper
db.next()
```

### Internal Steps

* Load CDX header
* Load root page of B-Tree
* Evaluate tag expression (future)
* Traverse nodes
* Position cursor
* Load matching DBF record

---

# 🟪 **7. Cursor Model**

A cursor encapsulates:

* DBF file handle
* CDX index handles
* current record position
* current index order
* record buffer
* filter block
* lock state

### Example (internal Rust struct)

```rust
struct Cursor {
    dbf: DbfFile,
    cdx: Option<CdxFile>,
    order: Option<Tag>,
    recno: u32,
    buffer: Record,
    filter: Option<ClipperBlock>,
}
```

---

# 🟨 **8. Locking & Concurrency**

Future modes:

### Shared lock

Multiple readers allowed.

### Exclusive lock

Single writer, blocks others.

### Atomic updates

```
db.transaction({
    db.replace({ "AGE": 40 })
})
```

### Optimistic concurrency (future)

Useful for network DB like PostgreSQL backend.

---

# 🟦 **9. Error Handling**

All DBF/CDX errors surface as VM exceptions:

```
[dbx] Index "NAME" tag not found
```

Examples:

* corrupt DBF
* corrupt CDX page
* record out of range
* type mismatch
* index expression error
* lock conflict

---

# 🟩 **10. Performance Strategy**

### ✔ Memory-mapped IO (where supported)

Speeds up navigation.

### ✔ B-Tree caching

Cache:

* root pages
* hot nodes
* last-used index pages

### ✔ Record buffer reuse

Prevent constant allocations.

### ✔ Pipelined prefetch (future)

Async DBF reading.

### ✔ Index pre-evaluation (future)

Clipper used a "codeblock compiled to p-code" inside CDX tags.
Clipper-2025 will use Marina bytecode internally!

---

# 🟧 **11. Differences From Clipper 5.x**

| Clipper 5.x               | Clipper-2025                 |
| ------------------------- | ---------------------------- |
| Commands                  | API functions                |
| Workareas                 | Cursors                      |
| SET ORDER                 | db.order()                   |
| SEEK                      | db.seek()                    |
| Field access: FIELD->NAME | rec["NAME"]                  |
| RDD-based                 | Module-based                 |
| Locking via commands      | Locking via API              |
| QBASIC-like scripting     | Expression-first programming |

---

# 🟫 **12. Future Enhancements (Clipper Dreams Completed)**

These were planned by CA but never implemented — and now **Marina will finish them**:

### ✔ SQL-based RDD

(Clipper's "SQLRDD" project died — Marina revives it.)

### ✔ CDX tag expressions as bytecode

No more "macro-compiled" string expressions.

### ✔ multi-engine indexing

DBF and SQL indexes behave similarly.

### ✔ remote cursor support

Clipper planned "NetRDD," you will implement modern equivalent:

* PostgreSQL streaming cursor
* MongoDB cursor
* REST-backed cursor

### ✔ Multi-threaded DB engine

Clipper never achieved this.
