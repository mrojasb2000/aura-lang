//! Aura Language Server Protocol (LSP) Engine
//!
//! Implements JSON-RPC 2.0 communication, document synchronization,
//! diagnostics publishing, Hover information, Go to Definition,
//! Contextual Completions, Document Symbols, and Code Formatting.

use crate::ast::*;
use crate::formatter::{FormatConfig, format_aura};
use crate::lexer::{Lexer, Span, TokenKind};
use crate::parser::Parser;
use crate::typechecker::{ConcreteType, TypeChecker};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Represents a 0-indexed position in a text document (LSP standard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LspPosition {
    pub line: u32,
    pub character: u32,
}

/// Represents a text range with start and end positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LspRange {
    pub start: LspPosition,
    pub end: LspPosition,
}

impl LspRange {
    pub fn new(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Self {
        Self {
            start: LspPosition {
                line: start_line,
                character: start_char,
            },
            end: LspPosition {
                line: end_line,
                character: end_char,
            },
        }
    }

    pub fn from_span(span: &Span, length: usize) -> Self {
        let line = if span.line > 0 {
            (span.line - 1) as u32
        } else {
            0
        };
        let char_start = if span.column > 0 {
            (span.column - 1) as u32
        } else {
            0
        };
        let char_end = char_start + (length as u32);
        Self::new(line, char_start, line, char_end)
    }

    pub fn contains(&self, pos: &LspPosition) -> bool {
        if pos.line < self.start.line || pos.line > self.end.line {
            return false;
        }
        if pos.line == self.start.line && pos.character < self.start.character {
            return false;
        }
        if pos.line == self.end.line && pos.character > self.end.character {
            return false;
        }
        true
    }
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A diagnostic message (error/warning) reported to the client editor.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub range: LspRange,
    pub severity: DiagnosticSeverity,
    pub source: String,
    pub message: String,
}

/// Symbol kind for LSP document symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/// Document symbol definition for outline view.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: LspRange,
    pub selection_range: LspRange,
    pub children: Vec<DocumentSymbol>,
}

/// A hover response card.
#[derive(Debug, Clone, PartialEq)]
pub struct HoverResult {
    pub contents: String,
    pub range: Option<LspRange>,
}

/// A location for Go to Definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub uri: String,
    pub range: LspRange,
}

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

/// Completion item.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

/// Text edit for formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct TextEdit {
    pub range: LspRange,
    pub new_text: String,
}

/// Definition information recorded during symbol indexing.
#[derive(Debug, Clone)]
pub struct SymbolDef {
    pub name: String,
    pub kind_name: String,
    pub type_str: String,
    pub doc_comment: Option<String>,
    pub def_range: LspRange,
    pub uri: String,
}

/// Identifier reference in source code.
#[derive(Debug, Clone)]
pub struct SymbolRef {
    pub name: String,
    pub range: LspRange,
    pub def_name: String,
}

/// Analysis state for an open Aura document.
pub struct DocumentState {
    pub uri: String,
    pub source: String,
    pub version: i64,
    pub lines: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
    pub symbols: Vec<DocumentSymbol>,
    pub definitions: HashMap<String, SymbolDef>,
    pub references: Vec<SymbolRef>,
    pub type_env: HashMap<String, ConcreteType>,
}

impl DocumentState {
    pub fn new(uri: String, source: String, version: i64) -> Self {
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let mut state = Self {
            uri,
            source,
            version,
            lines,
            diagnostics: Vec::new(),
            symbols: Vec::new(),
            definitions: HashMap::new(),
            references: Vec::new(),
            type_env: HashMap::new(),
        };
        state.analyze();
        state
    }

    pub fn update(&mut self, source: String, version: i64) {
        self.source = source;
        self.lines = self.source.lines().map(|s| s.to_string()).collect();
        self.version = version;
        self.analyze();
    }

    /// Performs lexical, syntactic, and semantic indexing of the document.
    pub fn analyze(&mut self) {
        self.diagnostics.clear();
        self.symbols.clear();
        self.definitions.clear();
        self.references.clear();
        self.type_env.clear();

        // 1. Lexical Analysis
        let mut lexer = Lexer::new(&self.source);
        let tokens = match lexer.tokenize() {
            Ok(toks) => toks,
            Err(lex_err) => {
                let range = parse_error_range(&lex_err, &self.lines);
                self.diagnostics.push(Diagnostic {
                    range,
                    severity: DiagnosticSeverity::Error,
                    source: "aura-lexer".to_string(),
                    message: lex_err,
                });
                return;
            }
        };

        // 2. Syntax Parsing
        let mut parser = Parser::new(tokens.clone());
        let module = match parser.parse_module() {
            Ok(m) => m,
            Err(parse_err) => {
                let range = parse_error_range(&parse_err, &self.lines);
                self.diagnostics.push(Diagnostic {
                    range,
                    severity: DiagnosticSeverity::Error,
                    source: "aura-parser".to_string(),
                    message: parse_err,
                });
                return;
            }
        };

        // 3. Type Checking
        let mut typechecker = TypeChecker::new();
        if let Err(type_err) = typechecker.check_module(&module) {
            let range = parse_error_range(&type_err, &self.lines);
            self.diagnostics.push(Diagnostic {
                range,
                severity: DiagnosticSeverity::Error,
                source: "aura-typechecker".to_string(),
                message: type_err,
            });
        }

        // Store inferred/declared types from environment
        for (var_name, var_type) in typechecker.env().all_variables() {
            self.type_env.insert(var_name.clone(), var_type.clone());
        }

        // 4. Extract symbols, definitions, and references
        self.index_symbols_and_definitions(&module, &tokens);
    }

