# Programming Language Features Checklist — Marina (Clipper-2025)

**Last Updated:** 2025-11-19

This document maps essential features of a great modern programming language to Marina's implementation roadmap.

---

## 1. Core Language Features

### 1.1 Data Types & Literals
- [x] **Integers & Floats** - Numbers (f64) ✅ Phase 1
- [x] **Strings** - String literals with quotes ✅ Phase 1
- [x] **Booleans** - TRUE/FALSE ✅ Phase 1
- [x] **Null/Nil** - NIL keyword ✅ Phase 1
- [x] **Arrays** - `{1, 2, 3}` syntax ✅ Phase 2
- [ ] **Lists** - Ordered collections (dynamic) → Phase 2 (PRIORITY)
- [ ] **Maps/Hashes** - `{"key" => "value"}` → Phase 2 (PRIORITY)
- [ ] **Tuples** - Immutable fixed-size collections → Phase 5
- [ ] **Sets** - Unique value collections → Phase 5
- [ ] **Byte Arrays/Buffers** - Binary data → Phase 5
- [ ] **Decimals** - Precise arithmetic for finance → Phase 5
- [ ] **BigInt** - Arbitrary precision integers → Phase 5
- [ ] **Symbols** - Interned strings → Phase 6

### 1.2 Operators
- [x] **Arithmetic** - `+, -, *, /, %, ^` ✅ Phase 1
- [x] **Comparison** - `=, !=, <, >, <=, >=` ✅ Phase 1
- [x] **Logical** - `AND, OR, NOT` ✅ Phase 1
- [x] **Assignment** - `:=` and `=` ✅ Phase 1
- [x] **Compound Assignment** - `+=, -=, *=, /=` ✅ Phase 2
- [x] **Increment/Decrement** - `++, --` ✅ Phase 2
- [ ] **Bitwise** - `&, |, ~, <<, >>` → Phase 5
- [ ] **Ternary** - `condition ? true_val : false_val` → Phase 5
- [ ] **Null Coalescing** - `??` → Phase 5
- [ ] **Optional Chaining** - `?.` → Phase 5
- [ ] **Pipeline** - `|>` → Phase 6
- [ ] **Range** - `1..10, 1..=10` → Phase 2
- [ ] **Spread/Splat** - `...array` → Phase 5

### 1.3 Control Flow
- [x] **If/Else** - Conditional branching ✅ Phase 1
- [ ] **ElseIf** - Multi-way conditionals → Phase 2
- [x] **Case/Switch** - Pattern matching ✅ Phase 2
- [x] **While Loops** - Pre-test loops ✅ Phase 1
- [x] **Do-While Loops** - Post-test loops ✅ Phase 1
- [x] **For Loops** - Counted iteration ✅ Phase 1
- [x] **Loop/Break** - Infinite loops with exit ✅ Phase 2
- [ ] **For-Each** - Iterator loops → Phase 2 (PRIORITY)
- [ ] **Continue** - Skip iteration → Phase 2
- [ ] **Return** - Early function exit ✅ Phase 1 (needs fixing)
- [ ] **Labeled Breaks** - Multi-level exit → Phase 3
- [ ] **Guard Clauses** - Early returns → Phase 3

### 1.4 Functions & Procedures
- [ ] **Function Definition** - `FUNCTION Name(params)` → Phase 1 (80% done)
- [ ] **Function Calls** - `Name(args)` → Phase 1 (80% done)
- [ ] **Return Values** - `RETURN value` → Phase 1 (needs fixing)
- [ ] **Multiple Parameters** - `FUNCTION(a, b, c)` → Phase 1 (needs fixing)
- [ ] **Default Parameters** - `FUNCTION(x := 10)` → Phase 3
- [ ] **Optional Parameters** - `FUNCTION(x?)` → Phase 3
- [ ] **Rest Parameters** - `FUNCTION(...args)` → Phase 3
- [ ] **Named Arguments** - `Call(name: "value")` → Phase 3
- [ ] **Closures/Lambdas** - `{|x| x + 1}` → Phase 3
- [ ] **Higher-Order Functions** - Functions returning functions → Phase 3
- [ ] **Recursion** - Functions calling themselves → Phase 1 (needs fixing)
- [ ] **Tail Call Optimization** - Stack-safe recursion → Phase 4
- [ ] **Pure Functions** - No side effects (convention) → Phase 6

