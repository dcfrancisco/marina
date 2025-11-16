use marina::{
    lexer::Lexer,
    parser::Parser,
    compiler::Compiler,
    vm::VM,
};

fn run_source(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let compiler = Compiler::new();
    let (chunk, functions) = compiler.compile(program)?;
    
    let mut vm = VM::new();
    vm.run(&chunk, functions)?;
    Ok(())
}

#[test]
fn test_case_simple_number() {
    let source = r#"
        LOCAL x := 2
        LOCAL result := 0
        CASE x
            CASE 1
                result := 10
            CASE 2
                result := 20
            CASE 3
                result := 30
        ENDCASE
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_case_with_otherwise() {
    let source = r#"
        LOCAL x := 99
        LOCAL result := 0
        CASE x
            CASE 1
                result := 10
            CASE 2
                result := 20
            OTHERWISE
                result := 999
        ENDCASE
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_case_string() {
    let source = r#"
        LOCAL day := "Monday"
        CASE day
            CASE "Monday"
                LOCAL x := 1
            CASE "Friday"
                LOCAL x := 5
        ENDCASE
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_case_boolean() {
    let source = r#"
        LOCAL flag := TRUE
        CASE flag
            CASE TRUE
                LOCAL x := 1
            CASE FALSE
                LOCAL x := 0
        ENDCASE
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_augmented_plus_assign() {
    let source = "LOCAL x := 10\nx += 5";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_augmented_minus_assign() {
    let source = "LOCAL x := 20\nx -= 7";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_augmented_multiply_assign() {
    let source = "LOCAL x := 3\nx *= 4";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_augmented_divide_assign() {
    let source = "LOCAL x := 100\nx /= 5";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_increment_operator() {
    let source = "LOCAL x := 5\nx++\nx++";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_decrement_operator() {
    let source = "LOCAL x := 10\nx--\nx--";
    assert!(run_source(source).is_ok());
}

#[test]
fn test_increment_in_loop() {
    let source = r#"
        LOCAL i := 0
        WHILE i < 5
            i++
        ENDDO
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_combined_augmented_ops() {
    let source = r#"
        LOCAL x := 100
        x += 50
        x -= 30
        x *= 2
        x /= 4
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
fn test_case_multiple_statements() {
    let source = r#"
        LOCAL x := 1
        CASE x
            CASE 1
                LOCAL a := 10
                LOCAL b := 20
                LOCAL c := 30
            OTHERWISE
                LOCAL d := 99
        ENDCASE
    "#;
    assert!(run_source(source).is_ok());
}

#[test]
#[ignore] // TODO: Nested CASE not yet supported - parser needs lookahead improvement
fn test_nested_case() {
    let source = r#"
        LOCAL x := 1
        LOCAL y := 2
        CASE x
            CASE 1
                CASE y
                    CASE 2
                        LOCAL z := 12
                ENDCASE
        ENDCASE
    "#;
    let result = run_source(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
}
