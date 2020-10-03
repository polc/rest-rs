use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, Type};

use route_recognizer::Params;
use serde_json::Value;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;

pub trait Iri: TryFrom<Params, Error = ()> + Display + Send {
    fn path_pattern() -> &'static str;
}

#[async_trait::async_trait]
pub trait Resource: OutputType + Sized {
    type Iri: Iri;

    fn iri(&self) -> Self::Iri;

    async fn fetch(iri: Self::Iri) -> Option<Self>;
}

pub struct ResourceRef<T: Resource + Clone + 'static>(pub T);

impl<T: Resource + Clone + 'static> Type for ResourceRef<T> {
    fn type_id() -> Cow<'static, str> {
        Cow::Owned(format!("Resource:{}", T::type_id()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        schema.register_resource::<T>();

        TypeMetadata::Resource {
            type_id: schema.register_type::<T>(),
        }
    }
}

#[async_trait::async_trait]
impl<T: Resource + Clone + 'static> OutputType for ResourceRef<T> {
    async fn resolve(&self) -> Value {
        Value::String(self.0.iri().to_string())
    }
}