### 1.5 Variables & Scope
- [x] **Local Variables** - `LOCAL x` ✅ Phase 1
- [x] **Static Variables** - `STATIC counter` ✅ Phase 1
- [x] **Private Variables** - `PRIVATE x` ✅ Phase 1
- [x] **Public Variables** - `PUBLIC x` ✅ Phase 1
- [ ] **Constants** - `CONST PI := 3.14159` → Phase 2
- [ ] **Immutability** - Read-only bindings → Phase 3
- [ ] **Shadowing** - Redefining variables → Phase 3
- [ ] **Destructuring** - `LOCAL {x, y} := point` → Phase 5
- [ ] **Pattern Matching** - Advanced case statements → Phase 5

---

## 2. Object-Oriented Programming

### 2.1 Classes & Objects
- [ ] **Class Definition** - `CLASS Customer ... ENDCLASS` → Phase 8
- [ ] **Instance Creation** - `obj := Customer():New()` → Phase 8
- [ ] **Instance Variables** - `VAR name, age` → Phase 8
- [ ] **Methods** - `METHOD GetName() ... ENDMETHOD` → Phase 8
- [ ] **Constructors** - `METHOD New() ... ENDMETHOD` → Phase 8
- [ ] **Destructors** - `METHOD Destroy() ... ENDMETHOD` → Phase 8
- [ ] **Inheritance** - `CLASS Manager FROM Employee` → Phase 8
- [ ] **Method Overriding** - Redefine parent methods → Phase 8
- [ ] **Super Calls** - `::Super:Method()` → Phase 8
- [ ] **Access Modifiers** - `PROTECTED, HIDDEN` → Phase 8
- [ ] **Static Methods** - Class-level methods → Phase 8
- [ ] **Static Properties** - Class-level data → Phase 8
- [ ] **Interfaces/Protocols** - Abstract contracts → Phase 8
- [ ] **Abstract Classes** - Cannot instantiate → Phase 8
- [ ] **Multiple Inheritance** - (Avoid - use composition) → Never
- [ ] **Mixins/Traits** - Reusable behavior → Phase 8

### 2.2 Encapsulation
- [ ] **Private Members** - `HIDDEN VAR internal` → Phase 8
- [ ] **Protected Members** - `PROTECTED VAR data` → Phase 8
- [ ] **Public Members** - `VAR public` → Phase 8
- [ ] **Property Getters** - `METHOD GetName()` → Phase 8
- [ ] **Property Setters** - `METHOD SetName(value)` → Phase 8
- [ ] **Computed Properties** - Dynamic values → Phase 8

---

## 3. Functional Programming

### 3.1 First-Class Functions
- [ ] **Functions as Values** - `f := MyFunc` → Phase 3
- [ ] **Anonymous Functions** - `{|x| x * 2}` → Phase 3
- [ ] **Closures** - Capture outer scope → Phase 3
- [ ] **Higher-Order Functions** - map/filter/reduce → Phase 3
- [ ] **Currying** - Partial application → Phase 3
- [ ] **Function Composition** - `f >> g` → Phase 6

### 3.2 Immutability & Purity
- [ ] **Immutable Data Structures** - Cannot modify → Phase 5
- [ ] **Persistent Collections** - Structural sharing → Phase 5
- [ ] **Pure Functions** - No side effects → Convention
- [ ] **Referential Transparency** - Same input = same output → Convention

### 3.3 Collection Operations
- [ ] **Map** - Transform elements → Phase 3
- [ ] **Filter** - Select elements → Phase 3
- [ ] **Reduce/Fold** - Accumulate values → Phase 3
- [ ] **Zip** - Combine collections → Phase 5
- [ ] **FlatMap** - Map + flatten → Phase 5
- [ ] **Partition** - Split by predicate → Phase 5
- [ ] **GroupBy** - Group by key → Phase 5

---

## 4. Concurrency & Parallelism

