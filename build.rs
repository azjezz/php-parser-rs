use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests = manifest.join("tests");
    let snapshot = manifest.join("bin").join("snapshot.rs");

    println!("cargo:rerun-if-changed={}", tests.to_string_lossy());
    println!("cargo:rerun-if-changed={}", snapshot.to_string_lossy());
    println!("cargo:rerun-if-env-changed=BUILD_INTEGRATION_TESTS");

    if env::var("BUILD_INTEGRATION_TESTS").unwrap_or_else(|_| "0".to_string()) == "0" {
        return;
    }

    let mut entries = read_dir(tests)
        .unwrap()
        .flatten()
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    let mut content = String::new();
    content.push_str("/// this file is auto-generated by the build script.\n");
    content.push_str("/// you should never manually change it.\n\n");
    content.push_str("use php_parser_rs::prelude::{Lexer, Parser};\n");
    content.push_str("use pretty_assertions::assert_str_eq;\n\n");
    content.push_str("static PARSER: Parser = Parser::new();\n");
    content.push_str("static LEXER: Lexer = Lexer::new();\n\n");

    for entry in entries {
        let code_filename = entry.join("code.php");
        let ast_filename = entry.join("ast.txt");
        let tokens_filename = entry.join("tokens.txt");
        let lexer_error_filename = entry.join("lexer-error.txt");
        let parser_error_filename = entry.join("parser-error.txt");

        if !code_filename.exists() {
            continue;
        }

        if ast_filename.exists() {
            assert!(
                !lexer_error_filename.exists(),
                "`lexer-error.txt` was not expected for `{}`.",
                entry.to_string_lossy()
            );
            assert!(
                !parser_error_filename.exists(),
                "`parser-error.txt` was not expected for `{}`.",
                entry.to_string_lossy()
            );

            content.push_str(&build_success_test(
                entry,
                code_filename,
                ast_filename,
                tokens_filename,
            ))
        } else if lexer_error_filename.exists() {
            assert!(
                !parser_error_filename.exists(),
                "`parser-error.txt` was not expected for `{}`.",
                entry.to_string_lossy()
            );

            content.push_str(&build_lexer_error_test(
                entry,
                code_filename,
                lexer_error_filename,
            ))
        } else {
            assert!(
                parser_error_filename.exists(),
                "unable to find `parser-error.txt` for `{}`.",
                entry.to_string_lossy()
            );

            content.push_str(&build_parser_error_test(
                entry,
                code_filename,
                parser_error_filename,
                tokens_filename,
            ))
        }
    }

    let dest = manifest.join("tests").join("integration_test.rs");
    std::fs::write(dest, content).expect("failed to write to file");
}

fn build_success_test(
    entry: PathBuf,
    code_filename: PathBuf,
    ast_filename: PathBuf,
    tokens_filename: PathBuf,
) -> String {
    format!(
        r#"#[test]
fn test_success_{}() {{
    let code_filename = "{}";
    let ast_filename = "{}";
    let tokens_filename = "{}";

    let code = std::fs::read_to_string(&code_filename).unwrap();
    let expected_ast = std::fs::read_to_string(&ast_filename).unwrap();
    let expected_tokens = std::fs::read_to_string(&tokens_filename).unwrap();

    let tokens = LEXER.tokenize(code.as_bytes()).unwrap();

    assert_str_eq!(expected_tokens.trim(), format!("{{:#?}}", tokens));

    let ast = PARSER.parse(tokens).unwrap();

    assert_str_eq!(expected_ast.trim(), format!("{{:#?}}", ast));
}}

"#,
        entry.file_name().unwrap().to_string_lossy(),
        code_filename.to_string_lossy(),
        ast_filename.to_string_lossy(),
        tokens_filename.to_string_lossy(),
    )
}

fn build_lexer_error_test(
    entry: PathBuf,
    code_filename: PathBuf,
    lexer_error_filename: PathBuf,
) -> String {
    format!(
        r#"#[test]
fn test_lexer_error_{}() {{
    let code_filename = "{}";
    let lexer_error_filename = "{}";

    let code = std::fs::read_to_string(&code_filename).unwrap();
    let expected_error = std::fs::read_to_string(&lexer_error_filename).unwrap();

    let error = LEXER.tokenize(code.as_bytes()).err().unwrap();

    assert_str_eq!(
        expected_error.trim(),
        format!("{{:?}} -> {{}}", error, error.to_string())
    );
}}

"#,
        entry.file_name().unwrap().to_string_lossy(),
        code_filename.to_string_lossy(),
        lexer_error_filename.to_string_lossy()
    )
}

fn build_parser_error_test(
    entry: PathBuf,
    code_filename: PathBuf,
    parser_error_filename: PathBuf,
    tokens_filename: PathBuf,
) -> String {
    format!(
        r#"#[test]
fn test_paser_error_{}() {{
    let code_filename = "{}";
    let tokens_filename = "{}";
    let parser_error_filename = "{}";

    let code = std::fs::read_to_string(&code_filename).unwrap();
    let expected_tokens = std::fs::read_to_string(&tokens_filename).unwrap();
    let expected_error = std::fs::read_to_string(&parser_error_filename).unwrap();

    let tokens = LEXER.tokenize(code.as_bytes()).unwrap();

    assert_str_eq!(expected_tokens.trim(), format!("{{:#?}}", tokens));

    let error = PARSER.parse(tokens).err().unwrap();

    assert_str_eq!(
        expected_error.trim(),
        format!("{{:?}} -> {{}}", error, error.to_string()),
    );
}}

"#,
        entry.file_name().unwrap().to_string_lossy(),
        code_filename.to_string_lossy(),
        tokens_filename.to_string_lossy(),
        parser_error_filename.to_string_lossy()
    )
}
