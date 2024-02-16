use super::ModelField;
use std::str::FromStr;

/// A span of text.
pub struct TextField(String);

impl ToString for TextField {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub struct StrToFieldError;

impl FromStr for TextField {
    type Err = StrToFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TextField(String::from(s)))
    }
}

impl ModelField for TextField {
    fn schema_type() -> &'static str {
        "TEXT"
    }

    fn schema_value(&self) -> String {
        self.0.clone()
    }
}

impl TextField {
    fn new() -> Self {
        TextField(String::new())
    }
}
