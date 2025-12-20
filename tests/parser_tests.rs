use marina::{
    lexer::Lexer,
    parser::Parser,
    diagnostics::Severity,
    ast::{Stmt, Expr, BinaryOp, VarScope},
};

fn parse_source(source: &str) -> Result<Vec<Stmt>, String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    Ok(program.statements)
}

fn parse_source_err(source: &str) -> String {
    match parse_source(source) {
        Ok(stmts) => panic!("Expected parse to fail, but succeeded with {} statements", stmts.len()),
        Err(e) => e,
    }
}

#[test]
fn test_parse_with_diagnostics_collects_multiple_errors() {
    // Two independent parse errors, separated by semicolons so the parser can recover.
    let source = "LOCAL x := ; LOCAL y := ;";

    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);

    let result = parser.parse_with_diagnostics();

    assert!(result.program.statements.is_empty());
    assert!(
        result.diagnostics.len() >= 2,
        "Expected multiple diagnostics, got {}: {:?}",
        result.diagnostics.len(),
        result.diagnostics
    );
    assert!(result.diagnostics.iter().all(|d| d.severity == Severity::Error));
    assert!(result.diagnostics.iter().all(|d| d.span.line == 1));
    assert!(result.diagnostics.iter().all(|d| d.span.column >= 1));
    assert!(result.diagnostics.iter().all(|d| !d.message.is_empty()));
}

#[test]
fn test_variable_declaration() {
    let stmts = parse_source("LOCAL x := 10").expect("Parse should succeed");
    
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::VarDecl { name, initializer, scope } => {
            assert_eq!(name, "x");
            assert_eq!(scope, &VarScope::Local);
            assert!(initializer.is_some());
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_multiple_declarations() {
    let stmts = parse_source("LOCAL x, y, z").expect("Parse should succeed");
    
    // Should be wrapped in a Block
    match &stmts[0] {
        Stmt::Block(decls) => {
            assert_eq!(decls.len(), 3);
        }
        Stmt::VarDecl { .. } => {
            // Single declaration is ok too
            assert_eq!(stmts.len(), 1);
        }
        _ => panic!("Expected Block or VarDecl"),
    }
}

#[test]
fn test_if_statement() {
    let stmts = parse_source("IF x > 10\n? \"Big\"\nENDIF").expect("Parse should succeed");
    
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::If { condition, then_branch, else_branch } => {
            assert!(matches!(condition, Expr::Binary { .. }));
            assert_eq!(then_branch.len(), 1);
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_if_else_statement() {
    let stmts = parse_source("IF x > 10\n? \"Big\"\nELSE\n? \"Small\"\nENDIF")
        .expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::If { condition, then_branch, else_branch } => {
            assert!(matches!(condition, Expr::Binary { .. }));
            assert_eq!(then_branch.len(), 1);
            assert!(else_branch.is_some());
            assert_eq!(else_branch.as_ref().unwrap().len(), 1);
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_while_loop() {
    let stmts = parse_source("WHILE x < 10\nx := x + 1\nENDDO")
        .expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::While { condition, body } => {
            assert!(matches!(condition, Expr::Binary { .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected While statement"),
    }
}

#[test]
fn test_for_loop() {
    let stmts = parse_source("FOR i := 1 TO 10\n? i\nNEXT")
        .expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::For { variable, start, end, step, body } => {
            assert_eq!(variable, "i");
            assert!(matches!(start, Expr::Number(_)));
            assert!(matches!(end, Expr::Number(_)));
            assert!(step.is_none());
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected For statement"),
    }
}

#[test]
fn test_for_loop_with_step() {
    let stmts = parse_source("FOR i := 1 TO 10 STEP 2\n? i\nNEXT")
        .expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::For { variable, start, end, step, body } => {
            assert_eq!(variable, "i");
            assert!(step.is_some());
        }
        _ => panic!("Expected For statement"),
    }
}

#[test]
fn test_binary_expression() {
    let stmts = parse_source("x := 10 + 20").expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::Expression(Expr::Assign { name, value }) => {
            assert_eq!(name, "x");
            match value.as_ref() {
                Expr::Binary { left, operator, right } => {
                    assert!(matches!(left.as_ref(), Expr::Number(10.0)));
                    assert_eq!(operator, &BinaryOp::Add);
                    assert!(matches!(right.as_ref(), Expr::Number(20.0)));
                }
                _ => panic!("Expected Binary expression"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_array_literal() {
    let stmts = parse_source("arr := {1, 2, 3}").expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::Expression(Expr::Assign { value, .. }) => {
            match value.as_ref() {
                Expr::Array(elements) => {
                    assert_eq!(elements.len(), 3);
                }
                _ => panic!("Expected Array"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_array_indexing() {
    let stmts = parse_source("x := arr[0]").expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::Expression(Expr::Assign { value, .. }) => {
            assert!(matches!(value.as_ref(), Expr::Index { .. }));
        }
        _ => panic!("Expected assignment with index"),
    }
}

#[test]
fn test_function_call() {
    let stmts = parse_source("Print(\"Hello\")").expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::Expression(Expr::Call { name, args }) => {
            assert_eq!(name, "Print");
            assert_eq!(args.len(), 1);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_print_shorthand() {
    let stmts = parse_source("? \"Hello\", x").expect("Parse should succeed");
    
    match &stmts[0] {
        Stmt::Expression(Expr::Call { name, args }) => {
            assert_eq!(name, "?");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected print call"),
    }
}

#[test]
fn test_unterminated_if_errors_with_location() {
    let err = parse_source_err("IF x > 10\n? \"Big\"");
    assert!(err.contains("Unterminated IF"), "{err}");
    assert!(err.contains("line 1"), "{err}");
    assert!(err.contains("column 1"), "{err}");
}

#[test]
fn test_unterminated_while_errors_with_location() {
    let err = parse_source_err("WHILE x < 10\nx := x + 1");
    assert!(err.contains("Unterminated WHILE"), "{err}");
    assert!(err.contains("line 1"), "{err}");
    assert!(err.contains("column 1"), "{err}");
}

#[test]
fn test_unterminated_for_errors_with_location() {
    let err = parse_source_err("FOR i := 1 TO 10\n? i");
    assert!(err.contains("Unterminated FOR"), "{err}");
    assert!(err.contains("line 1"), "{err}");
    assert!(err.contains("column 1"), "{err}");
}

#[test]
fn test_unterminated_loop_errors_with_location() {
    let err = parse_source_err("LOOP\n? 1");
    assert!(err.contains("Unterminated LOOP"), "{err}");
    assert!(err.contains("line 1"), "{err}");
    assert!(err.contains("column 1"), "{err}");
}

#[test]
fn test_unterminated_case_errors_with_location() {
    let err = parse_source_err("CASE x\nCASE 1\n? \"one\"");
    assert!(err.contains("Unterminated CASE"), "{err}");
    assert!(err.contains("line 1"), "{err}");
    assert!(err.contains("column 1"), "{err}");
}
