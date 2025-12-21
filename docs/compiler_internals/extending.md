# Extending the Compiler

Guidelines for adding new language features, opcodes, or built-in functions to Marina.

- Where to add new syntax: `src/parser/`
- Where to add new bytecode: `src/compiler/`, `src/bytecode.rs`
- Where to add new VM behavior: `src/vm/`