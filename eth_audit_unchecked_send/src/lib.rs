use eth_audit_core::{
    Detector,
    registry::register_detector,
    types::{ParsedContract, Finding, Severity, Location},
};
use solang_parser::pt::{SourceUnitPart, ContractPart, Statement, Expression};

/// Flags any low-level `.send(...)` whose return value is not checked.
pub struct UncheckedSendDetector;

impl Detector for UncheckedSendDetector {
    fn name(&self) -> &'static str { "unchecked-send" }

    fn run(&self, contract: &ParsedContract) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Walk each top-level part of the AST
        for part in &contract.ast.0 {
            if let SourceUnitPart::ContractDefinition(_, _, _, parts) = part {
                for item in parts {
                    if let ContractPart::FunctionDefinition(_, _, _, _, _, _, Some(body)) = item {
                        for stmt in &body.statements {
                            if let Statement::Expression(_, expr) = stmt {
                                if let Expression::FunctionCall(loc, func, _) = &**expr {
                                    if func.name.as_str().ends_with(".send") {
                                        // best-effort column: find in source line
                                        let line_src = contract
                                            .source
                                            .lines()
                                            .nth(loc.start_line - 1)
                                            .unwrap_or("");
                                        let column = line_src
                                            .find(".send")
                                            .map(|i| i + 1)
                                            .unwrap_or(1);

                                        findings.push(Finding {
                                            rule: self.name(),
                                            message: "`.send()` return value is unchecked".into(),
                                            severity: Severity::Medium,
                                            location: Location {
                                                line: loc.start_line,
                                                column,
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        findings
    }
}

#[no_mangle]
pub extern "C" fn init_plugin(reg: &mut eth_audit_core::registry::Registry) {
    register_detector(reg, UncheckedSendDetector);
}