    fn index_symbols_and_definitions(&mut self, module: &Module, tokens: &[crate::lexer::Token]) {
        // Collect comments above definitions
        let doc_comments = extract_doc_comments(&self.source);

        // Find token positions for fast range resolution
        let mut ident_spans: Vec<(&str, LspRange)> = Vec::new();
        for tok in tokens {
            if let TokenKind::Ident(ref name) = tok.kind {
                let r = LspRange::from_span(&tok.span, name.len());
                ident_spans.push((name.as_str(), r));
            }
        }

        for item in &module.items {
            match item {
                Item::Function(f) => {
                    let param_types: Vec<String> = f
                        .params
                        .iter()
                        .map(|p| {
                            let ty_str = p
                                .type_annotation
                                .as_ref()
                                .map(|t| format!("{}", t))
                                .unwrap_or_else(|| "Any".to_string());
                            format!("{}: {}", p.name, ty_str)
                        })
                        .collect();
                    let ret_str = f
                        .return_type
                        .as_ref()
                        .map(|t| format!(" -> {}", t))
                        .unwrap_or_default();
                    let recv_prefix = if let Some(ref r) = f.receiver {
                        format!("({}) ", r)
                    } else {
                        String::new()
                    };
                    let sig = format!(
                        "fn {}{}({}){}",
                        recv_prefix,
                        f.name,
                        param_types.join(", "),
                        ret_str
                    );

                    let fn_range = find_ident_range(&ident_spans, &f.name, 0);
                    let doc = doc_comments.get(&f.name).cloned();

                    self.definitions.insert(
                        f.name.clone(),
                        SymbolDef {
                            name: f.name.clone(),
                            kind_name: "function".to_string(),
                            type_str: sig.clone(),
                            doc_comment: doc.clone(),
                            def_range: fn_range,
                            uri: self.uri.clone(),
                        },
                    );

                    // Index parameters
                    for p in &f.params {
                        let p_ty = p
                            .type_annotation
                            .as_ref()
                            .map(|t| format!("{}", t))
                            .unwrap_or_else(|| "Any".to_string());
                        let p_range = find_ident_range(&ident_spans, &p.name, fn_range.start.line);
                        let p_key = format!("{}::{}", f.name, p.name);
                        self.definitions.insert(
                            p_key.clone(),
                            SymbolDef {
                                name: p.name.clone(),
                                kind_name: "parameter".to_string(),
                                type_str: format!("{}: {}", p.name, p_ty),
                                doc_comment: None,
                                def_range: p_range,
                                uri: self.uri.clone(),
                            },
                        );
                        self.definitions.insert(
                            p.name.clone(),
                            SymbolDef {
                                name: p.name.clone(),
                                kind_name: "parameter".to_string(),
                                type_str: format!("{}: {}", p.name, p_ty),
                                doc_comment: None,
                                def_range: p_range,
                                uri: self.uri.clone(),
                            },
                        );
                    }

                    self.symbols.push(DocumentSymbol {
                        name: f.name.clone(),
                        detail: Some(sig),
                        kind: SymbolKind::Function,
                        range: fn_range,
                        selection_range: fn_range,
                        children: Vec::new(),
                    });
                }
                Item::SumType(s) => {
                    let s_range = find_ident_range(&ident_spans, &s.name, 0);
                    let doc = doc_comments.get(&s.name).cloned();
                    let var_names: Vec<String> =
                        s.variants.iter().map(|v| v.name.clone()).collect();
                    let sig = format!("type {} = {}", s.name, var_names.join(" | "));

                    self.definitions.insert(
                        s.name.clone(),
                        SymbolDef {
                            name: s.name.clone(),
                            kind_name: "enum".to_string(),
                            type_str: sig.clone(),
                            doc_comment: doc,
                            def_range: s_range,
                            uri: self.uri.clone(),
                        },
                    );

                    let mut var_symbols = Vec::new();
                    for v in &s.variants {
                        let v_range = find_ident_range(&ident_spans, &v.name, s_range.start.line);
                        let v_sig = format!("variant {}::{}", s.name, v.name);
                        self.definitions.insert(
                            v.name.clone(),
                            SymbolDef {
                                name: v.name.clone(),
                                kind_name: "variant".to_string(),
                                type_str: v_sig.clone(),
                                doc_comment: None,
                                def_range: v_range,
                                uri: self.uri.clone(),
                            },
                        );
                        var_symbols.push(DocumentSymbol {
                            name: v.name.clone(),
                            detail: Some(v_sig),
                            kind: SymbolKind::EnumMember,
                            range: v_range,
                            selection_range: v_range,
                            children: Vec::new(),
                        });
                    }

                    self.symbols.push(DocumentSymbol {
                        name: s.name.clone(),
                        detail: Some(sig),
                        kind: SymbolKind::Enum,
                        range: s_range,
                        selection_range: s_range,
                        children: var_symbols,
                    });
                }
                Item::TypeAlias(t) => {
                    let t_range = find_ident_range(&ident_spans, &t.name, 0);
                    let sig = format!("type {} = {}", t.name, t.target);
                    let doc = doc_comments.get(&t.name).cloned();

                    self.definitions.insert(
                        t.name.clone(),
                        SymbolDef {
                            name: t.name.clone(),
                            kind_name: "type alias".to_string(),
                            type_str: sig.clone(),
                            doc_comment: doc,
                            def_range: t_range,
                            uri: self.uri.clone(),
                        },
                    );

                    self.symbols.push(DocumentSymbol {
                        name: t.name.clone(),
                        detail: Some(sig),
                        kind: SymbolKind::Interface,
                        range: t_range,
                        selection_range: t_range,
                        children: Vec::new(),
                    });
                }
                Item::Import(imp) => {
                    for it in &imp.items {
                        let it_name = it.alias.as_ref().unwrap_or(&it.name);
                        let it_range = find_ident_range(&ident_spans, it_name, 0);
                        self.definitions.insert(
                            it_name.clone(),
                            SymbolDef {
                                name: it_name.clone(),
                                kind_name: "import".to_string(),
                                type_str: format!("import {} from '{}'", it_name, imp.source),
                                doc_comment: None,
                                def_range: it_range,
                                uri: self.uri.clone(),
                            },
                        );
                    }
                }
                Item::Statement(Statement::Let {
                    name,
                    is_mut: _,
                    type_annotation,
                    value: _,
                }) => {
                    let v_range = find_ident_range(&ident_spans, name, 0);
                    let ty_str = if let Some(t) = type_annotation {
                        format!("{}", t)
                    } else if let Some(conc_ty) = self.type_env.get(name) {
                        format!("{}", conc_ty)
                    } else {
                        "Any".to_string()
                    };
                    let sig = format!("let {}: {}", name, ty_str);

                    self.definitions.insert(
                        name.clone(),
                        SymbolDef {
                            name: name.clone(),
                            kind_name: "variable".to_string(),
                            type_str: sig.clone(),
                            doc_comment: None,
                            def_range: v_range,
                            uri: self.uri.clone(),
                        },
                    );

                    self.symbols.push(DocumentSymbol {
                        name: name.clone(),
                        detail: Some(sig),
                        kind: SymbolKind::Variable,
                        range: v_range,
                        selection_range: v_range,
                        children: Vec::new(),
                    });
                }

                _ => {}
            }
        }

        // Map all identifier tokens to symbol references
        for (ident, range) in &ident_spans {
            if self.definitions.contains_key(*ident) {
                self.references.push(SymbolRef {
                    name: ident.to_string(),
                    range: *range,
                    def_name: ident.to_string(),
                });
            }
        }
    }

