use crate::schema::{Schema, TypeMetadata};
use http::Request;
use std::collections::HashMap;

mod parser;

#[derive(Debug, Clone)]
pub enum Selection {
    Scalar,
    Object(Vec<ObjectField>),
    List(Box<Selection>),
}

#[derive(Debug, Clone)]
pub struct ObjectField {
    pub name: &'static str,
    pub selection: Selection,
}

impl Selection {
    pub fn resource(type_id: &str, schema: &Schema) -> Self {
        let type_metadata = schema.type_metadata(type_id);

        match type_metadata {
            TypeMetadata::Scalar => Selection::Scalar,
            TypeMetadata::Object { fields } => Selection::Object(
                fields
                    .iter()
                    .map(|field| ObjectField {
                        name: field.name,
                        selection: Selection::resource(&field.type_id, schema),
                    })
                    .collect(),
            ),
            TypeMetadata::List { type_id } => Selection::List(
                Box::new(Selection::resource(type_id, schema)),
            ),
            TypeMetadata::Resource { .. } => Selection::Scalar,
        }
    }
}

