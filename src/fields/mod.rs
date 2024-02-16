pub trait ModelField {
    fn schema_type() -> &'static str;
    fn schema_value(&self) -> String;
}

mod text;
pub use text::{StrToFieldError, TextField};

mod unsigned_int;
pub use unsigned_int::UnsignedIntField;

mod signed_int;
pub use signed_int::SignedIntField;

mod float;
pub use float::FloatField;

mod boolean;
pub use boolean::BooleanField;
