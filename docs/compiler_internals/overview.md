# Overview

Marina is a modern compiler and virtual machine for the Clipper programming language. This document describes the high-level architecture, main components, and data flow from source code to execution.

- **Source Code** → Lexer → Parser → AST → Compiler → Bytecode → Virtual Machine (VM)

See each section for details on the respective stage.