### 4.1 Async/Await
- [ ] **Async Functions** - `ASYNC FUNCTION` → Phase 10
- [ ] **Await Expression** - `AWAIT asyncCall()` → Phase 10
- [ ] **Promises/Futures** - Deferred values → Phase 10
- [ ] **Async Iterators** - Streaming data → Phase 10

### 4.2 Threading & Parallelism
- [ ] **Thread Creation** - Spawn OS threads → Phase 10
- [ ] **Thread Pools** - Reusable workers → Phase 10
- [ ] **Channels** - Message passing → Phase 10
- [ ] **Mutexes/Locks** - Shared state → Phase 10
- [ ] **Atomic Operations** - Lock-free primitives → Phase 10

### 4.3 Goroutines/Actors (Future)
- [ ] **Lightweight Threads** - Green threads → Phase 10+
- [ ] **Actor Model** - Isolated state → Phase 10+
- [ ] **CSP Model** - Communicating processes → Phase 10+

---

## 5. Error Handling

### 5.1 Exceptions
- [ ] **Try/Catch/Finally** - Exception handling → Phase 3
- [ ] **Throw** - Raise exceptions → Phase 3
- [ ] **Custom Exceptions** - User-defined errors → Phase 3
- [ ] **Stack Traces** - Error origin tracking → Phase 3

### 5.2 Result Types (Preferred for Clipper-2025)
- [ ] **Result<T, E>** - Success or error → Phase 5
- [ ] **Option<T>** - Value or nil → Phase 5
- [ ] **Pattern Matching** - Handle variants → Phase 5
- [ ] **Chaining** - `result.map().andThen()` → Phase 5

---

## 6. Standard Library

### 6.1 String Operations
- [x] **Concatenation** - `+` operator ✅ Phase 1
- [ ] **Interpolation** - `"Hello {name}"` → Phase 2
- [ ] **Length** - `Len(str)` → Phase 5
- [ ] **Substring** - `SubStr(str, start, len)` → Phase 5
- [ ] **Upper/Lower** - `Upper(str), Lower(str)` → Phase 5
- [ ] **Trim** - `Trim(str), LTrim(), RTrim()` → Phase 5
- [ ] **Split** - `Split(str, delimiter)` → Phase 5
- [ ] **Join** - `Join(array, separator)` → Phase 5
- [ ] **Replace** - `Replace(str, old, new)` → Phase 5
- [ ] **Regex** - Pattern matching → Phase 5
- [ ] **Format** - `Format(template, args)` → Phase 5

### 6.2 Math Operations
- [ ] **Abs** - `Abs(n)` → Phase 5
- [ ] **Min/Max** - `Min(a, b), Max(a, b)` → Phase 5
- [ ] **Round/Floor/Ceil** - Rounding functions → Phase 5
- [ ] **Sqrt/Pow** - `Sqrt(n), Pow(base, exp)` → Phase 5
- [ ] **Trig Functions** - `Sin(), Cos(), Tan()` → Phase 5
- [ ] **Random** - `Random(), RandomRange(min, max)` → Phase 5

### 6.3 Array/Collection Functions
- [ ] **Length** - `Len(arr)` → Phase 5
- [ ] **Append/Push** - `Append(arr, val)` → Phase 5
- [ ] **Insert** - `Insert(arr, index, val)` → Phase 5
- [ ] **Delete** - `Delete(arr, index)` → Phase 5
- [ ] **Sort** - `Sort(arr)` → Phase 5
- [ ] **Reverse** - `Reverse(arr)` → Phase 5
- [ ] **Find/IndexOf** - `Find(arr, val)` → Phase 5
- [ ] **Contains** - `Contains(arr, val)` → Phase 5
- [ ] **Slice** - `Slice(arr, start, end)` → Phase 5

### 6.4 Date & Time
- [ ] **Current Date/Time** - `Now(), Today()` → Phase 5
- [ ] **Parse Date** - `ParseDate(str)` → Phase 5
- [ ] **Format Date** - `FormatDate(date, pattern)` → Phase 5
- [ ] **Date Arithmetic** - Add/subtract days → Phase 5
- [ ] **Time Zones** - UTC, local, conversions → Phase 5

