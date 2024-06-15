/// Trait for completion handling.
pub trait Completion {
    fn get(&self, input: &str) -> Option<Vec<&str>>;
}
