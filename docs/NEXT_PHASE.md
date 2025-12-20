# Next Phase: Packaging & Advanced Features

## 🎯 Phase Roadmap

This document outlines the next development phases for Clipper-2025, building on the working compiler and VM foundation.

---

## 🟦 PHASE 1: Complete Core Language Features

**Priority: HIGH** | **Estimated: 2-3 weeks**

### 1.1 Function Implementation
- [ ] Proper function call frames with parameter passing
- [ ] RETURN statement with value propagation
- [ ] Function table in bytecode
- [ ] Recursive function support
- [ ] Local variable scoping within functions

**Files to modify:**
- `src/compiler.rs` - Function compilation
- `src/vm.rs` - Call frame management
- `src/bytecode.rs` - Function value type

### 1.2 Standard Library
- [ ] String functions (SUBSTR, LEN, UPPER, LOWER, TRIM)
- [ ] Math functions (ABS, SQRT, ROUND, etc.)
- [ ] Array functions (ALEN, ASORT, ASCAN)
- [ ] Type conversion (STR, VAL, CTOD, DTOC)

**New files:**
- `src/stdlib/` - Standard library modules
- `src/stdlib/strings.rs`
- `src/stdlib/math.rs`
- `src/stdlib/arrays.rs`

### 1.3 Advanced Control Flow
- [ ] ELSEIF support (currently nested IF only)
- [ ] SWITCH/CASE statements
- [ ] BREAK statement for loops
- [ ] CONTINUE statement for loops

---

## 🟩 PHASE 2: Bytecode Packaging (.bc files)

**Priority: MEDIUM** | **Estimated: 1 week**

### 2.1 Bytecode Serialization
Implement `.bc` file format for saving/loading compiled bytecode.

**Format Specification:**
```
Magic: "CLBC" (4 bytes)
Version: u16 (2 bytes)
Constant Pool:
  - Count: u32
  - For each constant:
    - Type: u8 (0=Nil, 1=Number, 2=String, 3=Boolean, 4=Array)
    - Data: varies by type
Code Section:
  - Count: u32
  - Instructions: Vec<(OpCode, Option<usize>)>
```

**Implementation:**
- [ ] Serialization trait for bytecode
- [ ] Write `.bc` file format
- [ ] Read `.bc` file format
- [ ] Verify bytecode integrity (magic + version check)

**New file:**
- `src/bc_format.rs` - Bytecode serialization/deserialization

**Usage:**
```bash
# Compile to .bc
clipper -c program.prg -o program.bc

# Run .bc file
clipper program.bc
```

---

## 🟧 PHASE 3: Package System (.cpkg)

**Priority: MEDIUM** | **Estimated: 2 weeks**

### 3.1 Multi-file Projects
Support for organizing code across multiple `.prg` files.

**Package Structure:**
```
myapp/
├─ manifest.json
├─ src/
│  ├─ main.prg
│  ├─ customer.prg
│  ├─ invoice.prg
│  └─ utils.prg
└─ resources/
   └─ config.txt
```

**Manifest Format:**
```json
{
  "name": "myapp",
  "version": "1.0.0",
  "entry": "src/main.prg",
  "files": [
    "src/main.prg",
    "src/customer.prg",
    "src/invoice.prg",
    "src/utils.prg"
  ],
  "resources": [
    "resources/config.txt"
  ]
}
```

### 3.2 Package Format (.cpkg)
ZIP archive containing compiled bytecode + manifest.

**Structure:**
```
myapp.cpkg (ZIP)
├─ META-INF/
│  └─ MANIFEST.json
├─ bytecode/
│  ├─ main.bc
│  ├─ customer.bc
│  ├─ invoice.bc
│  └─ utils.bc
└─ resources/
   └─ config.txt
```

**Implementation:**
- [ ] Manifest parser
- [ ] Multi-file compilation
- [ ] ZIP creation (use `zip` crate)
- [ ] Package loader
- [ ] Module system (IMPORT/REQUIRE)

**New files:**
- `src/package/mod.rs`
- `src/package/manifest.rs`
- `src/package/loader.rs`

**Usage:**
```bash
# Create package
clipper pack myapp/

# Run package
clipper myapp.cpkg

# Extract package
clipper unpack myapp.cpkg -o myapp/
```

---

## 🟥 PHASE 4: JAR-Style Packaging (.cjar)

**Priority: LOW** | **Estimated: 1 week**

### 4.1 Java JAR Equivalent
Full-featured package format with classpath support.

**Format:**
```
myapp.cjar (ZIP)
├─ META-INF/
│  └─ MANIFEST.MF
├─ bytecode/
│  ├─ Main.bc
│  └─ lib/*.bc
├─ resources/
│  └─ assets/*
└─ lib/
   └─ external.cjar
```

**MANIFEST.MF:**
```
Main-Class: Main
Bytecode-Version: 1.0
Clipper-Version: 2025
Class-Path: lib/external.cjar
```

**Features:**
- [ ] Main-Class entry point
- [ ] Classpath resolution
- [ ] Nested package loading
- [ ] Resource loading API