### 6.5 File I/O
- [ ] **Read File** - `ReadFile(path)` → Phase 5
- [ ] **Write File** - `WriteFile(path, content)` → Phase 5
- [ ] **Append File** - `AppendFile(path, content)` → Phase 5
- [ ] **File Exists** - `FileExists(path)` → Phase 5
- [ ] **Directory Operations** - List, create, delete → Phase 5
- [ ] **Path Manipulation** - Join, dirname, basename → Phase 5

### 6.6 Network/HTTP (Future)
- [ ] **HTTP Client** - GET, POST, etc. → Phase 9+
- [ ] **HTTP Server** - Web framework → Phase 9+
- [ ] **WebSockets** - Real-time communication → Phase 9+
- [ ] **TCP/UDP Sockets** - Low-level networking → Phase 9+

---

## 7. Database Integration

### 7.1 DBF/CDX (Legacy Clipper)
- [ ] **Open DBF** - Clean cursor API (NO USE) → Phase 4
- [ ] **Query Records** - Iterator-based → Phase 4
- [ ] **Insert/Update/Delete** - CRUD operations → Phase 4
- [ ] **Indexes** - CDX support → Phase 4
- [ ] **Transactions** - BEGIN/COMMIT/ROLLBACK → Phase 4

### 7.2 SQL Databases
- [ ] **PostgreSQL** - Driver + query builder → Phase 7
- [ ] **SQLite** - Embedded database → Phase 7
- [ ] **MySQL/MariaDB** - Driver support → Phase 7
- [ ] **SQL Server** - Driver support → Phase 7
- [ ] **Prepared Statements** - SQL injection prevention → Phase 7
- [ ] **Connection Pooling** - Performance → Phase 7
- [ ] **Migrations** - Schema versioning → Phase 7

### 7.3 NoSQL Databases
- [ ] **MongoDB** - Document database → Phase 9
- [ ] **Redis** - Key-value store → Phase 9
- [ ] **Elasticsearch** - Search engine → Phase 9

### 7.4 ORM/Query Builders
- [ ] **Active Record** - Object-relational mapping → Phase 7
- [ ] **Query Builder** - Fluent SQL generation → Phase 7
- [ ] **Relationships** - Has-many, belongs-to → Phase 7

---

## 8. Module System & Packaging

### 8.1 Modules
- [ ] **Import/Require** - Load external code → Phase 5
- [ ] **Export** - Expose functions/classes → Phase 5
- [ ] **Namespaces** - Avoid name collisions → Phase 5
- [ ] **Module Aliases** - `IMPORT pkg AS alias` → Phase 5
- [ ] **Selective Imports** - `IMPORT {func1, func2} FROM pkg` → Phase 5

### 8.2 Package Management
- [ ] **Package Manifest** - `marina.toml` → Phase 12
- [ ] **Dependency Resolution** - Semantic versioning → Phase 12
- [ ] **Package Registry** - "Dockyard" central repo → Phase 12
- [ ] **Lock File** - Reproducible builds → Phase 12
- [ ] **Private Packages** - Internal distribution → Phase 12
- [ ] **Binary Packages** - Compiled libraries (.cjar) → Phase 12

---

## 9. Developer Tooling

### 9.1 Language Server Protocol (LSP)
- [x] **Syntax Diagnostics** - Error highlighting ✅ Phase 3 (basic)
- [x] **Keyword Completion** - Auto-complete ✅ Phase 3 (basic)
- [x] **Hover Info** - Type/doc tooltips ✅ Phase 3 (basic)
- [ ] **Go to Definition** - Jump to source → Phase 3 (in progress)
- [ ] **Find References** - Usage search → Phase 3 (in progress)
- [ ] **Rename Symbol** - Refactoring → Phase 3 (in progress)
- [ ] **Code Actions** - Quick fixes → Phase 3
- [ ] **Semantic Highlighting** - Better syntax colors → Phase 3
- [ ] **Inlay Hints** - Type annotations → Phase 3

