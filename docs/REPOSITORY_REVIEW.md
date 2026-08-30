# Repository Review

> Updated after local Phase 3 remediation on 2026-08-31. The remaining
> CI is currently backlog work; see [CI status](CI_STATUS.md).

This review consolidates the requested deliverables:

1. Documentation Drift Report
2. Roadmap Status Report
3. Phase 3 Completion Assessment
4. Recommended Documentation Changes
5. Release Readiness Checklist

Review basis:

- Source tree under `src/`
- Tests under `tests/`
- Examples under `examples/`
- VS Code extension under `vscode/marina/`
- Existing repository documentation
- `cargo test --all-targets --no-fail-fast` result: 100 passed, 0 failed
- Non-interactive examples: 28 ran, 13 skipped, 0 failed

## 1. Documentation Drift Report

### High-confidence drift

These topics are repeatedly documented as implemented even though the repository does not implement them:

- Module system and `import`
- Lazy module loading
- Native module/plugin ABI
- DBF/CDX engine
- PostgreSQL and MongoDB backends
- Macro system
- Package management
- Async runtime
- Full IDE integration

### Concrete mismatches found

- `README.md` previously framed the project around “Clipper programming language” and older phase labels, while the current requested focus is Phase 3 runtime/module stabilization.
- `docs/handbook/architecture/modules.md` described a live module system with `module.json`, exports, built-in modules, dependency resolution, and versioning. None of that exists in the code.
- `docs/guides/writing_modules.md` described import search paths, metadata files, native modules, and install flows that are not implemented.
- `docs/guides/database_programming.md` and handbook database pages described a DBF engine and future backends as an active design track even though the runtime only stubs database opcodes.
- `docs/handbook/architecture/macros.md` described a three-tier macro system that does not exist in the parser or compiler.
- `docs/handbook/vision/roadmap.md` used an older phase model centered on tooling, DB engines, macros, and ecosystem phases rather than the current Phase 3 stabilization target.
- `docs/PROJECT_SUMMARY.md` was behind the codebase and still claimed functions were not implemented.
- `docs/README.md` and `docs/handbook/README.md` presented speculative handbook material as authoritative current documentation.

### Implemented features under-documented or inconsistently documented

- Function/procedure support with call frames
- Array indexing and indexed assignment
- Augmented assignment and increment/decrement
- Docs renderer (`marina-docs`)
- Formatter behavior and limits
- Current VS Code extension scope

## 2. Roadmap Status Report

### Completed

- Core compiler pipeline
- Bytecode VM
- Functions/procedures
- Arrays and indexed assignment
- `CASE`
- Augmented operators
- Formatter MVP
- Docs renderer
- Syntax-only VS Code extension

### Release-candidate ready

- VM correctness, function semantics, and bytecode baseline verified locally
- Documentation alignment and core example curation completed
- Hosted CI confirmation remains external

### Post-Phase-3 planned

- Minimal module architecture
- User-module import system
- Lazy-loading design, if retained for the post-stabilization roadmap

### Deferred

- DB engines and drivers
- HTTP/web work
- Async runtime
- Full IDE feature rollout
- DAP

### Experimental

- `marina-lsp`
- Database surface statements that only stub at runtime

## 3. Phase 3 Completion Assessment

### Does the current work still belong in Phase 3?

Yes, if Phase 3 is defined as runtime and module stabilization.

The repository already has enough implemented language/runtime surface that finishing the stabilization pass is a more realistic priority than adding new domains.

### Phase 3 status after remediation

- Minimal built-in imports are implemented and documented; user modules remain deferred
- Nested `CASE` and `ELSEIF` are implemented and covered by tests
- Variable-scope documentation matches the current runtime limitation
- Database syntax exists without a supported runtime subsystem
- Legacy handbook pages are now explicitly marked as design material

### Scope creep candidates

- `marina-lsp` feature expansion
- `marina-dap`
- Database engine design work
- Macro design work
- Package and ecosystem planning

Recommendation:

Keep those out of the Phase 3 exit gate. They are either deferred or archival design material.

## 4. Recommended Documentation Changes

### Applied in this review

- Rewrote the root `README.md` to be implementation-accurate
- Added root [ARCHITECTURE.md](/Users/dannyfrancisco/codes/marina/ARCHITECTURE.md)
- Added root [ROADMAP.md](/Users/dannyfrancisco/codes/marina/ROADMAP.md)
- Added root [NEXT_PHASE.md](/Users/dannyfrancisco/codes/marina/NEXT_PHASE.md)
- Reframed the documentation index to distinguish authoritative current docs from design/archive material
- Reframed the handbook index the same way
- Replaced the outdated project summary

### Still recommended after this pass

- Treat `docs/handbook/` as design notes unless each page is individually brought in line with implementation
- Remove or quarantine future-facing guides from “getting started” paths
- Add a short release note document when `v1.0-rc1` is cut

## 5. VS Code Extension Review

### Current Support

From `vscode/marina/package.json` and the shipped grammar files, the extension currently provides:

- Language registration for `.prg`
- Syntax highlighting
- Basic language configuration

It does not currently provide:

- LSP integration
- Formatting integration
- Debug integration
- Tasks, commands, or workspace features

### Future Tooling (Post v1)

- Wire the formatter into the extension once formatting behavior is stable enough to support
- Evaluate LSP integration only after the core language surface and diagnostics contract settle
- Evaluate debugger work only after a real source mapping and stepping model exists

LSP implementation should not be treated as a Phase 3 requirement.

## 6. Release Readiness Checklist

### v1.0-rc1 blockers

- [x] Resolve or explicitly defer nested `CASE`
- [x] Resolve or explicitly defer `ELSEIF`
- [x] Publish an accurate statement on scope semantics for `STATIC`, `PRIVATE`, and `PUBLIC`
- [x] Decide whether stub database statements stay exposed in rc1
- [x] Keep root docs authoritative and consistent
- [x] Select a minimal, reliable example set for release validation
- [x] Reduce or accept current compiler warnings intentionally

### v1.0-rc1 required docs

- [x] Root README
- [x] Root architecture document
- [x] Root roadmap
- [x] Root next-phase document
- [x] Release notes / changelog
- [x] Clear statement of supported and unsupported language features

### v1.0-rc1 required tests

- [x] Lexer tests
- [x] Parser tests
- [x] Compiler tests
- [x] VM tests
- [x] Formatter tests
- [x] Docs renderer tests
- [x] Additional regression tests for Phase 3 blockers

### v1.0-rc1 required examples

- [x] A non-interactive arithmetic/control-flow example
- [x] A function call example
- [x] An arrays/indexed assignment example
- [x] A console builtin example
- [x] A documented list of intentionally interactive examples

### v1.0 final blockers beyond rc1

- [x] Clear Phase 3 closure decision on modules/imports
- [x] Warning cleanup or explicit warning policy
- [x] Final pass on example reliability and user-facing docs
- [x] Confidence pass on bytecode stability expectations
