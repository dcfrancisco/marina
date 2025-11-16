# Database Engine

Clipper-2025's database engine is designed around one philosophy:

> **DBF/CDX is supported because of history —
> but the engine must be modular, modern, and plug-and-play for SQL, NoSQL, and anything else.**

Clipper was loved because **DBF/CDX was simple, lightweight, embedded, and blazing fast**.
Clipper-2025 recreates that spirit — but without the messy RDD system or workareas.

## DBF Engine Overview

Clipper-2025's `dbf` module is a **modern, safe, fully encapsulated** DBF/CDX engine.

### Why DBF still matters:

* Simple embedded storage
* Zero server requirements
* Human-readable
* Lightning-fast sequential access
* Perfect for local / mobile / offline apps
* A modern Clipper implementation must support DBF — but cleanly.

### But unlike legacy Clipper:

❌ No workareas
❌ No `USE`, `APPEND`, `SKIP`, `REPLACE` commands
❌ No global shared state
❌ No RDD magic
❌ No "current alias"

Clipper-2025 replaces this with a modern API.

## DBF Opening & Cursors

Instead of workareas, Clipper-2025 uses **Cursors**:

```clipper
local db := dbf.open("customer.dbf")
```

### A `Cursor` object supports:

* navigation
* reading fields
* editing fields
* appending records
* deleting records
* updating indexes

### Example:

```clipper
db.goto(1)
Print(db.field("NAME"))
```

### Cursor Methods:

```
open(filename)
close()
goto(n)
next()
prior()
top()
bottom()
recno()
deleted()
append()
delete()
field(name)
fieldPut(name, value)
```

## DBF Field Types

Supported DBF types:

| Type | Description               |
| ---- | ------------------------- |
| C    | Character                 |
| N    | Numeric                   |
| L    | Logical                   |
| D    | Date                      |
| M    | Memo (DBT, FPT supported) |

Internally normalized to Marina VM types.

## Locking

Clipper-2025 supports:

### File-level locking

### Record-level locking

### Multi-process safety

Designed to allow:

* network DBF use
* cloud file syncing
* cross-platform compatibility

## Transactions

DBF does not support transactions natively.
But Marina DBF engine introduces:

### Soft transactions

Buffered changes before committing to disk.

```
db.begin()
db.fieldPut("BALANCE", db.field("BALANCE") - 100)
db.commit()
```

Or:

```
db.rollback()
```

## CDX Index Engine

One of the most powerful parts of Clipper was its **CDX structural indexes**.

Clipper-2025 includes a rewritten CDX engine:

### Features:

* structural CDX
* multiple tags
* top-down B-Tree
* compressed nodes
* partial key detection
* descending indexes
* filter indexes
* soft-seek
* nearest match
* automatic maintenance

### Example:

```clipper
db.index("cust.cdx", "NAME", "TAGNAME")
```

## Seek & Find

Similar API, but no commands:

```clipper
if db.seek("DANNY")
    Print("Found", db.field("NAME"))
endif
```

Soft-seek example:

```clipper
db.softSeek("DA")
```

## Replacing Workarea Commands with API Calls

Legacy commands:

```
USE CUSTOMER
SET ORDER TO TAGNAME
SEEK "DANNY"
REPLACE BALANCE WITH BALANCE+10
```

Clipper-2025:

```clipper
db := dbf.open("customer.dbf")
db.setOrder("TAGNAME")

if db.seek("DANNY")
    db.fieldPut("BALANCE", db.field("BALANCE") + 10)
endif
```

Same power.
Zero magic.
