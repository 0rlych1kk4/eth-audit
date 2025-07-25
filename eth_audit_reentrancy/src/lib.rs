use eth_audit_core::{
    Detector,
    registry::register_detector,
    types::{ParsedContract, Finding, Severity, Location},
};

/// A basic reentrancy detector:
/// 1. Find the first line with ".call{value"
/// 2. After that, look for "balance[msg.sender] -="
/// 3. If both appear in that order, emit a Finding at the call-site.
pub struct ReentrancyDetector;

impl Detector for ReentrancyDetector {
    fn name(&self) -> &'static str { "reentrancy" }

    fn run(&self, contract: &ParsedContract) -> Vec<Finding> {
        let mut findings = Vec::new();
        let mut call_line: Option<usize> = None;

        for (idx, line) in contract.source.lines().enumerate() {
            // Step 1: locate external call
            if call_line.is_none() && line.contains(".call{value") {
                call_line = Some(idx + 1);
            }

            // Step 2: once we’ve seen a call, look for the balance update
            if let Some(call_ln) = call_line {
                if line.contains("balance[msg.sender] -=") {
                    // Step 3: vulnerable pattern detected
                    let col = line.find(".call").unwrap_or(0) + 1;
                    findings.push(Finding {
                        rule: self.name(),
                        message: "External call before balance update → possible reentrancy".into(),
                        severity: Severity::High,
                        location: Location {
                            line: call_ln,
                            column: col,
                        },
                    });
                    break; // one finding per contract
                }
            }
        }

        findings
    }
}

#[no_mangle]
pub extern "C" fn init_plugin(reg: &mut eth_audit_core::registry::Registry) {
    register_detector(reg, ReentrancyDetector);
}
