# Supported Features

This is the release-scope reference for the current Phase 3 runtime.

## Implemented

- Numbers, strings, booleans, `NIL`, and arrays
- Variables, assignment, indexed access, and indexed assignment
- `LOCAL`, `STATIC`, `PRIVATE`, and `PUBLIC` declarations
- `IF`/`ELSEIF`/`ELSE`, `WHILE`, `DO WHILE`, `FOR`, `LOOP`, `EXIT`, and `CASE`
- Arithmetic, comparison, logical (`AND`/`OR`/`NOT`), augmented, and increment/decrement operators
- User-defined functions/procedures, multiple arguments, returns, recursion, and `Main()` entrypoint
- Built-in functions and five statically validated namespaces: `console`, `input`, `math`, `string`, and `system`
- Formatter MVP, documentation renderer, and syntax-only VS Code support

## Intentionally limited

- `IMPORT` validates only the five built-in namespaces; there is no filesystem
  loader, user module registry, package resolver, or lazy loading.
- `STATIC`, `PRIVATE`, and `PUBLIC` use shared global storage. They do not yet
  provide distinct Clipper visibility or lifetime semantics.
- Database statements compile to diagnostic no-op stubs. They do not provide
  DBF/CDX persistence, workareas, indexes, or drivers.
- General object/member expressions are unsupported. Member access is valid only
  as the target of a namespaced built-in function call.
- `.ch` preprocessing, `#command`, `#translate`, `#include`, `@ SAY/GET`,
  codeblocks, DAP, and broad LSP integration are deferred.
- Clipper dotted logical spellings such as `.AND.` and `.OR.` are not supported;
  use `AND`/`OR` or `&&`/`||` instead. Interactive examples using unsupported
  legacy syntax are excluded from the non-interactive release check.

## Validation

Run `cargo test --all-targets` and
`./scripts/run_examples_noninteractive.sh` from the repository root.
