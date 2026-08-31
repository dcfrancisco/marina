# ADR-005: Define the Clipper 5.2/5.3 compatibility boundary

- **Status:** Accepted for planning
- **Date:** 2026-08-31
- **Decision owners:** Marina maintainers
- **Related:** [ADR-003](ADR-003-language-feature-evolution.md), [ADR-004](ADR-004-database-feature-boundary.md)
- **Execution package:** [WP-012](../work-packages/WP-012-clipper-compatibility-inventory.md)

## Review findings

The repository includes Clipper-oriented headers, especially `include/STD.CH`,
`SIMPLEIO.CH`, and `COMMON.CH`. These headers reveal compatibility gaps that
are not ordinary parser bugs:

| Area | Current Marina state | Compatibility implication |
| --- | --- | --- |
| `.CH` includes and preprocessor | Not parsed | `#command`, `#translate`, and `#include` programs cannot run |
| `@ SAY/GET`, `ACCEPT`, `READ` | Not implemented | TUI forms require a future preprocessing/input design |
| Macro operator `&` and codeblocks | Not implemented as general language features | Dynamic dispatch and block APIs are unavailable |
| xBase commands/workareas/RDD | Minimal DBF command subset; no full workarea/RDD model | Compatibility is not command-for-command |
| `STATIC`/`PRIVATE`/`PUBLIC` visibility | Accepted through shared globals | Scope and lifetime differ from Clipper |
| Core procedural syntax | Implemented subset | This is the current compatibility baseline |

The checked-in headers are Clipper 5.2 reference material. No equivalent 5.3
conformance corpus is present, so claiming broad 5.3 compatibility would be
unsupported.

## Decision

Marina will target **Clipper 5.2 behavior first for an explicitly selected
compatibility subset**. Clipper 5.3 is a secondary reference only after a
feature has a 5.2 fixture and a documented reason to differ. Modern Marina
syntax and semantics remain valid where they intentionally replace workareas,
commands, implicit macros, and DOS UI.

Compatibility work must proceed as an inventory and conformance effort, not by
adding commands opportunistically. Each supported item must identify its source
header/specification, parser or preprocessor layer, runtime behavior, and
negative cases.

## Out of scope until separately approved

Full `.CH` preprocessing, macro expansion, `@ SAY/GET`, workareas/RDD behavior,
Clipper object/class compatibility, and undocumented 5.3 extensions. These
require separate design decisions and must not be implied by the compatibility
label.

## Consequences

Documentation can state a precise compatibility subset without promising that
legacy Clipper applications run unchanged. The headers remain useful fixtures,
while the modern expression-first core stays small and testable.
