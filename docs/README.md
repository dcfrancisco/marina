# Marina Documentation

Welcome to the Marina/Clipper-2025 documentation.

---

## 📚 **Documentation Structure**

Marina documentation is organized into three collections:

### 📘 [Official Handbook](handbook/README.md) — *Comprehensive Learning*
Complete end-to-end coverage of Clipper-2025:
- **Language** - Syntax, design philosophy
- **Architecture** - Compiler, VM, modules, macros
- **Database** - DBF/CDX engine, modern backends
- **Vision** - Lost Clipper dreams realized, roadmap
- **Reference** - Grammar, bytecode, standard library

**Best for:** Learning the system from scratch, understanding design decisions, implementors.

### 📖 [Practical Guides](guides/) — *How-To Documentation*
Task-focused guides for working with Marina:
- [Getting Started](guides/getting_started.md) - First steps with Clipper-2025
- [Writing Modules](guides/writing_modules.md) - Module system and organization
- [Database Programming](guides/database_programming.md) - DBF/CDX and future backends
- [Compiler Internals](guides/compiler_internals.md) - Lexer, parser, VM architecture
- [Macro Development](guides/macro_development.md) - Safe macro system

**Best for:** Developers writing code, building applications, specific tasks.

### 📑 [Quick Reference](reference/) — *Fast Lookups*
Concise technical references:
- [Language Syntax](reference/language_syntax.md) - Complete syntax guide
- [Bytecode Opcodes](reference/bytecode_opcodes.md) - VM instruction set
- [Standard Library](reference/stdlib_functions.md) - Built-in functions
- [Clipper Vision](reference/clipper_vision.md) - Project philosophy

**Best for:** Quick lookups, syntax checks, API reference.

---

## 🚀 **Quick Links**

- **Current Status**: Phase 2 and Phase 2.5 delivered; Phase 3 stabilization in progress
- **Next Phase**: Phase 3 - Runtime & Module Stabilization
- **Formatter**: `clipper fmt` (MVP: indentation + keyword casing)
- **See**: [Project README](../README.md) for implementation status
- **See**: [Instructions](../.github/copilot/INSTRUCTIONS.md) for development guidelines

---

## 📖 **Recommended Reading Paths**

### 🚀 New to Clipper-2025?

1. [Getting Started Guide](guides/getting_started.md) - First program in 5 minutes
2. [Language Syntax Reference](reference/language_syntax.md) - Quick syntax overview
3. [Standard Library](reference/stdlib_functions.md) - Available functions

### 📚 Want Deep Understanding?

Start with the [Official Handbook](handbook/README.md) which provides structured paths for:
- **Developers** - Learning to write Clipper-2025 code (~2 hours)
- **Implementors** - Understanding compiler/VM internals (~3 hours)
- **Visionaries** - Understanding project philosophy (~1.5 hours)

### 🛠️ Building Applications?

1. [Getting Started Guide](guides/getting_started.md)
2. [Writing Modules Guide](guides/writing_modules.md)
3. [Database Programming Guide](guides/database_programming.md)
4. [Standard Library Reference](reference/stdlib_functions.md)

### 🔧 Extending Marina?

1. [Compiler Internals Guide](guides/compiler_internals.md)
2. [Handbook Architecture Section](handbook/architecture/)
3. [Bytecode Reference](reference/bytecode_opcodes.md)
4. [Macro Development Guide](guides/macro_development.md)

---

## 🎯 **Project Goals**

Marina/Clipper-2025 completes the vision that Computer Associates never finished:

✅ **True Clipper VM** - Bytecode execution, cross-platform
✅ **Modern Language** - Expression-first, no xBase commands
✅ **Modular Architecture** - Clean separation of concerns
✅ **Database Agnostic** - DBF, PostgreSQL, MongoDB, etc.
✅ **Safe & Predictable** - No macro hell, proper errors
✅ **Extensible** - Module system, future OOP, macros
✅ **Cross-Platform** - macOS, Linux, Windows, future WASM

---

## 🛠️ **Contributing**

See the main [README](../README.md) for build instructions and [INSTRUCTIONS.md](../.github/copilot/INSTRUCTIONS.md) for development guidelines.

---

## 📜 **License**

MIT License - Copyright (c) 2025 Danny Francisco
