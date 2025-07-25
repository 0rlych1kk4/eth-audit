use solang_parser::pt::SourceUnit;
use solang_parser::pt::Loc;

/// A single security finding, including its rule, message, severity, and AST location.
#[derive(Debug, serde::Serialize)]
pub struct Finding {
    pub rule: &'static str,
    pub message: String,
    pub severity: Severity,
    pub loc: Loc,
}

/// Severity levels for findings.
#[derive(Debug, serde::Serialize)]
pub enum Severity {
    Low,
    Medium,
    High,
}

/// The parsed contract, combining its AST and raw source text.
#[derive(Debug)]
pub struct ParsedContract {
    /// The Solang AST of the entire source file.
    pub ast: SourceUnit,

    /// Source code string (used for extracting snippets or byte offsets).
    pub source: String,
}