### 9.2 Debug Adapter Protocol (DAP)
- [ ] **Breakpoints** - Pause execution → Phase 3
- [ ] **Step In/Out/Over** - Code navigation → Phase 3
- [ ] **Variable Inspection** - Runtime values → Phase 3
- [ ] **Watch Expressions** - Evaluate on-the-fly → Phase 3
- [ ] **Call Stack** - Execution history → Phase 3
- [ ] **Conditional Breakpoints** - Break on condition → Phase 3

### 9.3 Code Formatter
- [ ] **Auto-Format** - Consistent style → Phase 3
- [ ] **Configuration** - Custom rules → Phase 3
- [ ] **Editor Integration** - Format on save → Phase 3

### 9.4 Testing Framework
- [ ] **Unit Tests** - Function testing → Phase 5
- [ ] **Assertions** - `Assert(condition, message)` → Phase 5
- [ ] **Test Runner** - Automated execution → Phase 5
- [ ] **Mocking** - Test doubles → Phase 5
- [ ] **Coverage** - Line/branch coverage → Phase 5

### 9.5 Build System
- [x] **Compile to Bytecode** - `.prg → bytecode` ✅ Phase 1
- [ ] **Bytecode Serialization** - Save to `.bc` files → Phase 1 (pending)
- [ ] **Multi-file Projects** - Link modules → Phase 5
- [ ] **Incremental Compilation** - Only changed files → Phase 6
- [ ] **Optimization Levels** - `-O0, -O1, -O2, -O3` → Phase 13
- [ ] **Dead Code Elimination** - Remove unused → Phase 13

### 9.6 Documentation
- [ ] **Doc Comments** - `/// Documentation` → Phase 5
- [ ] **Doc Generation** - HTML/Markdown output → Phase 5
- [ ] **Examples in Docs** - Runnable code → Phase 5

---

## 10. Performance & Optimization

### 10.1 Compiler Optimizations
- [ ] **Constant Folding** - Compile-time evaluation → Phase 4
- [ ] **Dead Code Elimination** - Remove unreachable → Phase 4
- [ ] **Peephole Optimization** - Local patterns → Phase 4
- [ ] **Inline Expansion** - Small function inlining → Phase 4
- [ ] **Tail Call Optimization** - Stack-safe recursion → Phase 4

### 10.2 JIT Compilation
- [ ] **Bytecode → Native** - Runtime compilation → Phase 13
- [ ] **Inline Caching** - Call site optimization → Phase 13
- [ ] **Speculative Optimization** - Assume types → Phase 13
- [ ] **Deoptimization** - Fall back to interpreter → Phase 13

### 10.3 Memory Management
- [ ] **Garbage Collection** - Automatic memory → Phase 4
- [ ] **Mark & Sweep** - Simple GC → Phase 4
- [ ] **Generational GC** - Optimized GC → Phase 4+
- [ ] **Reference Counting** - Alternative to GC → Phase 4
- [ ] **Memory Profiling** - Leak detection → Phase 4

---

## 11. Interoperability

### 11.1 Foreign Function Interface (FFI)
- [ ] **Call C Functions** - Native integration → Phase 5
- [ ] **Call Rust Functions** - Host language → Phase 5
- [ ] **Shared Libraries** - `.so/.dll/.dylib` → Phase 5
- [ ] **Type Marshalling** - Convert types → Phase 5

### 11.2 Embedding
- [ ] **Library Mode** - Use as library → Phase 5
- [ ] **Scripting API** - Embed in apps → Phase 5
- [ ] **Sandboxing** - Security isolation → Phase 6

### 11.3 WebAssembly
- [ ] **Compile to WASM** - Browser execution → Phase 14
- [ ] **WASI Support** - System interface → Phase 14

---

## 12. Platform & Deployment

### 12.1 Cross-Platform Support
- [x] **macOS** - Native support ✅ Phase 1
- [x] **Linux** - Native support ✅ Phase 1
- [x] **Windows** - Native support ✅ Phase 1
- [ ] **iOS/Android** - Mobile (future) → Phase 14+
- [ ] **Web (WASM)** - Browser → Phase 14

### 12.2 Deployment
- [ ] **Single Binary** - Statically linked → Phase 5
- [ ] **Binary Packages** - `.cjar` format → Phase 12
- [ ] **Docker Images** - Containerization → Phase 12
- [ ] **Cloud Functions** - Serverless → Phase 14

