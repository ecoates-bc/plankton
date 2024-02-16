use crate::fields::ModelField;

#[derive(Debug, PartialEq, Eq)]
pub struct Schema {
    pub table: &'static str,
    pub fields: Vec<SchemaField>,
}

impl Schema {
    pub fn generate(&self) -> String {
        let field_defs: Vec<String> = self
            .fields
            .iter()
            .map(|field| field.to_scheme_row())
            .collect();

        format!(
            "CREATE TABLE {} (\n{}\n);",
            self.table,
            field_defs.join(",\n")
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SchemaField {
    pub name: &'static str,
    pub data_storage_class: &'static str,
    pub primary_key: bool,
    pub null: bool,
    pub unique: bool,
}

impl SchemaField {
    fn new(
        name: &'static str,
        data_storage_class: &'static str,
        primary_key: bool,
        null: bool,
        unique: bool,
    ) -> Self {
        SchemaField {
            name,
            data_storage_class,
            primary_key,
            null,
            unique,
        }
    }

    fn to_scheme_row(&self) -> String {
        let mut row = format!("\t{}\t{}", self.name, self.data_storage_class);

        if self.primary_key {
            row = format!("{}\tPRIMARY KEY", row);
        }

        if self.null {
            row = format!("{}\tNOT NULL", row);
        }

        if self.unique {
            row = format!("{}\tUNIQUE", row);
        }

        row
    }
}
