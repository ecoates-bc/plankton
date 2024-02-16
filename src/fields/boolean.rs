use std::{fmt::Display, str::FromStr};

use super::ModelField;

/// A boolean value
pub struct BooleanField(bool);

impl Display for BooleanField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "TRUE" } else { "FALSE" })
    }
}

impl ModelField for BooleanField {
    fn schema_type() -> &'static str {
        "BOOLEAN"
    }

    fn schema_value(&self) -> String {
        format!("{}", self)
    }
}

pub struct BoolToFieldError;

impl FromStr for BooleanField {
    type Err = BoolToFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "0" => Ok(BooleanField(false)),
            "1" => Ok(BooleanField(true)),
            "false" => Ok(BooleanField(false)),
            "true" => Ok(BooleanField(true)),
            _ => Err(BoolToFieldError),
        }
    }
}

impl BooleanField {
    fn new() -> Self {
        BooleanField(false)
    }
}
