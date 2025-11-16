# Formal Grammar (EBNF)

This grammar represents the **2025 Core Language**, with room for growth.

```
Program     := (Statement)* EOF

Statement   := VarDecl
             | FunctionDecl
             | ExpressionStmt
             | Block
             | ReturnStmt

Block       := "{" Statement* "}"

VarDecl     := "local" IDENT (":=" Expression)?

FunctionDecl:= "function" IDENT "(" ParamList? ")" Block

ParamList   := IDENT ("," IDENT)*

ReturnStmt  := "return" Expression?

ExpressionStmt := Expression

Expression  := Assignment

Assignment  := OrExpr (":=" Assignment)?

OrExpr      := AndExpr ( "or" AndExpr )*

AndExpr     := Equality ( "and" Equality )*

Equality    := Comparison ( ( "==" | "!=" ) Comparison )*

Comparison  := Term (( ">" | "<" | ">=" | "<=" ) Term)*

Term        := Factor ( ( "+" | "-" ) Factor )*

Factor      := Unary ( ( "*" | "/" | "%" ) Unary )*

Unary       := ( "-" | "not" ) Unary
             | Primary

Primary     := NUMBER
             | STRING
             | "true"
             | "false"
             | "nil"
             | IDENT
             | Codeblock
             | Call
             | "(" Expression ")"

Call        := IDENT "(" ArgList? ")"

ArgList     := Expression ("," Expression)*

Codeblock   := "{|" ParamList? "|" Block? "}" 
```

This grammar will evolve when:

* classes arrive
* lists/maps are added
* async/await is introduced
* match patterns / pipelines are added

But core syntax is stable.
