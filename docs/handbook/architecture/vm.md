# Marina Virtual Machine

The Marina VM is the beating heart of Clipper-2025.
It is what replaces the old PRG → OBJ → EXE workflow and brings Clipper into the modern era with portable bytecode.

## Execution Model

Marina uses a **stack-based virtual machine**, inspired by:

* CPython
* Lua VM
* Java VM (but simpler)
* Clipper's internal eval engine (conceptually)

Each instruction operates on a common operand stack, with explicit call frames and a constant pool.

### The VM state contains:

* **Instruction Pointer (IP)**
* **Operand Stack**
* **Call Stack (Frames)**
* **Constant Pool**
* **Function Table**
* **Global Symbol Table** (future)
* **Module Registry**

The VM executes bytecode sequentially unless a jump instruction changes control flow.

## Operand Stack

The operand stack is the core of all computation.

### Example:

Bytecode:

```
LOAD_LOCAL 0
LOAD_LOCAL 1
ADD
```

Stack trace:

```
push a
push b
pop b, pop a → push (a+b)
```

### Why a stack machine?

* Easy to generate code
* Simple to interpret
* More compact bytecode
* Portable across architectures
* Matches Clipper's lightweight philosophy

## Call Frames

Every function call creates a new **Frame**.

Frame contains:

* base pointer for locals
* number of locals
* slot for parameters
* IP return address
* reference to enclosing function
* reference to module context

### Example structure (Rust):

```rust
struct Frame {
    locals: Vec<Value>,
    return_ip: usize,
    function_id: usize,
}
```

## Calling Convention

### Arguments pushed left-to-right:

Source:

```clipper
add(a, b)
```

Order on stack:

```
push a
push b
CALL add
```

### VM responsibilities:

* pop args
* create frame
* set local 0..N with args
* jump to function bytecode
* push return value

## Memory Model

Marina VM uses:

### Tagged values (dynamic types)

Each value is an enum:

```rust
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Block(Block),
    List(Vec<Value>),    // future
    Map(HashMap),       // future
    Object(Object),     // future OOP
}
```

### No garbage collector yet.

Memory is:

* reference counted, OR
* freed at end of frame (primitives)

Future versions will implement:

* cycle detection
* generational GC

## Global Namespace (Future)

Modules and globals register symbols into:

```
GLOBAL["math.sin"]
GLOBAL["string.upper"]
GLOBAL["dbx.open"]
```

VM will dispatch based on:

* builtin table
* module registry
* Clipper-defined functions

## Exception Model

Errors throw VM exceptions:

* division by zero
* calling undefined function
* invalid opcode
* type mismatch
* stack underflow
* index error
* DBF/CDX engine errors

### Example:

```
Runtime Error: Division by zero
  at function divide (line 4)
  at function main (line 1)
```

## Debugger Hooks (Future)

VM will support:

* breakpoints
* stepping
* inspection of locals
* viewing stack
* viewing bytecode

These require:

* symbol table
* debug info section in `.bc`
* VM hooks

## VM Advantages Over Legacy Clipper

| Legacy Clipper  | Marina VM              |
| --------------- | ---------------------- |
| PRG → OBJ → EXE | PRG → BC → RUN         |
| DOS executable  | Cross-platform         |
| Static linking  | Dynamic module loading |
| Limited memory  | Unlimited memory model |
| xBase commands  | API-based DB engine    |
| No debugging    | Future debugger        |
| No JIT          | JIT optional           |
| No async        | async/await planned    |
| No threads      | green threads planned  |

## VM Extensibility Roadmap

Future VM capabilities:

* JIT compiler
* Bytecode optimizer
* Parallel execution
* Actor model
* Fiber scheduler
* Async I/O
* Module sandboxing
* WASM backend

Marina VM is designed to grow for *decades*.
