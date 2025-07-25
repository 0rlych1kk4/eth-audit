use crate::{Detector, types::ParsedContract};
/// A registry of detectors.
pub struct Registry {
    detectors: Vec<Box<dyn Detector + Send + Sync>>,
}

impl Registry {
    pub fn new() -> Self { Self { detectors: vec![] } }
    pub fn register(&mut self, d: Box<dyn Detector + Send + Sync>) {
        self.detectors.push(d);
    }
    pub fn run_all(&self, contract: &ParsedContract) -> Vec<crate::types::Finding> {
        self.detectors.iter().flat_map(|d| d.run(contract)).collect()
    }
}

/// Helper for plugins to register.
pub fn register_detector(
    reg: &mut Registry,
    d: impl Detector + Send + Sync + 'static
) {
    reg.register(Box::new(d));
}

