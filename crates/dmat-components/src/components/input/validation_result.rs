#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid { message: String },
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}
