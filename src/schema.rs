use crate::types::resource::{Id, Resource};
use crate::types::Type;
use crate::{ObjectOutputType, OutputType, ResolvedNode, Selection};
use futures::future::BoxFuture;
use futures::TryFutureExt;
use route_recognizer::Router;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: String,
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
    pub fn new_field<'a, T: Type>(schema: &mut Schema<'a>, name: &str) -> FieldMetadata {
        FieldMetadata {
            name: name.to_string(),
            field_type: schema.add_type::<T>(),
        }
    }

    pub fn new_object<'a, T: ObjectOutputType<'a>>(fields: &[FieldMetadata]) -> TypeMetadata {
        TypeMetadata::Object {
            name: T::type_name().to_string(),
            fields: fields.to_vec(),
        }
    }

    pub fn new_resource<T: Type>(schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Resource {
            name: format!("Resource:{}", T::type_name().to_string()),
            item_type: schema.add_type::<T>(),
        }
    }

    pub fn fields<'a>(&'a self, schema: &'a Schema<'a>) -> Option<&'a Vec<FieldMetadata>> {
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

pub type ResourceResolver<'a> = fn(Id, &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>>;

pub struct Schema<'a> {
    types: HashMap<String, TypeMetadata>,
    router: Router<ResourceResolver<'a>>,
}

impl<'a> Schema<'a> {
    pub fn new<Root: Type>() -> Self {
        let mut schema = Schema {
            types: Default::default(),
            router: Router::new(),
        };
        schema.add_type::<Root>();

        schema
    }

    pub fn type_metadata(&self, name: &str) -> &TypeMetadata {
        self.types
            .get(name)
            .expect(format!("Unable to get type metadata {}.", name).as_ref())
    }

    pub fn add_type<T: Type>(&mut self) -> String {
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

    pub fn add_resource_type<T: Resource + OutputType<'a> + Send>(&mut self) -> String {
        let name = self.add_type::<T>();

        if !self.types.contains_key(&name) {
            let uri = format!("/{}/:id", name.clone());

            self.router
                .add(uri.as_str(), |id: Id, node: &'a Selection| {
                    Box::pin(async move { T::get_item(id).await.resolve(node).await })
                });
        }

        name
    }
}
