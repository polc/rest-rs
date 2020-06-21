use crate::types::resource::Resource;
use crate::types::{Type, ObjectOutputType};
use futures::future::BoxFuture;
use futures::TryFutureExt;
use route_recognizer::Router;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: &'static str,
    pub field_type: String,
}

#[derive(Debug)]
pub enum TypeMetadata {
    Scalar {
        name: String,
    },
    Object {
        name: String,
        fields: Vec<FieldMetadata>,
    },
    List {
        name: String,
        item_type: String,
    },
    Resource {
        name: String,
        item_type: String,
    },
}

impl TypeMetadata {
    pub fn new_field<T: Type>(schema: &mut Schema, name: &'static str) -> FieldMetadata {
        FieldMetadata {
            name,
            field_type: schema.register_type::<T>(),
        }
    }

    pub fn new_object<T: ObjectOutputType>(fields: &[FieldMetadata]) -> TypeMetadata {
        TypeMetadata::Object {
            name: T::type_name().to_string(),
            fields: fields.to_vec(),
        }
    }

    pub fn fields<'a>(&'a self, schema: &'a Schema) -> Option<&'a Vec<FieldMetadata>> {
        match self {
            TypeMetadata::Object { fields, .. } => Some(fields),
            TypeMetadata::Resource { item_type, .. } | TypeMetadata::List { item_type, .. } => {
                let type_metadata = schema.type_metadata(item_type.as_ref());

                type_metadata.fields(&schema)
            }
            TypeMetadata::Scalar { .. } => None,
        }
    }
}

pub struct Schema {
    types: HashMap<String, TypeMetadata>,
}

impl Schema {
    pub fn new<T: Resource>() -> Self {
        let mut schema = Schema {
            types: Default::default(),
        };
        schema.register_type::<T>();

        schema
    }

    pub fn type_metadata(&self, name: &str) -> &TypeMetadata {
        self.types
            .get(name)
            .expect(format!("Unable to get type metadata {}.", name).as_ref())
    }

    pub fn register_type<T: Type>(&mut self) -> String {
        let name = T::type_name().to_string();

        if !self.types.contains_key(&name) {
            self.types.insert(
                name.clone(),
                TypeMetadata::Scalar {
                    name: "placeholder".to_string(),
                },
            );

            let type_metadata = T::type_metadata(self);
            self.types.insert(name.clone(), type_metadata);
        }

        name
    }
}
