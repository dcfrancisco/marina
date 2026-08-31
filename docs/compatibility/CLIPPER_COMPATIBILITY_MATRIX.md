# Clipper compatibility matrix

This matrix is the executable scope for [ADR-005](../adr/ADR-005-clipper-compatibility-boundary.md).
The checked-in headers are Clipper 5.2 reference material; 5.3 is secondary
until a concrete fixture or specification is recorded.

| Feature family | 5.2 reference | Marina status | Classification | Evidence / next step |
| --- | --- | --- | --- | --- |
| Core expressions and assignment | `STD.CH` conventions | Implemented subset | Supported subset | Lexer/parser/compiler/VM tests |
| `IF`, `ELSEIF`, loops, `CASE` | `STD.CH` block mappings | Implemented subset | Supported subset | Parser/compiler/VM tests |
| Functions, parameters, `RETURN` | `STD.CH` / language core | Implemented | Supported subset | Call-frame and arity tests |
| `LOCAL` | Language core | Implemented | Supported | VM local storage |
| `STATIC`, `PRIVATE`, `PUBLIC` | `CLIPDEFS.H` / language core | Shared global path | Intentional difference | Documented in module reference |
| `?` / `??` output commands | `SIMPLEIO.CH` | Built-in print forms | Modern replacement | Compiler print handling |
| `.CH` includes | `STD.CH`, `SIMPLEIO.CH` | Not parsed | Deferred | WP-012 conformance fixtures |
| `#command`, `#translate` | `STD.CH`, `COMMON.CH` | Not parsed | Deferred | Future preprocessor ADR |
| Macro operator `&` | Clipper macro rules | Not implemented | Deferred | Future macro ADR |
| `@ SAY/GET`, `ACCEPT`, `READ` | `SIMPLEIO.CH`, `STD.CH` | Not implemented | Deferred | Future TUI/preprocessor ADR |
| DBF commands/workarea subset | RDD headers | `USE`, `SKIP`, `DBSKIP()`, navigation, `DBSEEK`, `DBLIST`, `REPLACE` | Supported subset; no full workarea/RDD | ADR-004 / DBF tests |
| Codeblocks | Clipper expression syntax | Not implemented as values | Deferred | ADR-003 / WP-010 |
| Classes/objects | 5.x object extensions | Not implemented | Deferred | Feature-specific ADR required |
| 5.3-only behavior | No local corpus | Unverified | Not claimed | Add fixture before claiming support |

## Fixture policy

Every row classified as supported must have a positive and invalid-input test.
Every intentional difference must have a migration note. No feature is labeled
“Clipper 5.2/5.3 compatible” solely because a matching header exists.
