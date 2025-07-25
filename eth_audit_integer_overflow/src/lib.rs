use eth_audit_core::{
    Detector,
    registry::register_detector,
    types::{ParsedContract, Finding, Severity, Location},
};
use solang_parser::pt::{Expression, BinaryOp};

/// Flags any use of `+`, `-`, or `*` on unsigned integers (simple overflow check).
pub struct IntegerOverflowDetector;

impl Detector for IntegerOverflowDetector {
    fn name(&self) -> &'static str { "integer-overflow" }

    fn run(&self, contract: &ParsedContract) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Walk all binary-op expressions in the AST
        contract.ast.0.walk_expressions(|expr| {
            if let Expression::BinaryOp(loc, _, op, _) = expr {
                if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                    // record the location of the operator
                    findings.push(Finding {
                        rule: self.name(),
                        message: "Potential integer overflow/underflow".into(),
                        severity: Severity::Medium,
                        location: Location {
                            line: loc.start_line,
                            column: loc.start_column.unwrap_or(1),
                        },
                    });
                }
            }
        });

        findings
    }
}

#[no_mangle]
pub extern "C" fn init_plugin(reg: &mut eth_audit_core::registry::Registry) {
    register_detector(reg, IntegerOverflowDetector);
}