---

## 13. GUI & User Interface

### 13.1 Terminal/Console
- [x] **Basic Output** - `? "Hello"` ✅ Phase 1
- [x] **ANSI Colors** - Terminal colors ✅ Phase 1 (via lib)
- [x] **Cursor Control** - SetPos, GotoXY ✅ Phase 1 (via lib)
- [ ] **Readline** - Input editing → Phase 3
- [ ] **Progress Bars** - Visual feedback → Phase 5
- [ ] **Tables** - Formatted output → Phase 5

### 13.2 Cross-Platform GUI
- [ ] **Window Creation** - Modern VO concept → Phase 11
- [ ] **Declarative DSL** - UI description → Phase 11
- [ ] **Widgets** - Buttons, inputs, etc. → Phase 11
- [ ] **Layout System** - Flexible positioning → Phase 11
- [ ] **Event Handling** - User interactions → Phase 11
- [ ] **Data Binding** - Model-view sync → Phase 11

---

## 14. Security

### 14.1 Memory Safety
- [x] **No Buffer Overflows** - Rust-based VM ✅ Phase 1
- [x] **No Use-After-Free** - Rust ownership ✅ Phase 1
- [ ] **Input Validation** - Built-in sanitization → Phase 5

### 14.2 Cryptography
- [ ] **Hashing** - SHA256, etc. → Phase 7
- [ ] **Encryption** - AES, etc. → Phase 7
- [ ] **Random** - Cryptographically secure → Phase 7

---

## Summary: Phase Mapping

| Phase | Focus | Completion % | Priority Features |
|-------|-------|--------------|-------------------|
| **Phase 1** | Core VM & Compiler | 80% | ✅ Basic types, operators, control flow, bytecode |
| **Phase 2** | Arrays/Maps/Lists | 0% | 🎯 Maps, lists, for-each, ranges, constants |
| **Phase 3** | Tooling & Dev XP | 20% | 🔄 LSP, DAP, formatter, closures, higher-order fns |
| **Phase 4** | DBF/CDX Engine | 0% | Clean cursor API, transactions |
| **Phase 5** | Standard Library | 0% | String, math, array, file I/O, modules |
| **Phase 6** | Macro System | 0% | Compile-time code generation |
| **Phase 7** | SQL Engines | 0% | PostgreSQL, SQLite, query builders |
| **Phase 8** | Native OOP | 0% | Classes, inheritance, methods |
| **Phase 9** | NoSQL Engines | 0% | MongoDB, Redis |
| **Phase 10** | Async/Await | 0% | Concurrency model, threads |
| **Phase 11** | Cross-Platform GUI | 0% | Modern VO concept, declarative DSL |
| **Phase 12** | Package Ecosystem | 0% | Dockyard registry, marina.toml |
| **Phase 13** | JIT Compiler | 0% | Performance optimization |
| **Phase 14** | WASM & Embedded | 0% | Web, IoT targets |

---

## Next Immediate Actions (Priority Order)

### 🎯 Phase 1 Completion (80% → 100%)
1. Fix user-defined functions (parameters, return values, call frames)
2. Implement bytecode serialization (.bc files)
3. Improve error messages with source location
4. Document bytecode spec in docs/reference/bytecode.md

### 🎯 Phase 2 - Essential Foundation (NEXT PRIORITY)
1. **Maps/Hashes** - `map := {"key" => "value"}`
2. **Lists** - Dynamic ordered collections
3. **For-Each** - `FOR EACH item IN collection`
4. **Constants** - `CONST PI := 3.14159`
5. **String Interpolation** - `"Hello {name}"`
6. **ElseIf** - Multi-way conditionals
7. **Ranges** - `1..10` syntax

### 🎯 Phase 3 - Developer Experience (IN PROGRESS)
1. Complete LSP features (go-to-def, find refs, rename)
2. Implement DAP (breakpoints, stepping, inspection)
3. Code formatter (auto-format, style enforcement)
4. Closures/lambdas - `{|x| x + 1}`
5. Higher-order functions (map, filter, reduce)

---

**Last Review:** 2025-11-19  
**Next Review:** After Phase 2 completion
