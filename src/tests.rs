use super::fields::*;
use super::schema::*;
use model_proc_macro::Model;

#[derive(Model)]
struct SampleModel {
    #[primary_key]
    pub id: UnsignedIntField,
    pub description: TextField,
}

#[derive(Model)]
struct FancierModel {
    #[primary_key]
    pub id: UnsignedIntField,
    #[not_null]
    pub percentage: FloatField,
    #[unique]
    pub is_on: BooleanField,
}

#[test]
fn test_storageclass() {
    let sclass = TextField::schema_type();
    assert_eq!(sclass, "TEXT")
}

#[test]
fn simple_sample_model() {
    let manager = SampleModelManager::new();
    assert_eq!(
        Schema {
            table: "SampleModel",
            fields: vec![
                SchemaField {
                    name: "id",
                    data_storage_class: "INTEGER",
                    primary_key: true,
                    null: false,
                    unique: false,
                },
                SchemaField {
                    name: "description",
                    data_storage_class: "TEXT",
                    primary_key: false,
                    null: false,
                    unique: false,
                }
            ]
        },
        manager.schema
    );
}

#[test]
fn sample_model_schema() {
    let manager = SampleModelManager::new();
    assert_eq!(
        "CREATE TABLE SampleModel (\n\tid\tINTEGER\tPRIMARY KEY,\n\tdescription\tTEXT\n);",
        manager.schema.generate()
    )
}

#[test]
fn test_fancier_model() {
    let manager = FancierModelManager::new();
    assert_eq!(
        Schema {
            table: "FancierModel",
            fields: vec![
                SchemaField {
                    name: "id",
                    data_storage_class: "INTEGER",
                    primary_key: true,
                    null: false,
                    unique: false,
                },
                SchemaField {
                    name: "percentage",
                    data_storage_class: "FLOAT",
                    primary_key: false,
                    null: true,
                    unique: false,
                },
                SchemaField {
                    name: "is_on",
                    data_storage_class: "BOOLEAN",
                    primary_key: false,
                    null: false,
                    unique: true,
                }
            ]
        },
        manager.schema
    )
}

#[test]
fn test_fancier_schema() {
    let manager = FancierModelManager::new();
    assert_eq!(
        "CREATE TABLE FancierModel (\n\tid\tINTEGER\tPRIMARY KEY,\n\tpercentage\tFLOAT\tNOT NULL,\n\tis_on\tBOOLEAN\tUNIQUE\n);",
        manager.schema.generate()
    )
}
