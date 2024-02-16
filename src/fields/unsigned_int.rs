use std::fmt::Display;

use super::ModelField;

/// A strictly positive integer
pub struct UnsignedIntField(usize);

impl Display for UnsignedIntField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ModelField for UnsignedIntField {
    fn schema_type() -> &'static str {
        "INTEGER"
    }

    fn schema_value(&self) -> String {
        format!("{}", self)
    }
}

impl UnsignedIntField {
    fn new() -> Self {
        UnsignedIntField(0)
    }
}