    /// Provides hover information for a position in the document.
    pub fn hover(&self, pos: &LspPosition) -> Option<HoverResult> {
        // 1. Check if hovering over a reference or definition
        for s_ref in &self.references {
            if s_ref.range.contains(pos) {
                if let Some(def) = self.definitions.get(&s_ref.def_name) {
                    let mut md = format!("```aura\n{}\n```", def.type_str);
                    if let Some(ref doc) = def.doc_comment {
                        md.push_str("\n\n---\n");
                        md.push_str(doc);
                    }
                    return Some(HoverResult {
                        contents: md,
                        range: Some(s_ref.range),
                    });
                }
            }
        }

        // 2. Check if hovering over standard keywords or built-ins
        let word = self.word_at_position(pos)?;
        if let Some(keyword_hover) = get_keyword_hover(&word.0) {
            return Some(HoverResult {
                contents: keyword_hover,
                range: Some(word.1),
            });
        }

        // 3. Check if hovering over built-in types or standard library
        if let Some(builtin_hover) = get_builtin_hover(&word.0) {
            return Some(HoverResult {
                contents: builtin_hover,
                range: Some(word.1),
            });
        }

        None
    }

    /// Resolves Go to Definition for a position in the document.
    pub fn definition(&self, pos: &LspPosition) -> Option<Location> {
        for s_ref in &self.references {
            if s_ref.range.contains(pos) {
                if let Some(def) = self.definitions.get(&s_ref.def_name) {
                    return Some(Location {
                        uri: def.uri.clone(),
                        range: def.def_range,
                    });
                }
            }
        }

        // Check if cursor is directly on a definition symbol
        for def in self.definitions.values() {
            if def.def_range.contains(pos) {
                return Some(Location {
                    uri: def.uri.clone(),
                    range: def.def_range,
                });
            }
        }

        None
    }

    /// Provides completions at a position in the document.
    pub fn completions(&self, _pos: &LspPosition) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // 1. In-scope definitions
        for def in self.definitions.values() {
            let kind = match def.kind_name.as_str() {
                "function" => CompletionItemKind::Function,
                "enum" => CompletionItemKind::Enum,
                "variant" => CompletionItemKind::EnumMember,
                "type alias" => CompletionItemKind::Interface,
                "parameter" => CompletionItemKind::Variable,
                _ => CompletionItemKind::Variable,
            };

            items.push(CompletionItem {
                label: def.name.clone(),
                kind,
                detail: Some(def.type_str.clone()),
                documentation: def.doc_comment.clone(),
                insert_text: None,
            });
        }

        // 2. Aura Language Keywords
        let keywords = [
            (
                "fn",
                "Declare a function",
                "fn ${1:name}(${2:params}) -> ${3:Type} {\n\t$0\n}",
            ),
            ("let", "Declare a variable", "let ${1:name} = ${2:value};"),
            ("mut", "Mutable variable modifier", "mut "),
            (
                "match",
                "Pattern match expression",
                "match ${1:expr} {\n\t${2:pattern} => ${3:result},\n}",
            ),
            ("when", "Guard condition in match", "when ${1:condition}"),
            (
                "if",
                "Conditional branching",
                "if ${1:condition} {\n\t$0\n} else {\n}",
            ),
            ("else", "Else branch", "else {\n\t$0\n}"),
            ("async", "Asynchronous function modifier", "async "),
            ("await", "Await a Task or Promise", "await "),
            (
                "spawn",
                "Spawn a lightweight concurrent task",
                "spawn {\n\t$0\n}",
            ),
            (
                "routine",
                "Launch an Aura Routine (lightweight concurrent task)",
                "routine {\n\t$0\n}",
            ),
            (
                "go",
                "Launch a goroutine (lightweight concurrent task)",
                "go {\n\t$0\n}",
            ),
            (
                "select",
                "Select multiplexing on channels",
                "select {\n\t${1:chan} <- msg => {\n\t\t$0\n\t}\n}",
            ),
            (
                "import",
                "Import declarations from module",
                "import { ${1:item} } from \"${2:source}\";",
            ),
            ("export", "Export declaration", "export "),
            (
                "type",
                "Declare type alias or sum type",
                "type ${1:Name} = $0;",
            ),
            ("while", "While loop", "while ${1:condition} {\n\t$0\n}"),
            (
                "for",
                "For-in loop",
                "for ${1:item} in ${2:iterable} {\n\t$0\n}",
            ),
            (
                "defer",
                "Defer execution until function exit (LIFO)",
                "defer $0;",
            ),
            (
                "embed",
                "Embed static text file at compile time",
                "embed(\"${1:path}\")",
            ),
            (
                "embedBytes",
                "Embed static binary asset as Uint8Array at compile time",
                "embedBytes(\"${1:path}\")",
            ),
            ("return", "Return statement", "return $0;"),
        ];

