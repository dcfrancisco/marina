# **Clipper-2025 Bytecode Specification**

*The portable instruction format executed by the Marina VM.*

This document defines:

* instruction set
* bytecode file structure (`.bc`)
* operand stack behavior
* call frames
* constants
* execution model
* future extensions

Bytecode is **stack-based**, compact, portable, and deterministic — designed to replace Clipper's OBJ+EXE architecture with a modern VM-executed format.

---

# 🌊 **1. Bytecode Philosophy**

Clipper-2025 bytecode aims to be:

### ✔ Simple

Small opcode set, easy to debug, easy to emit.

### ✔ Portable

Runs on macOS, Linux, Windows, WASM.

### ✔ Predictable

No JIT (yet), deterministic execution model.

### ✔ Embeddable

VM is a Rust library embeddable into other apps.

### ✔ Backwards-expandable

Instruction opcodes are versioned and forward-compatible.

---

# 🟦 **2. Execution Model**

The VM uses:

* **Operand Stack** — evaluates expressions
* **Call Stack** — one frame per function call
* **Instruction Pointer (IP)** — index into bytecode array
* **Frame Pointer (FP)** — base of local variables
* **Constant Pool** — literals
* **Function Table** — compiled function definitions

### Stack Example

For:

```
a + b * 2
```

Bytecode:

```
LOAD_LOCAL a
LOAD_LOCAL b
PUSH_CONST 2
MUL
ADD
```

---

# 🟥 **3. Opcode Reference**

Below is the "v1 core" instruction set.

---

## **A. Stack Operations**

| Opcode           | Operands | Description                     |
| ---------------- | -------- | ------------------------------- |
| `PUSH_CONST idx` | 1 byte   | Push literal from constant pool |
| `PUSH_NIL`       | none     | Push nil                        |
| `POP`            | none     | Discard top of stack            |
| `DUP`            | none     | Duplicate top stack value       |

---

## **B. Local Variable Access**

| Opcode            | Operands | Description                    |
| ----------------- | -------- | ------------------------------ |
| `LOAD_LOCAL idx`  | 1 byte   | Push local variable onto stack |
| `STORE_LOCAL idx` | 1 byte   | Pop stack and store into local |

Locals are resolved at compile time.

---

## **C. Arithmetic**

| Opcode | Description             |
| ------ | ----------------------- |
| `ADD`  | Pop 2 → push sum        |
| `SUB`  | Pop 2 → push difference |
| `MUL`  | Pop 2 → push product    |
| `DIV`  | Pop 2 → push quotient   |
| `MOD`  | Pop 2 → push modulo     |

All ops expect numeric types.

---

## **D. Comparison**

| Opcode | Description   |
| ------ | ------------- |
| `EQ`   | equal         |
| `NE`   | not equal     |
| `GT`   | greater       |
| `GE`   | greater-equal |
| `LT`   | less          |
| `LE`   | less-equal    |

---

## **E. Logical**

| Opcode | Description      |
| ------ | ---------------- |
| `AND`  | boolean and      |
| `OR`   | boolean or       |
| `NOT`  | boolean negation |

Short-circuit logic performed via jumps, not opcodes.

---

## **F. Control Flow**

| Opcode                | Operands | Description          |
| --------------------- | -------- | -------------------- |
| `JMP offset`          | 2 bytes  | Unconditional jump   |
| `JMP_IF_FALSE offset` | 2 bytes  | Jump if false (pop)  |
| `RETURN`              | none     | Return from function |

---

## **G. Function Calls**

| Opcode                 | Operands | Description                       |
| ---------------------- | -------- | --------------------------------- |
| `CALL idx argc`        | 2 bytes  | Call function from function table |
| `CALL_BUILTIN id argc` | 2 bytes  | Call builtin function             |

Arguments are pushed onto the stack in order.

### Example:

```
CALL 12 3
```

Calls function #12 with 3 arguments.

---

# 🟩 **4. Bytecode File Format (.bc)**

A `.bc` file contains:

```
Header
Constant Pool
Function Table
Bytecode Sequence(s)
Debug Info (optional)
```

---

## **A. Header**

| Field       | Size | Notes            |
| ----------- | ---- | ---------------- |
| Magic bytes | 4    | `CLBC`           |
| Version     | 1    | bytecode version |
| Flags       | 1    | reserved         |

Example:

```
43 4C 42 43 01 00
```

---

## **B. Constant Pool**

Var-length section containing:

```
[ count:u16 ]
[ constant ]
[ constant ]
...
```

Supported constant types:

* numbers (f64)
* strings (UTF-8)
* booleans
* nil

Format:

```
type:u8
value:varies
```

---

## **C. Function Table**

```
[ count:u16 ]
for each function:
    name_index:u16
    arg_count:u8
    local_count:u8
    bytecode_offset:u32
    bytecode_length:u32
```

---

## **D. Bytecode Sequences**

Sequential array of opcodes + operands.

---

## **E. Debug Info (optional)**

Stores:

* source map
* line/column mapping
* function names

Not required for execution.

---

# 🟪 **5. Sample Bytecode Dump**

Source:

```clipper
function add(a, b)
    return a + b
```

Constants:

```
0: "add"
```

Function table:

```
func #0
  name_index = 0
  args = 2
  locals = 0
  offset = 0
  length = 4
```

Bytecode:

```
LOAD_LOCAL 0
LOAD_LOCAL 1
ADD
RETURN
```

---

# 🟧 **6. Error Model**

Runtime errors:

* invalid opcode
* division by zero
* calling undefined function
* wrong argument count
* type errors
* stack overflow

Compiler errors:

* undefined identifiers
* unclosed blocks
* arity mismatch
* syntax errors

Files stop execution safely.

---

# 🟨 **7. Future Bytecode Extensions**

Planned:

### ✔ classes & objects

```
OBJ_NEW, GET_SLOT, SET_SLOT
```

### ✔ closures / nested functions

```
CLOSURE_NEW, LOAD_UPVALUE
```

### ✔ lists & dicts

```
LIST_NEW, MAP_NEW
```

### ✔ async

```
AWAIT, YIELD
```

### ✔ optimized JIT instructions

(peephole optimizer)
