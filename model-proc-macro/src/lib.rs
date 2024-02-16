extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, token::Mod, Attribute, Data, DataStruct, DeriveInput, Fields, Ident, Meta,
    Type, TypePath,
};

#[proc_macro_derive(Model, attributes(primary_key, not_null, unique))]
pub fn create_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // extract table name and field names
    let name = input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Cannot derive a model from non-struct data."),
    };
    let fields = get_model_fields(&data);
    let schema_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ty = &field.data_type;
            let pk = &field.primary_key;
            let null = &field.null;
            let unique = &field.unique;

            quote! {
                SchemaField {
                    name: stringify!(#name).into(),
                    data_storage_class: #ty::schema_type(),
                    primary_key: #pk,
                    null: #null,
                    unique: #unique,
                }
            }
        })
        .collect();

    // create a ModelManager struct
    let manager_name = format_ident!("{}Manager", name);
    let model_manager_expr = quote! {
        pub struct #manager_name {
            pub schema: Schema,
        }

        impl #manager_name {
            pub fn new() -> Self {
                #manager_name {
                    schema: Schema {
                        table: stringify!(#name).into(),
                        fields: vec![#(#schema_fields),*],
                    }
                }
            }
        }
    };

    TokenStream::from(model_manager_expr)
}

fn get_model_fields(data: &DataStruct) -> Vec<ModelField> {
    match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let f_cloned = f.clone();
                let attributes = parse_field_attrs(&f_cloned.attrs);
                ModelField {
                    name: f_cloned
                        .ident
                        .expect("Unable to get name of model field.")
                        .clone(),
                    data_type: match f_cloned.ty {
                        Type::Path(path) => path,
                        _ => panic!("Expected type path in model field."),
                    },
                    primary_key: attributes.0,
                    null: attributes.1,
                    unique: attributes.2,
                }
            })
            .collect(),
        _ => vec![],
    }
}

fn parse_field_attrs(attrs: &Vec<Attribute>) -> (bool, bool, bool) {
    let mut attr_result = (false, false, false);
    for a in attrs {
        if let Some(attr_name) = a.meta.path().get_ident() {
            match attr_name.to_string().as_str() {
                "primary_key" => attr_result.0 = true,
                "not_null" => attr_result.1 = true,
                "unique" => attr_result.2 = true,
                _ => (),
            }
        }
    }
    attr_result
}

struct ModelField {
    name: Ident,
    data_type: TypePath,
    primary_key: bool,
    null: bool,
    unique: bool,
}
