use crate::query::Selection;
use crate::types::object::ObjectOutputType;
use crate::types::resource::{Iri, ResourceRef, Resource};
use crate::types::{OutputType, Type};
use futures::future::BoxFuture;
use route_recognizer::{Params, Router};
use std::collections::HashMap;
use std::convert::TryFrom;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: &'static str,
    pub type_id: String,
}

#[derive(Debug)]
pub enum TypeMetadata {
    Scalar,
    Object { fields: Vec<FieldMetadata> },
    List { type_id: String },
    Resource { type_id: String },
}

impl TypeMetadata {
    pub fn new_field<T: Type>(schema: &mut Schema, name: &'static str) -> FieldMetadata {
        FieldMetadata {
            name,
            type_id: schema.register_type::<T>(),
        }
    }

    pub fn new_object<T: ObjectOutputType>(fields: &[FieldMetadata]) -> TypeMetadata {
        TypeMetadata::Object {
            fields: fields.to_vec(),
        }
    }

    pub fn fields<'a>(&'a self, schema: &'a Schema) -> &[FieldMetadata] {
        match self {
            TypeMetadata::Object { fields } => fields,
            TypeMetadata::Resource { type_id } | TypeMetadata::List { type_id } => {
                schema.type_metadata(type_id).fields(&schema)
            }
            TypeMetadata::Scalar => &[],
        }
    }
}

pub struct RouteDefinition {
    pub type_id: String,
    pub resolver: fn(Params, &Selection) -> BoxFuture<'static, Option<Value>>,
}

pub struct Schema {
    pub types: HashMap<String, TypeMetadata>,
    pub router: Router<RouteDefinition>,
}

impl Schema {
    pub fn new<T: Resource>() -> Self {
        let mut schema = Schema {
            types: Default::default(),
            router: Router::new(),
        };
        schema.register_type::<T>();
        schema.register_resource::<T>();

        schema
    }

    pub fn type_metadata(&self, type_id: &str) -> &TypeMetadata {
        self.types
            .get(type_id)
            .expect(format!("Unable to get type metadata {}.", type_id).as_ref())
    }

    pub fn register_type<T: Type>(&mut self) -> String {
        let type_id = T::type_id().to_string();

        if !self.types.contains_key(&type_id) {
            // first add a placeholder type to allow recursive types
            self.types.insert(type_id.clone(), TypeMetadata::Scalar);

            let type_metadata = T::type_metadata(self);
            self.types.insert(type_id.clone(), type_metadata);
        }

        type_id
    }

    pub fn register_resource<T: Resource>(&mut self) {
        let route_definition = RouteDefinition {
            type_id: T::type_id().to_string(),
            resolver: |params: Params, selection: &Selection| {
                Box::pin(async move {
                    match T::Iri::try_from(params) {
                        Ok(route) => match T::fetch(route).await {
                            Some(resource) => Some(resource.resolve().await),
                            _ => None,
                        },
                        _ => None,
                    }
                })
            },
        };

        self.router.add(T::Iri::path_pattern(), route_definition);
    }
}
