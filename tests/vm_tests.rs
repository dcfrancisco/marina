use marina::{
    lexer::Lexer,
    parser::Parser,
    compiler::Compiler,
    vm::VM,
    bytecode::Value,
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
fn test_vm_basic_arithmetic() {
    // Just verify it runs without error
    let result = run_source("LOCAL x := 10 + 20");
    assert!(result.is_ok());
}

#[test]
fn test_vm_string_concatenation() {
    let result = run_source("LOCAL s := \"Hello\" + \" World\"");
    assert!(result.is_ok());
}

#[test]
fn test_vm_if_statement() {
    let result = run_source("IF 5 > 3\nLOCAL x := 1\nENDIF");
    assert!(result.is_ok());
}

#[test]
fn test_vm_while_loop() {
    let result = run_source("LOCAL x := 0\nWHILE x < 3\nx := x + 1\nENDDO");
    assert!(result.is_ok());
}

#[test]
fn test_vm_for_loop() {
    let result = run_source("FOR i := 1 TO 5\nLOCAL x := i\nNEXT");
    assert!(result.is_ok());
}

#[test]
fn test_vm_array_creation() {
    let result = run_source("LOCAL arr := {1, 2, 3}");
    assert!(result.is_ok());
}

#[test]
fn test_vm_array_indexing() {
    let result = run_source("LOCAL arr := {10, 20, 30}\nLOCAL x := arr[0]");
    assert!(result.is_ok());
}

#[test]
fn test_vm_indexed_assignment() {
    let result = run_source("LOCAL arr := {1, 2, 3}\narr[1] := 99");
    assert!(result.is_ok(), "Indexed assignment should execute without error");
}

#[test]
fn test_vm_exit_in_loop() {
    let result = run_source("LOCAL x := 0\nLOOP\nx := x + 1\nIF x > 5\nEXIT\nENDIF\nENDLOOP");
    assert!(result.is_ok(), "EXIT should break out of loop properly");
}

#[test]
fn test_vm_exit_in_while() {
    let result = run_source("LOCAL x := 0\nWHILE TRUE\nx := x + 1\nIF x > 3\nEXIT\nENDIF\nENDDO");
    assert!(result.is_ok(), "EXIT should break out of WHILE loop");
}

#[test]
fn test_vm_exit_in_for() {
    let result = run_source("FOR i := 1 TO 100\nIF i > 5\nEXIT\nENDIF\nNEXT");
    assert!(result.is_ok(), "EXIT should break out of FOR loop");
}

#[test]
fn test_vm_boolean_logic() {
    let result = run_source("IF TRUE AND FALSE\nLOCAL x := 1\nENDIF");
    assert!(result.is_ok());
}

#[test]
fn test_vm_comparison() {
    let result = run_source("IF 10 >= 5\nLOCAL x := 1\nENDIF");
    assert!(result.is_ok());
}

#[test]
fn test_vm_nested_loops() {
    let result = run_source(
        "FOR i := 1 TO 3\nFOR j := 1 TO 3\nLOCAL x := i * j\nNEXT\nNEXT"
    );
    assert!(result.is_ok());
}

#[test]
fn test_vm_do_while() {
    let result = run_source("LOCAL x := 0\nDO\nx := x + 1\nWHILE x < 3");
    assert!(result.is_ok());
}

#[test]
fn test_vm_division() {
    let result = run_source("LOCAL x := 10 / 2");
    assert!(result.is_ok());
}

#[test]
fn test_vm_power() {
    let result = run_source("LOCAL x := 2 ^ 3");
    assert!(result.is_ok());
}

#[test]
fn test_vm_modulo() {
    let result = run_source("LOCAL x := 10 % 3");
    assert!(result.is_ok());
}

#[test]
fn test_vm_string_comparison() {
    let result = run_source("IF \"abc\" == \"abc\"\nLOCAL x := 1\nENDIF");
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
}
