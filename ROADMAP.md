# Roadmap

This roadmap reflects repository reality as of the Phase 3 release-candidate review.

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

- Broader language compatibility remains post-Phase-3 work.

### Phase 3: Runtime and Module Stabilization

**Status: Release-candidate ready; CI deferred to backlog**

#### Completed

- Call-frame based function execution
- Console and input builtins dispatched through the VM
- Formatter MVP
- Documentation renderer
- Basic syntax extension for VS Code
- Test coverage for compiler, parser, VM, formatter, and docs subsystems
- Minimal persisted DBF runtime: `USE`, `DBLIST`, `DBEOF()`, command `SKIP`,
  function `DBSKIP()`,
  `DBGOTOP`, `DBGOBOTTOM`, `DBSEEK`, and `REPLACE`
- Runnable DBF example and generated fixture helper

#### Backlog

- Reactivate automatic CI when maintainer ownership and branch protection are ready

#### Post-Phase-3 planned

- Minimal module architecture beyond the current built-in namespace imports
- Structured diagnostics and deeper runtime tooling

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

- Expanded database engine capabilities beyond the shipped DBF subset
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
| VM run loop | Completed for RC baseline | Covered by current VM tests |
| Function call frames | Completed for RC baseline | Calls, returns, recursion, and cleanup are tested |
| Arrays/indexing | Completed | Parse, compile, execute |
| `CASE` statements | Completed | Includes nested `CASE` and `OTHERWISE` |
| Database statements | Completed (minimal DBF) | Persisted DBF cursor/list/seek/replace subset; advanced features pending |
| Module system | Planned | User modules and filesystem loading are not implemented |
| Import system | Completed (minimal) | Static imports for five built-in namespaces only |
| Lazy loading | Planned | Not implemented |
| Native module ABI | Planned | Not implemented |
| Built-in runtime functions | Completed | Implemented inside VM |
| Formatter | Completed | MVP only |
| Docs renderer | Completed | HTML and PDF output tested |
| VS Code syntax package | Completed | Syntax-only support |
| `marina-lsp` | Experimental | Feature-gated binary, not integrated |
| `marina-dap` | Deferred | Stub only |

## Current Gaps and Pending Work

The following are the active gaps after the current implementation review:

- `.ch` preprocessing and `#include`/`#command`/`#translate`
- User modules and filesystem loading
- Structured diagnostics and exception handling
- CDX/NTX indexes, memo fields, append/delete, locking, and transactions
- SQL/NoSQL drivers and backend-neutral database APIs
- Automatic CI (currently manual/backlog)
- Full LSP and DAP implementations

## Roadmap Corrections From Earlier Docs

The repository no longer treats the following as active near-term work:

- DBF/CDX engine work
- PostgreSQL and MongoDB backends
- Macro system rollout
- Package manager work
- Async runtime work
- “Tooling-first” phase framing

Those topics can stay as long-term design notes, but they are not current implementation milestones.