**Usage:**
```bash
# Create executable JAR
clipper jar myapp/ -o myapp.cjar

# Run with classpath
clipper -cp myapp.cjar:lib/utils.cjar Main

# Simple run
clipper -jar myapp.cjar
```

---

## 🟪 PHASE 5: Standalone Executables

**Priority: LOW** | **Estimated: 2 weeks**

### 5.1 Self-Contained Binary
Embed VM + bytecode into a single executable (like PyInstaller).

**Approach:**
1. Compile all `.prg` → `.bc`
2. Embed bytecode into VM binary
3. Strip debugging symbols
4. Produce platform-specific executable

**Implementation:**
- [ ] Binary resource embedding
- [ ] Startup stub that extracts bytecode
- [ ] Platform-specific builds (Windows .exe, Linux binary, macOS app)

**New files:**
- `src/packager/mod.rs`
- `src/packager/embed.rs`
- `build.rs` (build script)

**Usage:**
```bash
# Build standalone executable
clipper build myapp.prg --output myapp

# Run
./myapp
```

**Result:**
- Windows: `myapp.exe`
- Linux: `myapp`
- macOS: `myapp.app`

---

## 🟫 PHASE 6: Advanced Language Features

**Priority: VARIES** | **Estimated: 4-6 weeks**

### 6.1 Closures/Lambdas (Codeblocks)
```clipper
LOCAL transform := {|x| x * 2}
LOCAL result := Eval(transform, 5)  // 10
```

**Requirements:**
- [ ] Block literal parsing `{|params| body}`
- [ ] Upvalue capture
- [ ] Closure compilation
- [ ] Eval() built-in function

### 6.2 Object-Oriented Programming
```clipper
CLASS Customer
   VAR name
   VAR balance
   
   METHOD new(n, b)
      ::name := n
      ::balance := b
      RETURN Self
      
   METHOD deposit(amount)
      ::balance := ::balance + amount
ENDCLASS

LOCAL c := Customer():new("Alice", 100)
c:deposit(50)
```

**Requirements:**
- [ ] CLASS/ENDCLASS parsing
- [ ] METHOD definitions
- [ ] Self reference
- [ ] Instance creation
- [ ] Method dispatch

### 6.3 Exception Handling
```clipper
TRY
   // risky code
CATCH e
   ? "Error:", e:message
FINALLY
   // cleanup
ENDTRY
```

---

## 📊 Priority Matrix

| Phase | Priority | Complexity | Impact | Dependencies |
|-------|----------|------------|--------|--------------|
| Phase 1: Core Features | 🔴 HIGH | Medium | High | None |
| Phase 2: .bc Files | 🟡 MEDIUM | Low | Medium | None |
| Phase 3: .cpkg | 🟡 MEDIUM | Medium | High | Phase 2 |
| Phase 4: .cjar | 🟢 LOW | Low | Medium | Phase 3 |
| Phase 5: Standalone | 🟢 LOW | High | High | Phase 2 |
| Phase 6: Advanced | 🟡 VARIES | High | High | Phase 1 |

---

## 🎯 Recommended Order

### Iteration 1 (Foundation)
1. ✅ **Complete** - Basic compiler + VM
2. **Next** - Phase 1.1: Functions (critical for real programs)
3. **Next** - Phase 1.2: Standard library basics

### Iteration 2 (Persistence)
4. Phase 2: Bytecode serialization (.bc files)
5. Phase 1.3: Advanced control flow

### Iteration 3 (Packaging)
6. Phase 3: Multi-file projects + .cpkg
7. Phase 6.1: Closures/lambdas

### Iteration 4 (Polish)
8. Phase 4: JAR-style packaging
9. Phase 6.2: OOP support
10. Phase 5: Standalone executables

---

## 📝 Next Immediate Steps

**Week 1-2: Functions**
- Implement proper call frames
- Add function parameters
- Fix RETURN statement
- Test recursive functions

**Week 3: Standard Library Core**
- String manipulation
- Basic math functions
- Array utilities

**Week 4: Bytecode Files**
- Design .bc format
- Implement serialization
- Add compile/run modes

---

## 🔧 Technical Debt to Address

- [ ] Better error messages with line numbers
- [ ] Memory management optimization
- [ ] Bytecode optimization pass
- [ ] Comprehensive test suite
- [ ] Benchmarking suite
- [ ] Profiler integration
- [ ] Debugger with breakpoints

---

## 📚 Documentation Needed

- [ ] Language specification document
- [ ] Bytecode format specification
- [ ] Package format specification
- [ ] API documentation
- [ ] Tutorial series
- [ ] Migration guide (from classic Clipper)

---

## 🚀 Long-term Vision

**Clipper-2025 Goals:**
- Modern, fast, expression-based language
- Full compatibility with packaging ecosystems
- Easy distribution (JAR-like)
- Native performance (Rust VM)
- Rich standard library
- Strong OOP support
- Lambda/closure support
- Exception handling
- Interop with Rust/C (FFI)

---

*This roadmap is a living document. Update as priorities shift and new requirements emerge.*
