# Roadmap

This roadmap reflects repository reality as of the current Phase 3 review.

Status labels:

- `Completed`
- `In Progress`
- `Planned`
- `Deferred`
- `Experimental`

## Phase Summary

### Phase 1: Core Language and Execution

**Status: Completed**

Completed items:

- Lexer
- Recursive-descent parser
- AST representation
- Bytecode compiler
- Stack-based VM
- Arithmetic, comparison, and logical operators
- Variables and assignment
- Arrays and indexed assignment
- Core control flow
- Functions and procedures
- REPL and inspection flags

Notes:

- “Completed” here means implemented and test-covered, not frozen forever.
- Some edge cases still need cleanup inside Phase 3.

### Phase 2: Language Surface Expansion

**Status: Completed**

Completed items:

- `CASE` / `OTHERWISE`
- Augmented assignment
- Increment and decrement
- Additional examples
- Parser/compiler/VM modularization

Remaining caveats:

- Nested `CASE` is still ignored in tests and should be fixed before v1.0.

### Phase 3: Runtime and Module Stabilization

**Status: In Progress**

#### Completed

- Call-frame based function execution
- Console and input builtins dispatched through the VM
- Formatter MVP
- Documentation renderer
- Basic syntax extension for VS Code
- Test coverage for compiler, parser, VM, formatter, and docs subsystems

#### In Progress

- VM correctness hardening
- Bytecode behavior review and stabilization
- Function call semantics review
- Documentation alignment with implemented behavior
- Example coverage for core language behavior

#### Planned

- Minimal import design
- Minimal module architecture
- Lazy-loading design bounded to core use cases
- Phase 3 release criteria and examples cleanup

#### Deferred

- Expanded editor integration
- DAP implementation
- Rich LSP feature work

#### Experimental

- `marina-lsp` binary

### Post-Phase-3 Work

These items are not current-phase deliverables.

#### Planned

- Standard library growth beyond current builtins
- File and module packaging story
- Better debugging support

#### Deferred

- Database engines
- PostgreSQL drivers
- MongoDB drivers
- HTTP clients
- Web frameworks
- Async runtime
- LSP rollout as a supported product feature
- Full debugger

## Detailed Status

| Item | Status | Notes |
| --- | --- | --- |
| Lexer/tokenization | Completed | Covered by tests |
| Parser | Completed | Diagnostics path exists |
| AST | Completed | Single-file program model |
| Bytecode compiler | Completed | Stable enough for current tests |
| VM run loop | In Progress | Needs stabilization pass, not redesign |
| Function call frames | In Progress | Implemented, but still part of stabilization scope |
| Arrays/indexing | Completed | Parse, compile, execute |
| `CASE` statements | In Progress | Implemented; nested case still open |
| Database statements | Experimental | Parse/compile path exists; runtime is stubbed |
| Module system | Planned | Not implemented |
| Import system | Planned | Not implemented |
| Lazy loading | Planned | Not implemented |
| Native module ABI | Planned | Not implemented |
| Built-in runtime functions | Completed | Implemented inside VM |
| Formatter | Completed | MVP only |
| Docs renderer | Completed | HTML and PDF output tested |
| VS Code syntax package | Completed | Syntax-only support |
| `marina-lsp` | Experimental | Feature-gated binary, not integrated |
| `marina-dap` | Deferred | Stub only |

## Roadmap Corrections From Earlier Docs

The repository no longer treats the following as active near-term work:

- DBF/CDX engine work
- PostgreSQL and MongoDB backends
- Macro system rollout
- Package manager work
- Async runtime work
- “Tooling-first” phase framing

Those topics can stay as long-term design notes, but they are not current implementation milestones.
