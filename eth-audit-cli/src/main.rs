use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use anyhow::Result;
use eth_audit_core::registry::Registry;
use eth_audit_core::types::ParsedContract;

#[cfg(feature = "reentrancy")]
use eth_audit_reentrancy::init_plugin;

/// A Rust-native smart-contract auditor CLI.
#[derive(Parser)]
#[command(author, version, about = "Audit Solidity contracts for security issues", long_about = None)]
struct Cli {
    /// Solidity file path
    #[clap(value_parser)]
    input: PathBuf,

    /// Detector rules to run (comma-separated) or `all`
    #[clap(long, default_value = "all")]
    rules: String,

    /// Output format
    #[clap(long, value_enum, default_value = "json")]
    format: OutputFormat,

    /// Write results to file (defaults to stdout)
    #[clap(long)]
    output: Option<PathBuf>,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
    Sarif,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Read and parse the contract
    let source = std::fs::read_to_string(&args.input)?;
    let contract = ParsedContract { source };

    // Build the registry and load plugins
    let mut reg = Registry::new();
    #[cfg(feature = "reentrancy")]
    init_plugin(&mut reg);

    // Run detectors
    let findings = reg.run_all(&contract);

    // Serialize output
    let out = match args.format {
        OutputFormat::Text => findings.iter()
            .map(|f| format!(
                "[{:?}] {} at {}:{}",
                f.severity, f.message, f.location.line, f.location.column
            ))
            .collect::<Vec<_>>()
            .join("\n"),

        OutputFormat::Json => serde_json::to_string_pretty(&findings)?,

        OutputFormat::Sarif => {
            // TODO: hook up serde_sarif or custom SARIF serializer
            panic!("SARIF format not yet implemented")
        }
    };

    // Emit to file or stdout
    if let Some(path) = args.output {
        std::fs::write(path, out)?;
    } else {
        println!("{}", out);
    }

    Ok(())
}
