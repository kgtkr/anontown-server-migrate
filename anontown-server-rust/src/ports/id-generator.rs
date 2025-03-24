pub trait IdGeneratorPort: Send + Sync {
    fn generate(&self) -> String;
} 