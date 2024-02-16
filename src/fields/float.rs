use std::fmt::Display;

use super::ModelField;

/// A single precision floating point value
pub struct FloatField(f32);

impl Display for FloatField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl ModelField for FloatField {
    fn schema_type() -> &'static str {
        "FLOAT"
    }

    fn schema_value(&self) -> String {
        format!("{}", self)
    }
}

impl FloatField {
    fn new() -> Self {
        FloatField(0.)
    }

    fn from(f: f32) -> Self {
        FloatField(f)
    }
}
