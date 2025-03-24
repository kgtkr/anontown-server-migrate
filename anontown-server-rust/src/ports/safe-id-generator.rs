pub trait SafeIdGeneratorPort: Send + Sync {
    fn generate(&self) -> String;
} 