        for (kw, doc, snippet) in keywords {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some(format!("keyword: {}", kw)),
                documentation: Some(doc.to_string()),
                insert_text: Some(snippet.to_string()),
            });
        }

        // 3. Built-in types
        let builtin_types = [
            "Int",
            "Float",
            "String",
            "Bool",
            "Option",
            "Result",
            "Task",
            "Channel",
            "List",
            "Array",
            "Iterator",
            "Interator",
            "Map",
            "Set",
        ];
        for ty in builtin_types {
            items.push(CompletionItem {
                label: ty.to_string(),
                kind: CompletionItemKind::Class,
                detail: Some(format!("type: {}", ty)),
                documentation: Some(format!("Built-in Aura type {}", ty)),
                insert_text: None,
            });
        }

        // 4. Built-in functions
        let builtin_funcs = [
            ("println", "println(value: Any): Unit", "println($0)"),
            ("print", "print(value: Any): Unit", "print($0)"),
            ("eprintln", "eprintln(value: Any): Unit", "eprintln($0)"),
            ("len", "len(container: Any): Int", "len($0)"),
            (
                "map",
                "map(coll, fn): Transform collection or iterator",
                "map(${1:coll}, ${2:fn})",
            ),
            (
                "filter",
                "filter(coll, pred): Filter collection or iterator",
                "filter(${1:coll}, ${2:pred})",
            ),
            (
                "sort",
                "sort(coll, cmp?): Sort collection or iterator",
                "sort(${1:coll})",
            ),
            ("iter", "iter(coll): Convert to Iterator", "iter(${1:coll})"),
        ];
        for (f, doc, snippet) in builtin_funcs {
            items.push(CompletionItem {
                label: f.to_string(),
                kind: CompletionItemKind::Function,
                detail: Some(format!("built-in function: {}", f)),
                documentation: Some(doc.to_string()),
                insert_text: Some(snippet.to_string()),
            });
        }

        // Deduplicate by label
        let mut seen = std::collections::HashSet::new();
        items.retain(|it| seen.insert(it.label.clone()));

        items
    }

    /// Formats the document.
    pub fn format(&self, _config: Option<FormatConfig>) -> Result<Vec<TextEdit>, String> {
        let formatted = format_aura(&self.source).map_err(|e| format!("{}", e))?;
        if formatted == self.source {
            return Ok(Vec::new());
        }

        let total_lines = self.lines.len() as u32;
        let last_line_len = self.lines.last().map(|l| l.len() as u32).unwrap_or(0);

        Ok(vec![TextEdit {
            range: LspRange::new(0, 0, total_lines, last_line_len),
            new_text: formatted,
        }])
    }

    fn word_at_position(&self, pos: &LspPosition) -> Option<(String, LspRange)> {
        let line_idx = pos.line as usize;
        if line_idx >= self.lines.len() {
            return None;
        }
        let line = &self.lines[line_idx];
        let col = pos.character as usize;
        if col > line.len() {
            return None;
        }

        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() {
            return None;
        }

        let target_col = if col >= chars.len() {
            chars.len().saturating_sub(1)
        } else {
            col
        };
        if !is_ident_char(chars[target_col]) {
            return None;
        }

        let mut start = target_col;
        while start > 0 && is_ident_char(chars[start - 1]) {
            start -= 1;
        }

        let mut end = target_col;
        while end + 1 < chars.len() && is_ident_char(chars[end + 1]) {
            end += 1;
        }

        let word: String = chars[start..=end].iter().collect();
        let range = LspRange::new(pos.line, start as u32, pos.line, (end + 1) as u32);
        Some((word, range))
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn find_ident_range(spans: &[(&str, LspRange)], name: &str, min_line: u32) -> LspRange {
    for (ident, range) in spans {
        if *ident == name && range.start.line >= min_line {
            return *range;
        }
    }
    LspRange::new(min_line, 0, min_line, name.len() as u32)
}

fn extract_doc_comments(source: &str) -> HashMap<String, String> {
    let mut comments = HashMap::new();
    let lines: Vec<&str> = source.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i].trim();
        if line.starts_with("fn ")
            || line.starts_with("pub fn ")
            || line.starts_with("export fn ")
            || line.starts_with("type ")
            || line.starts_with("export type ")
        {
            // Gather doc lines above
            let mut doc_lines = Vec::new();
            let mut j = i;
            while j > 0 {
                j -= 1;
                let prev = lines[j].trim();
                if prev.starts_with("///") {
                    doc_lines.push(prev.trim_start_matches("///").trim().to_string());
                } else if prev.starts_with("//") {
                    doc_lines.push(prev.trim_start_matches("//").trim().to_string());
                } else {
                    break;
                }
            }

            if !doc_lines.is_empty() {
                doc_lines.reverse();
                // Extract symbol name
                if let Some(sym_name) = extract_name_from_decl(line) {
                    comments.insert(sym_name, doc_lines.join("\n"));
                }
            }
        }
    }

    comments
}

fn extract_name_from_decl(decl: &str) -> Option<String> {
    let tokens: Vec<&str> = decl.split_whitespace().collect();
    for i in 0..tokens.len() {
        if (tokens[i] == "fn" || tokens[i] == "type") && i + 1 < tokens.len() {
            let raw_name = tokens[i + 1];
            let clean_name = raw_name
                .split(|c| c == '(' || c == '<' || c == '=' || c == '{' || c == ':')
                .next()
                .unwrap_or("");
            if !clean_name.is_empty() {
                return Some(clean_name.to_string());
            }
        }
    }
    None
}

fn parse_error_range(err: &str, lines: &[String]) -> LspRange {
    // Try to match line N, col M patterns
    if let Some(pos) = extract_line_col_from_error(err) {
        let line = if pos.0 > 0 { pos.0 - 1 } else { 0 };
        let col = if pos.1 > 0 { pos.1 - 1 } else { 0 };
        let line_len = lines
            .get(line as usize)
            .map(|l| l.len() as u32)
            .unwrap_or(col + 5);
        return LspRange::new(line, col, line, line_len.max(col + 1));
    }
    LspRange::new(0, 0, 0, 1)
}

fn extract_line_col_from_error(err: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = err
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() >= 2 {
        if let (Ok(l), Ok(c)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            if l < 10000 && c < 500 {
                return Some((l, c));
            }
        }
    }
    None
}

