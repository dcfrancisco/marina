# Module and scope reference

This is the authoritative Phase 3 module boundary.

## Supported built-in namespaces

| Namespace | Functions |
| --- | --- |
| `console` | `clearscreen`, `setpos`, `devpos`, `gotoxy`, `outstd`, `setcolor`, `setcursor`, `savepos`, `restorepos` |
| `input` | `inkey`, `getinput`, `getsecret` |
| `math` | `abs`, `sqrt`, `round`, `int`, `min`, `max`, `sin`, `cos`, `tan` |
| `string` | `replicate`, `space`, `len`, `substr`, `trim`, `alltrim`, `ltrim`, `rtrim`, `chr`, `asc`, `val`, `str` |
| `system` | `sleep` |

An `IMPORT` is required before a namespaced call:

```clipper
IMPORT "math"
LOCAL root := math.sqrt(81)
```

Imports are static validation of built-in namespaces in Phase 3. There is no
filesystem module loader, package resolver, dynamic loading, or user module
registry yet. General member expressions are not supported; member access is
valid only as the target of a namespaced function call.

## Variable scope in Phase 3

- `LOCAL` uses local storage and is frame-scoped inside functions.
- `STATIC`, `PRIVATE`, and `PUBLIC` currently use the shared global storage path.
  They are accepted for compatibility but do not yet implement distinct
  Clipper visibility or lifetime rules.

