use std::fmt::Display;

use super::ModelField;

/// an integer that can be positive or negative
pub struct SignedIntField(isize);

impl Display for SignedIntField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ModelField for SignedIntField {
    fn schema_type() -> &'static str {
        "INTEGER"
    }

    fn schema_value(&self) -> String {
        format!("{}", self)
    }
}

impl SignedIntField {
    fn new() -> Self {
        SignedIntField(0)
    }
}
