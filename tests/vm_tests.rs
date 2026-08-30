use marina::{compiler::Compiler, lexer::Lexer, parser::Parser, vm::VM};

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

fn run_source_with_vm(source: &str) -> Result<VM, String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let compiler = Compiler::new();
    let (chunk, functions) = compiler.compile(program)?;
    let mut vm = VM::new();
    vm.run(&chunk, functions)?;
    Ok(vm)
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
    assert!(
        result.is_ok(),
        "Indexed assignment should execute without error"
    );
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
    let result = run_source("FOR i := 1 TO 3\nFOR j := 1 TO 3\nLOCAL x := i * j\nNEXT\nNEXT");
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

#[test]
fn test_vm_user_defined_function_call() {
    let result = run_source(
        "FUNCTION Add(a, b)\nRETURN a + b\n\nLOCAL total := Add(10, 20)",
    );
    assert!(result.is_ok(), "Function calls should execute without error");
}

#[test]
fn test_vm_nested_function_calls() {
    let result = run_source(
        "FUNCTION Add(a, b)\nRETURN a + b\n\nFUNCTION Twice(x)\nRETURN Add(x, x)\n\nLOCAL total := Twice(21)",
    );
    assert!(
        result.is_ok(),
        "Nested user-defined function calls should execute without error"
    );
}

#[test]
fn test_vm_recursive_function_call() {
    let result = run_source(
        "FUNCTION Fact(n)\nIF n <= 1\nRETURN 1\nENDIF\nRETURN n * Fact(n - 1)\n\nLOCAL value := Fact(5)",
    );
    assert!(
        result.is_ok(),
        "Recursive user-defined function calls should execute without error"
    );
}

#[test]
fn test_vm_main_entrypoint_execution() {
    let result = run_source(
        "LOCAL initialized := 1\n\nFUNCTION Main()\nLOCAL total := 40 + 2\nRETURN total",
    );
    assert!(
        result.is_ok(),
        "Programs with Main() entrypoint should execute without error"
    );
}

#[test]
fn test_vm_static_private_public_access_from_function() {
    let result = run_source(
        "STATIC s := 10\nPRIVATE p := 20\nPUBLIC g := 30\n\nFUNCTION Total()\nRETURN s + p + g\n\nLOCAL total := Total()",
    );
    assert!(
        result.is_ok(),
        "STATIC, PRIVATE, and PUBLIC should currently be accessible through the shared global path"
    );
}

#[test]
fn test_vm_local_shadows_global_like_current_compiler_model() {
    let result = run_source(
        "PUBLIC value := 10\n\nFUNCTION Demo()\nLOCAL value := 20\nRETURN value\n\nLOCAL result := Demo()",
    );
    assert!(
        result.is_ok(),
        "LOCAL variables should continue to use local storage even when a same-named global exists"
    );
}

#[test]
fn test_vm_imported_string_math_and_system_modules() {
    let result = run_source(
        "IMPORT \"string\"\nIMPORT \"math\"\nIMPORT \"system\"\nLOCAL size := string.len(\"abcd\")\nLOCAL root := math.sqrt(81)\nsystem.sleep(0)",
    );
    assert!(
        result.is_ok(),
        "Imported built-in modules should execute through namespaced calls"
    );
}

#[test]
fn test_vm_main_cleans_call_frame_and_locals() {
    let vm = run_source_with_vm(
        "FUNCTION Main()\nLOCAL temporary := 42\nRETURN temporary",
    )
    .expect("Main should execute");
    assert_eq!(vm.call_depth(), 0, "Main frame must be released");
    assert_eq!(vm.local_count(), 0, "Main locals must be released");
    assert!(vm.stack_snapshot().is_empty(), "return value must not leak");
}

#[test]
fn test_vm_runtime_errors_are_reported() {
    let division = run_source("LOCAL value := 1 / 0");
    assert!(division.unwrap_err().contains("Division by zero"));

    let index = run_source("LOCAL values := {1}\nLOCAL value := values[2]");
    assert!(index.unwrap_err().contains("out of bounds"));
}

#[test]
fn test_vm_unknown_function_error_is_deterministic() {
    let err = run_source("LOCAL value := DefinitelyMissing(1)")
        .expect_err("unknown functions must fail");
    assert_eq!(err, "Unknown function: DefinitelyMissing");
}