fn get_keyword_hover(kw: &str) -> Option<String> {
    let doc = match kw {
        "fn" => {
            "### `fn` *(Keyword)*\n\n**`fn` keyword** — Declares a function in Aura.\n\n```aura\nfn <name>(<params>): <ReturnType> => <Body>\n```\n\nDeclares a function. Functions are first-class, support closures, and tail-recursive calls are automatically compiled with **Tail-Call Optimization (TCO)** into iterative loops.\n\n---\n#### 📥 Parámetros / Entrada\n* `params`: Comma-separated typed parameters (`param: Type`).\n* `ReturnType`: Output type annotation (`Int`, `String`, `Unit`, etc.).\n\n---\n#### 📤 Retorno\n* Evaluates to the result of the body expression or `Unit` (`()`).\n\n---\n#### 💡 Ejemplo\n```aura\nfn add(a: Int, b: Int): Int => {\n    a + b\n}\n```"
        }
        "let" => {
            "### `let` *(Keyword)*\n\n```aura\nlet <name>: <Type> = <value>;\n```\n\nDeclares an **immutable** variable binding (default in Aura). Cannot be reassigned."
        }
        "mut" => {
            "### `mut` *(Keyword)*\n\n```aura\nlet mut <name>: <Type> = <initial_value>;\n```\n\nDeclares a **mutable** variable binding. Allows reassignment via `=`."
        }
        "type" => {
            "### `type` *(Keyword)*\n\n```aura\ntype <Name> = | <Variant1>(<Type>) | <Variant2>;\n```\n\nDeclares an **Algebraic Data Type (ADT / Sum Type)** or record/struct type. Variants are checked exhaustively with `match`."
        }
        "interface" => {
            "### `interface` *(Keyword)*\n\n```aura\ninterface <Name> {\n    fn <method>(<params>): <ReturnType>;\n}\n```\n\nDeclares a **structural interface** (Go-style duck typing). Types implicitly satisfy it by implementing all defined methods without explicit `implements`."
        }
        "match" => {
            "### `match` *(Keyword)*\n\n```aura\nmatch <expr> {\n    <Pattern> => <Branch>,\n    <Pattern> when <guard> => <Branch>,\n    _ => <Default>\n}\n```\n\nPattern matching expression with **exhaustive** compile-time validation of all sum type variants. Evaluates to the value of the executed branch."
        }
        "when" => {
            "### `when` *(Keyword)*\n\n```aura\n<Pattern> when <boolean_condition> => <Branch>\n```\n\nPattern guard filter. The arm only executes if pattern matches AND the `when` condition is `true`."
        }
        "defer" => {
            "### `defer` *(Keyword)*\n\n```aura\ndefer <statement_or_block>;\n```\n\nDefers statement execution until the enclosing function returns. Executes in deterministic **LIFO (Last-In, First-Out)** order. Ideal for closing database connections and releasing locks."
        }
        "errdefer" => {
            "### `errdefer` *(Keyword)*\n\n```aura\nerrdefer <statement_or_block>;\n```\n\nDefers statement execution until the enclosing function returns, **only if an error or panic occurs** (Zig model). Executes in deterministic LIFO order. Ideal for transaction rollbacks (`tx.rollback()`) and temporary resource cleanup on failure."
        }
        "packed" => {
            "### `packed` *(Keyword)*\n\n```aura\npacked struct <Name> {\n    <field>: <Type>,\n}\n```\n\nDeclares a **packed structure** with zero padding and exact binary memory alignment (Zig model). Ideal for binary network protocols, file formats, and zero-copy deserialization."
        }
        "spawn" | "routine" | "go" => {
            "### `go` / `routine` / `spawn` *(Keyword)*\n\n```aura\ngo {\n    <concurrent_work>\n};\n\n// O llamada directa:\ngo worker(ch);\n```\n\nLaunches a **Goroutine / Aura Routine** (lightweight fiber). Communicates concurrently via typed CSP channels (`Channel<T>`)."
        }
        "select" => {
            "### `select` *(Keyword)*\n\n```aura\nselect {\n    case msg <- ch => { ... },\n    default => { ... }\n}\n```\n\nMultiplexes on multiple asynchronous channel operations. If `default` is provided, executes non-blocking."
        }
        "embed" => {
            "### `embed` *(Macro / Keyword)*\n\n```aura\nembed(\"<relative_path>\"): String\n```\n\nEmbeds a static text asset directly into the compiled standalone binary at build time as a UTF-8 `String`."
        }
        "embedBytes" => {
            "### `embedBytes` *(Macro / Keyword)*\n\n```aura\nembedBytes(\"<relative_path>\"): List<Byte>\n```\n\nEmbeds a static binary file directly into the compiled output as a byte array (`Uint8Array`)."
        }
        "async" => {
            "### `async` *(Keyword)*\n\n```aura\nasync fn <name>(<params>): Task<<T>, <E>> => ...\n```\n\nMarks a function as asynchronous, returning a `Task<T, E>`."
        }
        "await" => {
            "### `await` *(Keyword)*\n\n```aura\nawait <task_expression>\n```\n\nSuspends execution until the awaited `Task<T, E>` resolves, unwrapping its inner value."
        }
        "panic" => {
            "### `panic` *(Built-in Function)*\n\n```aura\npanic(message: String): Never\n```\n\nStops normal execution and initiates stack unwinding. Can be intercepted with `recover()` inside a `defer` block."
        }
        "recover" => {
            "### `recover` *(Built-in Function)*\n\n```aura\nrecover(): Option<Any>\n```\n\nCatches an active panic during unwinding inside a `defer` block. Returns `Some(error)` or `None`."
        }
        "if" => {
            "### `if` *(Keyword)*\n\n```aura\nif <cond> => <ThenExpr> else => <ElseExpr>\n```\n\nConditional branching expression. Both branches must unify to the same type."
        }
        "else" => "### `else` *(Keyword)*\n\nAlternative branch of an `if` expression.",
        "while" => {
            "### `while` *(Keyword)*\n\n```aura\nwhile <condition> { <body> }\n```\n\nIterative loop. Runs while `<condition>` is `true`."
        }
        "for" => {
            "### `for` *(Keyword)*\n\n```aura\nfor <item> in <iterable_or_channel> { <body> }\n```\n\nIterates over a list, range, or CSP channel."
        }
        "in" => "### `in` *(Keyword)*\n\nSpecifies the collection or channel in a `for` loop.",
        "break" => "### `break` *(Keyword)*\n\nTerminates innermost or labeled enclosing loop.",
        "continue" => {
            "### `continue` *(Keyword)*\n\nSkips to next iteration of innermost or labeled enclosing loop."
        }
        "return" => "### `return` *(Keyword)*\n\nExits early from the enclosing function.",
        "import" => {
            "### `import` *(Keyword)*\n\nImports symbols from another `.aura` file or npm module."
        }
        "export" => {
            "### `export` *(Keyword)*\n\nExports symbols to consumers and generates `.d.ts` declarations."
        }
        _ => return None,
    };
    Some(doc.to_string())
}

