use marina::{
    lexer::Lexer,
    parser::Parser,
    compiler::Compiler,
    bytecode::OpCode,
};

fn compile_source(source: &str) -> Result<(marina::bytecode::Chunk, std::collections::HashMap<String, usize>), String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let compiler = Compiler::new();
    compiler.compile(program)
}

#[test]
fn test_compile_constant() {
    let (chunk, _) = compile_source("LOCAL x := 42").expect("Compile should succeed");
    
    // Should have constants for NIL and 42
    assert!(chunk.constants.len() >= 1);
    
    // Should have PUSH instruction
    let has_push = chunk.code.iter().any(|inst| inst.opcode == OpCode::Push);
    assert!(has_push);
}

#[test]
fn test_compile_arithmetic() {
    let (chunk, _) = compile_source("x := 10 + 20").expect("Compile should succeed");
    
    // Should have ADD instruction
    let has_add = chunk.code.iter().any(|inst| inst.opcode == OpCode::Add);
    assert!(has_add);
}

#[test]
fn test_compile_if_statement() {
    let (chunk, _) = compile_source("IF x > 10\n? \"Big\"\nENDIF")
        .expect("Compile should succeed");
    
    // Should have comparison and conditional jump
    let has_greater = chunk.code.iter().any(|inst| inst.opcode == OpCode::Greater);
    let has_jump_if_false = chunk.code.iter().any(|inst| inst.opcode == OpCode::JumpIfFalse);
    
    assert!(has_greater);
    assert!(has_jump_if_false);
}

#[test]
fn test_compile_while_loop() {
    let (chunk, _) = compile_source("WHILE x < 10\nx := x + 1\nENDDO")
        .expect("Compile should succeed");
    
    // Should have comparison, conditional jump, and unconditional jump back
    let has_less = chunk.code.iter().any(|inst| inst.opcode == OpCode::Less);
    let has_jump_if_false = chunk.code.iter().any(|inst| inst.opcode == OpCode::JumpIfFalse);
    let has_jump = chunk.code.iter().any(|inst| inst.opcode == OpCode::Jump);
    
    assert!(has_less);
    assert!(has_jump_if_false);
    assert!(has_jump);
}

#[test]
fn test_compile_for_loop() {
    let (chunk, _) = compile_source("FOR i := 1 TO 10\n? i\nNEXT")
        .expect("Compile should succeed");
    
    // Should have SetLocal, GetLocal, LessEqual, Jump
    let has_set_local = chunk.code.iter().any(|inst| inst.opcode == OpCode::SetLocal);
    let has_get_local = chunk.code.iter().any(|inst| inst.opcode == OpCode::GetLocal);
    let has_less_equal = chunk.code.iter().any(|inst| inst.opcode == OpCode::LessEqual);
    
    assert!(has_set_local);
    assert!(has_get_local);
    assert!(has_less_equal);
}

#[test]
fn test_compile_array() {
    let (chunk, _) = compile_source("arr := {1, 2, 3}")
        .expect("Compile should succeed");
    
    // Should have MakeArray instruction
    let has_make_array = chunk.code.iter().any(|inst| inst.opcode == OpCode::MakeArray);
    assert!(has_make_array);
}

#[test]
fn test_compile_array_indexing() {
    let (chunk, _) = compile_source("x := arr[0]")
        .expect("Compile should succeed");
    
    // Should have GetIndex instruction
    let has_get_index = chunk.code.iter().any(|inst| inst.opcode == OpCode::GetIndex);
    assert!(has_get_index);
}

#[test]
fn test_compile_indexed_assignment() {
    let (chunk, _) = compile_source("arr[0] := 99")
        .expect("Compile should succeed");
    
    // Should have SetIndex instruction
    let has_set_index = chunk.code.iter().any(|inst| inst.opcode == OpCode::SetIndex);
    assert!(has_set_index, "SetIndex opcode should be emitted for indexed assignment");
}

#[test]
fn test_compile_exit_in_loop() {
    let (chunk, _) = compile_source("LOOP\nIF x > 10\nEXIT\nENDIF\nENDLOOP")
        .expect("Compile should succeed");
    
    // Should have Jump instructions (for both LOOP and EXIT)
    let jump_count = chunk.code.iter().filter(|inst| inst.opcode == OpCode::Jump).count();
    assert!(jump_count >= 2, "Should have jumps for LOOP and EXIT");
    
    // Should NOT have Halt for EXIT
    let has_halt = chunk.code.iter().any(|inst| inst.opcode == OpCode::Halt);
    // Only final Halt should exist
    assert_eq!(chunk.code.iter().filter(|inst| inst.opcode == OpCode::Halt).count(), 1);
}

#[test]
fn test_compile_print() {
    let (chunk, _) = compile_source("? \"Hello\"")
        .expect("Compile should succeed");
    
    // Should have Print instruction
    let has_print = chunk.code.iter().any(|inst| inst.opcode == OpCode::Print);
    assert!(has_print);
}

#[test]
fn test_compile_function() {
    let (chunk, functions) = compile_source("FUNCTION Add(a, b)\nRETURN a + b")
        .expect("Compile should succeed");
    
    // Should have function registered
    assert!(functions.contains_key("Add"));
    
    // Should have Return instruction
    let has_return = chunk.code.iter().any(|inst| inst.opcode == OpCode::Return);
    assert!(has_return);
}
