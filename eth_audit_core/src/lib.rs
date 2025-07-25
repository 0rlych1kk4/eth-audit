pub mod registry;
pub mod types;

pub trait Detector {
    fn name(&self) -> &'static str;
    fn run(&self, contract: &types::ParsedContract) -> Vec<types::Finding>;
}