fn get_builtin_hover(name: &str) -> Option<String> {
    let doc = match name {
        "Int" => {
            "### `Int` *(Core Primitive Type)*\n\n64-bit signed integer (`i64`). Supports arithmetic and bitwise operations."
        }
        "Float" => {
            "### `Float` *(Core Primitive Type)*\n\n64-bit IEEE 754 floating-point number (`f64`)."
        }
        "String" => {
            "### `String` *(Core Primitive Type)*\n\nImmutable UTF-8 string with template interpolation support (`${expr}`)."
        }
        "Bool" => "### `Bool` *(Core Primitive Type)*\n\nBoolean truth value (`true` or `false`).",
        "Unit" => {
            "### `Unit` *(Core Primitive Type)*\n\nUnit type `()`. Denotes the absence of a meaningful value (equivalent to `void`)."
        }
        "Byte" => {
            "### `Byte` *(Core Primitive Type)*\n\n8-bit unsigned integer (`0..255`). Used for raw byte buffers."
        }
        "Option" => {
            "### `Option<T>` *(Standard Sum Type)*\n\n```aura\ntype Option<T> = | Some(T) | None\n```\n\nEliminates null/undefined pointers. Unwrapped via `match` or `?` operator."
        }
        "Result" => {
            "### `Result<T, E>` *(Standard Sum Type)*\n\n```aura\ntype Result<T, E> = | Ok(T) | Err(E)\n```\n\nRepresents success (`Ok`) or failure (`Err`). Propagated with `?` operator."
        }
        "Task" => {
            "### `Task<T, E>` *(Async Computation)*\n\nRepresents an asynchronous computation that can be awaited with `await`."
        }
        "Channel" => {
            "### `Channel<T>` *(CSP Concurrency)*\n\n```aura\nChannel<T>::new(buffer_size: Int = 0): Channel<T>\n```\n\nTyped concurrent channel. Send with `ch <- val`, receive with `<-ch`."
        }
        "SendChannel" => {
            "### `SendChannel<T>` *(Write-Only Channel)*\n\nWrite-only channel view. Only sends (`ch <- val`) permitted."
        }
        "RecvChannel" => {
            "### `RecvChannel<T>` *(Read-Only Channel)*\n\nRead-only channel view. Only receives (`<-ch`) permitted."
        }
        "Context" => {
            "### `Context` *(Structured Concurrency)*\n\nPropagates cancellation signals and timeouts across concurrent fibers."
        }
        "WaitGroup" => {
            "### `WaitGroup` *(Sync Primitive)*\n\nCoordinates and waits for a group of concurrent fibers to complete."
        }
        "Mutex" => {
            "### `Mutex` *(Sync Primitive)*\n\nMutual exclusion lock with `lock()` and `unlock()`."
        }
        "List" => {
            "### `List<T>` *(Collection)*\n\nImmutable indexed sequence of elements of type `T`. Supports `.map()`, `.filter()`, `.sort()`, and pipeline transformations."
        }
        "Array" => {
            "### `Array<T>` *(Collection)*\n\nOrdered collection of elements of type `T` (alias for `List<T>`). Supports `.map()`, `.filter()`, `.sort()`, and pipeline transformations."
        }
        "Iterator" | "Interator" => {
            "### `Iterator<T>` *(Interface)*\n\n```aura\ninterface Iterator<T> {\n  next(): Option<T>;\n}\n```\n\nInterface for sequential item iteration. Provides `.map()`, `.filter()`, `.sort()`, `.toList()`, and `for..in` loop consumption."
        }
        "Map" => "### `Map<K, V>` *(Collection)*\n\nAssociative key-value collection.",
        "Set" => "### `Set<T>` *(Collection)*\n\nCollection of unique values.",
        "map" => {
            "### `map` *(Collection & Iterator Transform)*\n\n```aura\nmap(coll: Array<T>, fn: (item: T) => U): Array<U>\nmap(coll: Iterator<T>, fn: (item: T) => U): Iterator<U>\n```\n\nTransforms elements of a collection or iterator by applying a mapping function. Usable as a method (`coll.map(fn)`) or via pipeline (`coll |> map(fn)`)."
        }
        "filter" => {
            "### `filter` *(Collection & Iterator Filter)*\n\n```aura\nfilter(coll: Array<T>, pred: (item: T) => Bool): Array<T>\nfilter(coll: Iterator<T>, pred: (item: T) => Bool): Iterator<T>\n```\n\nSelects elements matching a predicate function. Usable as a method (`coll.filter(pred)`) or via pipeline (`coll |> filter(pred)`)."
        }
        "sort" => {
            "### `sort` *(Collection & Iterator Sort)*\n\n```aura\nsort(coll: Array<T>, cmp?: (a: T, b: T) => Int): Array<T>\nsort(coll: Iterator<T>, cmp?: (a: T, b: T) => Int): Iterator<T>\n```\n\nReturns elements sorted numerically or by custom comparator. Pure and non-mutating. Usable as a method (`coll.sort()`) or via pipeline (`coll |> sort()`)."
        }
        "iter" => {
            "### `iter` *(Built-in Function)*\n\n```aura\niter(coll: Array<T> | Iterator<T>): Iterator<T>\n```\n\nWraps an array, list, or iterable in an `Iterator<T>`."
        }
        "println" => {
            "### `println` *(Built-in Function)*\n\n```aura\nprintln(value: Any): Unit\n```\n\nPrints value followed by a newline to standard output."
        }
        "print" => {
            "### `print` *(Built-in Function)*\n\n```aura\nprint(value: Any): Unit\n```\n\nPrints value without a newline to standard output."
        }
        "eprintln" => {
            "### `eprintln` *(Built-in Function)*\n\n```aura\neprintln(value: Any): Unit\n```\n\nPrints value followed by a newline to standard error."
        }
        "len" => {
            "### `len` *(Built-in Function)*\n\n```aura\nlen(container: Any): Int\n```\n\nReturns length or element count of List, String, Map, or Channel."
        }
        "pg" | "postgres" => {
            "### `pg` *(Native Database Driver)*\n\n```aura\npg::connect(url: String): Result<PgClient, Error>\n```\n\nPostgreSQL database driver with pooling and transactions."
        }
        "redis" => {
            "### `redis` *(Native Database Driver)*\n\n```aura\nredis::connect(url: String): Result<RedisClient, Error>\n```\n\nRedis client for caching, KV, and Pub/Sub."
        }
        "mysql" => {
            "### `mysql` *(Native Database Driver)*\n\n```aura\nmysql::connect(url: String): Result<MySqlClient, Error>\n```\n\nMySQL database client with connection pooling."
        }
        "mongo" | "mongodb" => {
            "### `mongo` *(Native Database Driver)*\n\n```aura\nmongo::connect(url: String): Result<MongoClient, Error>\n```\n\nMongoDB document database driver."
        }
        "http" => {
            "### `http` *(Native Server & Client)*\n\n```aura\nhttp::Server::new(): Server\n```\n\nNative HTTP web server and client library."
        }
        _ => return None,
    };
    Some(doc.to_string())
}

