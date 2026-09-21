use aura_lang::lsp::{DiagnosticSeverity, DocumentState, LspPosition, LspServer};

#[test]
fn test_lsp_hover_function_and_docs() {
    let source = r#"
/// Calculates the factorial of an integer
export fn factorial(n: Int): Int {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

let ans = factorial(5);
"#;

    let doc = DocumentState::new("file:///math.aura".to_string(), source.to_string(), 1);
    assert_eq!(doc.diagnostics.len(), 0);

    // Hover over 'factorial' definition (char 14)
    let hover_fn = doc.hover(&LspPosition {
        line: 2,
        character: 14,
    });
    assert!(hover_fn.is_some());
    let content = hover_fn.unwrap().contents;
    assert!(content.contains("fn factorial(n: Int) -> Int"));
    assert!(content.contains("Calculates the factorial of an integer"));

    // Hover over keyword 'fn' (char 8)
    let hover_kw = doc.hover(&LspPosition {
        line: 2,
        character: 8,
    });
    assert!(hover_kw.is_some());
    assert!(hover_kw.unwrap().contents.contains("`fn` keyword"));

    // Hover over primitive type 'Int' (char 25)
    let hover_ty = doc.hover(&LspPosition {
        line: 2,
        character: 25,
    });
    assert!(hover_ty.is_some());
    assert!(hover_ty.unwrap().contents.contains("64-bit signed integer"));
}

#[test]
fn test_lsp_go_to_definition_local_and_functions() {
    let source = r#"
fn compute_total(price: Float, tax_rate: Float): Float {
    let subtotal = price * (1.0 + tax_rate);
    return subtotal;
}

let my_price = 100.0;
let final_total = compute_total(my_price, 0.15);
"#;

    let doc = DocumentState::new("file:///billing.aura".to_string(), source.to_string(), 1);
    assert_eq!(doc.diagnostics.len(), 0);

    // Jump to definition of compute_total from call site (line 7, col 20)
    let def_fn = doc.definition(&LspPosition {
        line: 7,
        character: 20,
    });
    assert!(def_fn.is_some());
    let loc = def_fn.unwrap();
    assert_eq!(loc.uri, "file:///billing.aura");
    assert_eq!(loc.range.start.line, 1);

    // Jump to definition of my_price from call site (line 7, col 35)
    let def_var = doc.definition(&LspPosition {
        line: 7,
        character: 35,
    });
    assert!(def_var.is_some());
    let loc_var = def_var.unwrap();
    assert_eq!(loc_var.range.start.line, 6);
}

#[test]
fn test_lsp_sum_types_and_functions_hover_and_symbols() {
    let source = r#"
type Status = 
    | Active
    | Pending(Int)
    | Suspended(String);

fn checkStatus(status: Status): Bool => {
    match status {
        Active => true,
        _ => false,
    }
}
"#;

    let doc = DocumentState::new("file:///status.aura".to_string(), source.to_string(), 1);
    assert_eq!(doc.diagnostics.len(), 0);

    // Document symbols
    assert!(doc.symbols.iter().any(|s| s.name == "Status"));
    assert!(doc.symbols.iter().any(|s| s.name == "checkStatus"));

    // Hover over function
    let hover_fn = doc.hover(&LspPosition {
        line: 6,
        character: 7,
    });
    assert!(hover_fn.is_some());
    assert!(hover_fn.unwrap().contents.contains("fn checkStatus"));
}

#[test]
fn test_lsp_diagnostics_reporting() {
    // Syntax error test
    let bad_syntax = "fn broken_fn( { let x = 1; }";
    let doc_syntax = DocumentState::new("file:///bad.aura".to_string(), bad_syntax.to_string(), 1);
    assert!(!doc_syntax.diagnostics.is_empty());
    assert_eq!(
        doc_syntax.diagnostics[0].severity,
        DiagnosticSeverity::Error
    );

    // Type error test
    let bad_type = "fn get_num(): Int => \"not a number\";";
    let doc_type = DocumentState::new("file:///bad_type.aura".to_string(), bad_type.to_string(), 1);
    assert!(!doc_type.diagnostics.is_empty());
    assert_eq!(doc_type.diagnostics[0].severity, DiagnosticSeverity::Error);
}

#[test]
fn test_lsp_completions_and_formatting() {
    let source = r#"
fn multiply(a: Int, b: Int): Int => a * b;
let factor = 10;
"#;

    let doc = DocumentState::new("file:///test.aura".to_string(), source.to_string(), 1);
    let comps = doc.completions(&LspPosition {
        line: 2,
        character: 0,
    });

    assert!(comps.iter().any(|c| c.label == "multiply"));
    assert!(comps.iter().any(|c| c.label == "factor"));
    assert!(comps.iter().any(|c| c.label == "match"));
    assert!(comps.iter().any(|c| c.label == "spawn"));

    // Test formatting
    let edits = doc.format(None).expect("Formatting should succeed");
    // Format shouldn't fail
    assert!(edits.len() <= 1);
}

#[test]
fn test_lsp_server_json_rpc_pipeline() {
    let mut server = LspServer::new();

    // 1. Initialize
    let init_req = r#"{"jsonrpc":"2.0","id":100,"method":"initialize","params":{"processId":1234,"capabilities":{}}}"#;
    let init_res = server
        .handle_message(init_req)
        .expect("Expected initialize response");
    assert!(init_res.contains(r#""id":100"#));
    assert!(init_res.contains("hoverProvider"));
    assert!(init_res.contains("definitionProvider"));

    // 2. didOpen with valid file
    let file_uri = "file:///app/src/index.aura";
    let did_open_req = format!(
        r#"{{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{{"uri":"{}","version":1,"text":"export fn add(x: Int, y: Int): Int => x + y;\nlet total = add(1, 2);"}} }}"#,
        file_uri
    );
    let diag_res = server
        .handle_message(&did_open_req)
        .expect("Expected diagnostics notification");
    assert!(diag_res.contains("publishDiagnostics"));

    // 3. Hover query
    let hover_req = format!(
        r#"{{"jsonrpc":"2.0","id":101,"method":"textDocument/hover","params":{{"uri":"{}","line":0,"character":11}}}}"#,
        file_uri
    );
    let hover_res = server
        .handle_message(&hover_req)
        .expect("Expected hover response");
    assert!(hover_res.contains("fn add(x: Int, y: Int) -> Int"));

    // 4. Definition query
    let def_req = format!(
        r#"{{"jsonrpc":"2.0","id":102,"method":"textDocument/definition","params":{{"uri":"{}","line":1,"character":13}}}}"#,
        file_uri
    );
    let def_res = server
        .handle_message(&def_req)
        .expect("Expected definition response");
    assert!(def_res.contains(file_uri));

    // 5. Document symbols query
    let sym_req = format!(
        r#"{{"jsonrpc":"2.0","id":103,"method":"textDocument/documentSymbol","params":{{"uri":"{}"}}}}"#,
        file_uri
    );
    let sym_res = server
        .handle_message(&sym_req)
        .expect("Expected documentSymbol response");
    assert!(sym_res.contains("add"));

    // 6. Formatting query
    let fmt_req = format!(
        r#"{{"jsonrpc":"2.0","id":104,"method":"textDocument/formatting","params":{{"uri":"{}"}}}}"#,
        file_uri
    );
    let fmt_res = server
        .handle_message(&fmt_req)
        .expect("Expected formatting response");
    assert!(fmt_res.contains("newText"));

    // 7. Shutdown
    let shutdown_req = r#"{"jsonrpc":"2.0","id":105,"method":"shutdown","params":{}}"#;
    let shutdown_res = server
        .handle_message(shutdown_req)
        .expect("Expected shutdown response");
    assert!(shutdown_res.contains("null"));
    assert!(server.is_shutdown);
}
