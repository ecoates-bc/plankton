extern crate model_proc_macro;
mod fields;
mod schema;

pub mod prelude {
    pub use crate::fields::*;
    pub use crate::schema::*;
    pub use model_proc_macro::Model;
}

mod tests;
