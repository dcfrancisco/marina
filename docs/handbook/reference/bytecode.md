# Bytecode Instruction Set

This is the exact instruction set used in Marina VM v1.0.

## Stack Operations

| Instruction  | Operands | Description              |
| ------------ | -------- | ------------------------ |
| `PUSH_CONST` | idx      | Push constant pool entry |
| `PUSH_NIL`   | —        | Push nil                 |
| `PUSH_TRUE`  | —        | Push true                |
| `PUSH_FALSE` | —        | Push false               |
| `POP`        | —        | Discard top value        |
| `DUP`        | —        | Duplicate top value      |

## Local Variables

| Instruction       | Description            |
| ----------------- | ---------------------- |
| `LOAD_LOCAL idx`  | Push local variable    |
| `STORE_LOCAL idx` | Pop → assign local var |

## Arithmetic

| Instruction | Operation |
| ----------- | --------- |
| `ADD`       | a + b     |
| `SUB`       | a - b     |
| `MUL`       | a * b     |
| `DIV`       | a / b     |
| `MOD`       | a % b     |

## Comparison

| Instruction | Meaning |
| ----------- | ------- |
| `EQ`        | ==      |
| `NE`        | !=      |
| `GT`        | >       |
| `LT`        | <       |
| `GE`        | >=      |
| `LE`        | <=      |

## Logical

| Instruction |
| ----------- |
| `AND`       |
| `OR`        |
| `NOT`       |

## Control Flow

| Instruction         | Description          |
| ------------------- | -------------------- |
| `JMP addr`          | jump                 |
| `JMP_IF_FALSE addr` | conditional jump     |
| `RETURN`            | return from function |

## Functions

| Instruction                    | Meaning                    |
| ------------------------------ | -------------------------- |
| `CALL function_index argc`     | call user-defined function |
| `CALL_BUILTIN builtin_id argc` | call built-in              |
