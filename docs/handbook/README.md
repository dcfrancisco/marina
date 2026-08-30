# Clipper-2025 / Marina Handbook

*Current runtime guide plus clearly marked design material*
**Version 1.0 (2025 Edition)**

---

## Welcome

This handbook combines current Marina behavior with historical and future
design notes. It is not a claim that every described module, database, or macro
feature is implemented. For release scope, use the
[Supported Features](../reference/SUPPORTED_FEATURES.md) reference.

Clipper-2025 is:
- **Expression-first** modern language
- **VM-based** with portable bytecode
- **Built-in namespace imports** with user modules planned
- **Database-boundary documented** (persistence is deferred)
- **Macro direction documented** (preprocessing is deferred)
- **Cross-platform** (macOS, Linux, Windows)
- **Future-proof** for the next 30 years

---

## Quick Start

**New to Clipper-2025?** Start here:

1. [Introduction](introduction.md) - Project vision and goals
2. [Language → Design Philosophy](language/design_philosophy.md) - Core principles
3. [Language → Syntax](language/syntax.md) - Quick language overview

**Building applications?** Read these, noting that database and module pages
are design notes:

1. [Architecture → Modules](architecture/modules.md) - How to organize code
2. [Database → DBF Engine](database/dbf_engine.md) - Future database design notes
3. [Reference → Standard Library](reference/stdlib.md) - Built-in functions

**Implementing tools or extending Marina?** Technical deep-dives:

1. [Architecture → Compiler](architecture/compiler.md) - Compilation pipeline
2. [Architecture → VM](architecture/vm.md) - Virtual machine specification
3. [Reference → Bytecode](reference/bytecode.md) - Instruction set
4. [Reference → Grammar](reference/grammar.md) - Formal EBNF grammar

---

## Table of Contents

### 📘 Introduction
- [Introduction](introduction.md) - Project overview and goals

### 🔤 Language

- [Design Philosophy](language/design_philosophy.md) - Core principles and approach
- [Syntax Overview](language/syntax.md) - Complete language syntax reference

### 🏗️ Architecture

- [Compiler](architecture/compiler.md) - Lexer, Parser, AST, Code generation
- [Virtual Machine](architecture/vm.md) - Execution model, stack machine, memory
- [Modules](architecture/modules.md) - Module system, imports, namespaces
- [Macros](architecture/macros.md) - `.ch` system, AST macros, DSLs

### 💾 Database

- [DBF Engine](database/dbf_engine.md) - Future DBF/CDX design notes
- [Modern Backends](database/modern_backends.md) - Future SQL/NoSQL design notes

### 🔮 Vision & Roadmap

- [Lost Visions](vision/lost_visions.md) - Unfulfilled Clipper dreams now realized
- [Roadmap](vision/roadmap.md) - Development phases 2025-2027+

### 📖 Reference

- [Grammar](reference/grammar.md) - Formal EBNF specification
- [Bytecode](reference/bytecode.md) - Complete instruction set
- [Standard Library](reference/stdlib.md) - Functions, modules, CLI tools

---

## Reading Paths

### For Developers (Learning the Language)

1. Introduction
2. Language → Design Philosophy
3. Language → Syntax
4. Architecture → Modules
5. Database → DBF Engine
6. Reference → Standard Library

**Time to read:** ~2 hours

### For Language Implementors

1. Introduction
2. Architecture → Compiler
3. Architecture → VM
4. Reference → Grammar
5. Reference → Bytecode
6. Architecture → Macros

**Time to read:** ~3 hours

### For Understanding Vision

1. Introduction
2. Vision → Lost Visions
3. Vision → Roadmap
4. Language → Design Philosophy
5. Architecture → Modules
6. Database → Modern Backends

**Time to read:** ~1.5 hours

---

## Project Structure

This handbook is organized into focused documents for easy navigation. Pages
marked “Future Design Notes” are not current runtime capabilities:

```
handbook/
├── README.md (this file)
├── introduction.md
├── language/
│   ├── design_philosophy.md
│   └── syntax.md
├── architecture/
│   ├── compiler.md
│   ├── vm.md
│   ├── modules.md
│   └── macros.md
├── database/
│   ├── dbf_engine.md
│   └── modern_backends.md
├── vision/
│   ├── lost_visions.md
│   └── roadmap.md
└── reference/
    ├── grammar.md
    ├── bytecode.md
    └── stdlib.md
```

---

## Quick Links

- **Repository:** [Marina on GitHub](https://github.com/dcfrancisco/marina)
- **Main Documentation:** [docs/README.md](../README.md)
- **Examples:** [examples/](../../examples/)
- **Source Code:** [src/](../../src/)

---

## Contributing

Marina is open-source and community-driven. This handbook serves as both:

1. **User manual** - How to use Clipper-2025
2. **Specification** - How the language and VM work
3. **Vision document** - Where the project is heading

Improvements, corrections, and clarifications are welcome.

---

## License

Marina is intended to be:

- Open-source
- Permissive license (MIT or Apache)
- Community-driven
- The official spiritual successor to Clipper
- Modern, safe, and future-proof

**No baggage from xBase standards.**
**Clipper restored — but pure.**

---

*"Clipper reborn as a modern VM-based language, yet still feeling like the Clipper you used in the 1990s — but without the baggage of xBase."*
