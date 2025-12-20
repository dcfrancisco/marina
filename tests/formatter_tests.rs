use marina::formatter::{format_source, FormatOptions};

#[test]
fn formats_indentation_and_keywords_preserving_comments() {
    let input = r#"// header
function Factorial(n)
    local result

    if n <= 1
        result := 1
    else
        result := n * Factorial(n - 1)
    endif

return result
"#;

    let expected = r#"// header
function Factorial(n)
    local result

    if n <= 1
        result := 1
    else
        result := n * Factorial(n - 1)
    endif

return result
"#;

    let formatted = format_source(input, FormatOptions { indent_size: 4 });
    assert_eq!(formatted, expected);
}

#[test]
fn formats_case_blocks_with_label_bodies() {
    let input = r#"case grade
case 90
? "A"
otherwise
? "F"
endcase
"#;

    let expected = r#"case grade
    case 90
        ? "A"
    otherwise
        ? "F"
endcase
"#;

    let formatted = format_source(input, FormatOptions { indent_size: 4 });
    assert_eq!(formatted, expected);
}

#[test]
fn formats_do_while_blocks() {
    let input = r#"do
? "x"
while .t.
"#;

    // This project doesn't currently tokenize .t. as TRUE; we only normalize indentation/keywords.
    let expected = r#"do
    ? "x"
while .t.
"#;

    let formatted = format_source(input, FormatOptions { indent_size: 4 });
    assert_eq!(formatted, expected);
}
