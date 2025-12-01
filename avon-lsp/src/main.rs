//! Avon Language Server Protocol (LSP) Implementation
//!
//! This LSP reuses the compiler's lexer, parser, and eval functions to provide
//! perfect accuracy. No duplicate parsing logic - just use the same validation
//! that the compiler uses.
//!
//! ## How it Works
//! 1. User types Avon code in editor
//! 2. LSP receives text via `did_change` callback
//! 3. Pass text to lexer::tokenize() - catches syntax errors
//! 4. Pass tokens to parser::parse() - builds AST
//! 5. Pass AST to eval::eval() with empty symbols - catches type/runtime errors
//! 6. Convert any errors to LSP diagnostics
//! 7. Publish diagnostics to editor
//!
//! Result: Perfect diagnostics, zero false positives, because we're using
//! the exact same validation logic as the compiler.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use avon::common::Value;
use avon::eval::{collect_file_templates, eval, initial_builtins};
use avon::lexer::tokenize;
use avon::parser::parse_with_error;

#[derive(Clone)]
struct Backend {
    client: Client,
    document_map: Arc<RwLock<HashMap<String, String>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> RpcResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![]),
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;

        {
            let mut map = self.document_map.write().await;
            map.insert(uri.clone(), text.clone());
        }

        self.publish_diagnostics(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.content_changes[0].text.clone();

        {
            let mut map = self.document_map.write().await;
            map.insert(uri.clone(), text.clone());
        }

        self.publish_diagnostics(&uri, &text).await;
    }

    async fn completion(&self, params: CompletionParams) -> RpcResult<Option<CompletionResponse>> {
        // Get the document content
        let uri = params.text_document_position.text_document.uri.to_string();
        let map = self.document_map.read().await;
        let text = match map.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(map);

        // Get the line and character
        let line = params.text_document_position.position.line as usize;
        let character = params.text_document_position.position.character as usize;

        // Get the word at cursor position
        let lines: Vec<&str> = text.lines().collect();
        let current_line = lines.get(line).unwrap_or(&"");

        // Find the start of the word (identifier characters: alphanumeric, underscore)
        let word_start = current_line[..character]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &current_line[word_start..character];

        // If word is empty, don't provide completions
        if word.is_empty() {
            return Ok(None);
        }

        // Get all builtin function names
        let mut completions = Vec::new();
        let builtins = initial_builtins();

        for (name, value) in builtins.iter() {
            // Only include builtin functions and functions in completions
            match value {
                Value::Builtin(_, _) | Value::Function { .. } => {
                    if name.starts_with(word) {
                        completions.push(CompletionItem {
                            label: name.clone(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some("builtin function".to_string()),
                            insert_text: Some(name.clone()),
                            ..Default::default()
                        });
                    }
                }
                _ => {}
            }
        }

        // Sort completions alphabetically
        completions.sort_by(|a, b| a.label.cmp(&b.label));

        if completions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(completions)))
        }
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }
}

impl Backend {
    /// Publish diagnostics for a document using the compiler's validation
    async fn publish_diagnostics(&self, uri: &str, text: &str) {
        let mut diagnostics = Vec::new();
        fn full_line_end(text: &str, line: u32) -> u32 {
            // VS Code LSP uses UTF-16 code units for Position.character
            text.lines()
                .nth(line as usize)
                .map(|line_text| line_text.encode_utf16().count() as u32)
                .unwrap_or(0)
        }

        // Step 1: Tokenize
        match tokenize(text.to_string()) {
            Err(err) => {
                // Lexer error - report it
                let line = (err.line.saturating_sub(1)) as u32;
                let end_character = full_line_end(text, line);
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line, character: 0 },
                        end: Position {
                            line,
                            character: end_character,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("avon".into()),
                    message: err.message.clone(),
                    ..Default::default()
                });
            }
            Ok(tokens) => {
                // Step 2: Parse (report errors without crashing)
                match parse_with_error(tokens) {
                    Err(err) => {
                        let line = (err.line.saturating_sub(1)) as u32;
                        let end_character = full_line_end(text, line);
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line, character: 0 },
                                end: Position {
                                    line,
                                    character: end_character,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("avon".into()),
                            message: err.message.clone(),
                            ..Default::default()
                        });
                    }
                    Ok(ast) => {
                        // Step 3: Type-check and validate using eval
                        // Use initial_builtins to include all built-in functions (split, join, etc.)
                        let mut symbols = initial_builtins();
                        match eval(ast.program, &mut symbols, text) {
                            Err(err) => {
                                // Compilation error - report it
                                let line = (err.line.saturating_sub(1)) as u32;
                                let end_character = full_line_end(text, line);
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position { line, character: 0 },
                                        end: Position {
                                            line,
                                            character: end_character,
                                        },
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    source: Some("avon".into()),
                                    message: err.message.clone(),
                                    ..Default::default()
                                });
                            }
                            Ok(value) => {
                                // Step 4: If result is FileTemplate, List, or bare Template/Path, validate
                                // Bare Templates/Paths can't be deployed and indicate a likely user error
                                // Lists may contain FileTemplates that need rendering validation
                                // FileTemplates are always validated for rendering errors
                                match &value {
                                    Value::FileTemplate { .. } | Value::List(_) => {
                                        if let Err(err) = collect_file_templates(&value, text) {
                                            let line = (err.line.saturating_sub(1)) as u32;
                                            let end_character = full_line_end(text, line);
                                            diagnostics.push(Diagnostic {
                                                range: Range {
                                                    start: Position { line, character: 0 },
                                                    end: Position {
                                                        line,
                                                        character: end_character,
                                                    },
                                                },
                                                severity: Some(DiagnosticSeverity::ERROR),
                                                source: Some("avon".into()),
                                                message: err.message.clone(),
                                                ..Default::default()
                                            });
                                        }
                                    }
                                    Value::Template(_, _) | Value::Path(_, _) => {
                                        // Bare Templates and Paths can't be deployed - report error
                                        let err_message = "cannot deploy bare template or path - use @file {{...}} syntax to create a FileTemplate";
                                        diagnostics.push(Diagnostic {
                                            range: Range {
                                                start: Position {
                                                    line: 0,
                                                    character: 0,
                                                },
                                                end: Position {
                                                    line: 0,
                                                    character: full_line_end(text, 0),
                                                },
                                            },
                                            severity: Some(DiagnosticSeverity::ERROR),
                                            source: Some("avon".into()),
                                            message: err_message.to_string(),
                                            ..Default::default()
                                        });
                                    }
                                    _ => {
                                        // Other return values (numbers, strings, dicts, etc.) are fine
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Publish diagnostics to the client
        let uri: Url = uri
            .parse()
            .unwrap_or_else(|_| Url::parse("file:///unknown").unwrap());
        let _ = self
            .client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: Arc::new(RwLock::new(HashMap::new())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
