pub trait ObjectIdGeneratorPort: Send + Sync {
    fn generate(&self) -> String;
} 