/// The Aura LSP Server managing multiple documents and processing JSON-RPC messages.
pub struct LspServer {
    pub documents: HashMap<String, DocumentState>,
    pub is_initialized: bool,
    pub is_shutdown: bool,
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            is_initialized: false,
            is_shutdown: false,
        }
    }

    /// Handles an incoming JSON-RPC raw request string and produces an optional JSON-RPC response.
    pub fn handle_message(&mut self, message: &str) -> Option<String> {
        let (id, method, params) = parse_json_rpc(message)?;

        match method.as_str() {
            "initialize" => {
                self.is_initialized = true;
                let result = format!(
                    r#"{{"capabilities":{{"textDocumentSync":1,"hoverProvider":true,"definitionProvider":true,"completionProvider":{{"resolveProvider":false,"triggerCharacters":[".",":"]}},"documentSymbolProvider":true,"documentFormattingProvider":true}}}}"#
                );
                Some(format_json_response(id, &result))
            }
            "initialized" => None,
            "shutdown" => {
                self.is_shutdown = true;
                Some(format_json_response(id, "null"))
            }
            "exit" => None,
            "textDocument/didOpen" => {
                if let (Some(uri), Some(text)) = (
                    extract_json_string(&params, "uri"),
                    extract_json_string(&params, "text"),
                ) {
                    let version = extract_json_int(&params, "version").unwrap_or(1);
                    let state = DocumentState::new(uri.clone(), text, version);
                    let diag_notification = format_publish_diagnostics(&uri, &state.diagnostics);
                    self.documents.insert(uri, state);
                    return Some(diag_notification);
                }
                None
            }
            "textDocument/didChange" => {
                if let (Some(uri), Some(text)) = (
                    extract_json_string(&params, "uri"),
                    extract_json_string(&params, "text"),
                ) {
                    let version = extract_json_int(&params, "version").unwrap_or(1);
                    if let Some(doc) = self.documents.get_mut(&uri) {
                        doc.update(text, version);
                        return Some(format_publish_diagnostics(&uri, &doc.diagnostics));
                    } else {
                        let state = DocumentState::new(uri.clone(), text, version);
                        let diag = format_publish_diagnostics(&uri, &state.diagnostics);
                        self.documents.insert(uri, state);
                        return Some(diag);
                    }
                }
                None
            }
            "textDocument/didClose" => {
                if let Some(uri) = extract_json_string(&params, "uri") {
                    self.documents.remove(&uri);
                }
                None
            }
            "textDocument/hover" => {
                let uri = extract_json_string(&params, "uri")?;
                let line = extract_json_int(&params, "line")? as u32;
                let char_idx = extract_json_int(&params, "character")? as u32;
                let pos = LspPosition {
                    line,
                    character: char_idx,
                };

                if let Some(doc) = self.documents.get(&uri) {
                    if let Some(hover) = doc.hover(&pos) {
                        let escaped_contents = escape_json_string(&hover.contents);
                        let range_json = if let Some(r) = hover.range {
                            format!(
                                r#","range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}"#,
                                r.start.line, r.start.character, r.end.line, r.end.character
                            )
                        } else {
                            String::new()
                        };
                        let res = format!(
                            r#"{{"contents":{{"kind":"markdown","value":"{}"}}{}}}"#,
                            escaped_contents, range_json
                        );
                        return Some(format_json_response(id, &res));
                    }
                }
                Some(format_json_response(id, "null"))
            }
            "textDocument/definition" => {
                let uri = extract_json_string(&params, "uri")?;
                let line = extract_json_int(&params, "line")? as u32;
                let char_idx = extract_json_int(&params, "character")? as u32;
                let pos = LspPosition {
                    line,
                    character: char_idx,
                };

                if let Some(doc) = self.documents.get(&uri) {
                    if let Some(loc) = doc.definition(&pos) {
                        let res = format!(
                            r#"{{"uri":"{}","range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
                            escape_json_string(&loc.uri),
                            loc.range.start.line,
                            loc.range.start.character,
                            loc.range.end.line,
                            loc.range.end.character
                        );
                        return Some(format_json_response(id, &res));
                    }
                }
                Some(format_json_response(id, "null"))
            }
            "textDocument/completion" => {
                let uri = extract_json_string(&params, "uri")?;
                let line = extract_json_int(&params, "line").unwrap_or(0) as u32;
                let char_idx = extract_json_int(&params, "character").unwrap_or(0) as u32;
                let pos = LspPosition {
                    line,
                    character: char_idx,
                };

                if let Some(doc) = self.documents.get(&uri) {
                    let items = doc.completions(&pos);
                    let mut items_json = Vec::new();
                    for item in items {
                        let detail_str = item
                            .detail
                            .map(|d| format!(r#","detail":"{}""#, escape_json_string(&d)))
                            .unwrap_or_default();
                        let doc_str = item
                            .documentation
                            .map(|d| format!(r#","documentation":"{}""#, escape_json_string(&d)))
                            .unwrap_or_default();
                        let insert_str = item
                            .insert_text
                            .map(|i| format!(r#","insertText":"{}""#, escape_json_string(&i)))
                            .unwrap_or_default();
                        items_json.push(format!(
                            r#"{{"label":"{}","kind":{}{}{}{}}}"#,
                            escape_json_string(&item.label),
                            item.kind as u32,
                            detail_str,
                            doc_str,
                            insert_str
                        ));
                    }
                    let res = format!("[{}]", items_json.join(","));
                    return Some(format_json_response(id, &res));
                }
                Some(format_json_response(id, "[]"))
            }
            "textDocument/formatting" => {
                let uri = extract_json_string(&params, "uri")?;
                if let Some(doc) = self.documents.get(&uri) {
                    if let Ok(edits) = doc.format(None) {
                        let mut edits_json = Vec::new();
                        for edit in edits {
                            edits_json.push(format!(
                                r#"{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"newText":"{}"}}"#,
                                edit.range.start.line,
                                edit.range.start.character,
                                edit.range.end.line,
                                edit.range.end.character,
                                escape_json_string(&edit.new_text)
                            ));
                        }
                        return Some(format_json_response(
                            id,
                            &format!("[{}]", edits_json.join(",")),
                        ));
                    }
                }
                Some(format_json_response(id, "[]"))
            }
            "textDocument/documentSymbol" => {
                let uri = extract_json_string(&params, "uri")?;
                if let Some(doc) = self.documents.get(&uri) {
                    let symbols_json: Vec<String> =
                        doc.symbols.iter().map(format_document_symbol).collect();
                    return Some(format_json_response(
                        id,
                        &format!("[{}]", symbols_json.join(",")),
                    ));
                }
                Some(format_json_response(id, "[]"))
            }
            _ => {
                if id.is_some() {
                    Some(format_json_response(id, "null"))
                } else {
                    None
                }
            }
        }
    }

    /// Runs the stdio LSP server loop.
    pub fn run_stdio(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut reader = stdin.lock();

        loop {
            let mut content_length: Option<usize> = None;
            let mut header_line = String::new();

            loop {
                header_line.clear();
                let bytes_read = reader.read_line(&mut header_line)?;
                if bytes_read == 0 {
                    return Ok(()); // EOF
                }

                let trimmed = header_line.trim();
                if trimmed.is_empty() {
                    break; // Header section finished
                }

                if trimmed.to_lowercase().starts_with("content-length:") {
                    if let Some(len_str) = trimmed.split(':').nth(1) {
                        content_length = len_str.trim().parse::<usize>().ok();
                    }
                }
            }

            if let Some(len) = content_length {
                let mut body_buf = vec![0u8; len];
                io::Read::read_exact(&mut reader, &mut body_buf)?;
                let body_str = String::from_utf8_lossy(&body_buf);

                if let Some(response) = self.handle_message(&body_str) {
                    let response_bytes = response.as_bytes();
                    write!(stdout, "Content-Length: {}\r\n\r\n", response_bytes.len())?;
                    stdout.write_all(response_bytes)?;
                    stdout.flush()?;
                }

                if self.is_shutdown {
                    break;
                }
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// JSON-RPC Parser and Formatter Helpers
// -----------------------------------------------------------------------------

fn parse_json_rpc(input: &str) -> Option<(Option<String>, String, String)> {
    let method = extract_json_string(input, "method")?;
    let id = extract_json_raw_field(input, "id");
    let params = extract_json_object_or_self(input, "params");
    Some((id, method, params))
}

fn format_json_response(id: Option<String>, result: &str) -> String {
    let id_str = id.unwrap_or_else(|| "null".to_string());
    format!(r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#, id_str, result)
}

fn format_publish_diagnostics(uri: &str, diags: &[Diagnostic]) -> String {
    let mut diag_items = Vec::new();
    for d in diags {
        diag_items.push(format!(
            r#"{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"severity":{},"source":"{}","message":"{}"}}"#,
            d.range.start.line,
            d.range.start.character,
            d.range.end.line,
            d.range.end.character,
            d.severity as u32,
            escape_json_string(&d.source),
            escape_json_string(&d.message)
        ));
    }
    format!(
        r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{{"uri":"{}","diagnostics":[{}]}}}}"#,
        escape_json_string(uri),
        diag_items.join(",")
    )
}

fn format_document_symbol(sym: &DocumentSymbol) -> String {
    let detail_str = sym
        .detail
        .as_ref()
        .map(|d| format!(r#","detail":"{}""#, escape_json_string(d)))
        .unwrap_or_default();
    let children_json: Vec<String> = sym.children.iter().map(format_document_symbol).collect();
    let children_str = if !children_json.is_empty() {
        format!(r#","children":[{}]"#, children_json.join(","))
    } else {
        String::new()
    };

    format!(
        r#"{{"name":"{}","kind":{}{},"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"selectionRange":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}{}}}"#,
        escape_json_string(&sym.name),
        sym.kind as u32,
        detail_str,
        sym.range.start.line,
        sym.range.start.character,
        sym.range.end.line,
        sym.range.end.character,
        sym.selection_range.start.line,
        sym.selection_range.start.character,
        sym.selection_range.end.line,
        sym.selection_range.end.character,
        children_str
    )
}

fn extract_json_string(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\"", field);
    let field_pos = json.find(&pattern)?;
    let after_field = &json[field_pos + pattern.len()..];
    let colon_pos = after_field.find(':')?;
    let after_colon = &after_field[colon_pos + 1..].trim_start();

    if after_colon.starts_with('"') {
        let mut s = String::new();
        let mut escaped = false;
        for c in after_colon[1..].chars() {
            if escaped {
                match c {
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    _ => s.push(c),
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                return Some(s);
            } else {
                s.push(c);
            }
        }
    }
    None
}

fn extract_json_int(json: &str, field: &str) -> Option<i64> {
    let pattern = format!("\"{}\"", field);
    let field_pos = json.find(&pattern)?;
    let after_field = &json[field_pos + pattern.len()..];
    let colon_pos = after_field.find(':')?;
    let after_colon = &after_field[colon_pos + 1..].trim_start();

    let num_str: String = after_colon
        .chars()
        .take_while(|c| c.is_digit(10) || *c == '-')
        .collect();
    num_str.parse::<i64>().ok()
}

fn extract_json_raw_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\"", field);
    let field_pos = json.find(&pattern)?;
    let after_field = &json[field_pos + pattern.len()..];
    let colon_pos = after_field.find(':')?;
    let after_colon = &after_field[colon_pos + 1..].trim_start();

    if after_colon.starts_with('"') {
        return extract_json_string(json, field).map(|s| format!("\"{}\"", escape_json_string(&s)));
    }

    let raw: String = after_colon
        .chars()
        .take_while(|c| c.is_digit(10) || *c == '-' || *c == 'n' || *c == 'u' || *c == 'l')
        .collect();
    if !raw.is_empty() { Some(raw) } else { None }
}

fn extract_json_object_or_self(json: &str, field: &str) -> String {
    let pattern = format!("\"{}\"", field);
    if let Some(field_pos) = json.find(&pattern) {
        let after_field = &json[field_pos + pattern.len()..];
        if let Some(colon_pos) = after_field.find(':') {
            return after_field[colon_pos + 1..].trim_start().to_string();
        }
    }
    json.to_string()
}

fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_hover_and_definition() {
        let src = r#"
/// Adds two numbers together
export fn add(x: Int, y: Int): Int => x + y;

let result = add(10, 20);
"#;
        let state = DocumentState::new("file:///main.aura".to_string(), src.to_string(), 1);
        assert_eq!(state.diagnostics.len(), 0);

        // Test hover on 'add' in definition
        let hover_def = state.hover(&LspPosition {
            line: 2,
            character: 12,
        });
        assert!(hover_def.is_some());
        let val = hover_def.unwrap().contents;
        assert!(val.contains("fn add(x: Int, y: Int) -> Int"));
        assert!(val.contains("Adds two numbers together"));

        // Test Go to Definition on 'add' call at line 4
        let def_loc = state.definition(&LspPosition {
            line: 4,
            character: 14,
        });
        assert!(def_loc.is_some());
        let loc = def_loc.unwrap();
        assert_eq!(loc.uri, "file:///main.aura");
        assert_eq!(loc.range.start.line, 2);
    }

    #[test]
    fn test_lsp_json_rpc_lifecycle() {
        let mut server = LspServer::new();

        // Initialize
        let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let init_resp = server
            .handle_message(init_req)
            .expect("Expected init response");
        assert!(init_resp.contains("capabilities"));
        assert!(init_resp.contains("hoverProvider"));

        // didOpen
        let did_open = r#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"uri":"file:///test.aura","text":"fn hello(): String => \"world\";","version":1}}"#;
        let did_open_resp = server
            .handle_message(did_open)
            .expect("Expected diagnostics notification");
        assert!(did_open_resp.contains("publishDiagnostics"));

        // hover
        let hover_req = r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/hover","params":{"uri":"file:///test.aura","line":0,"character":4}}"#;
        let hover_resp = server
            .handle_message(hover_req)
            .expect("Expected hover response");
        assert!(hover_resp.contains("hello"));

        // completions
        let comp_req = r#"{"jsonrpc":"2.0","id":3,"method":"textDocument/completion","params":{"uri":"file:///test.aura","line":0,"character":5}}"#;
        let comp_resp = server
            .handle_message(comp_req)
            .expect("Expected comp response");
        assert!(comp_resp.contains("hello"));
        assert!(comp_resp.contains("async"));
    }
}
