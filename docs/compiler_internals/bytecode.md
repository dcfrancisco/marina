# Bytecode Generation

The compiler traverses the AST and emits bytecode instructions for the Marina VM. This stage handles variable scope, function calls, and control flow.

- Input: AST
- Output: Bytecode (instructions + constants)
- Key files: `src/compiler/`, `src/bytecode.rs`