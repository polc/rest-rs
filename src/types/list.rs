use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, Type};

use serde_json::Value;
use std::borrow::Cow;

impl<T: Type> Type for [T] {
    fn type_id() -> Cow<'static, str> {
        Cow::Owned(format!("[{}]", T::type_id()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::List {
            type_id: schema.register_type::<T>(),
        }
    }
}

#[async_trait::async_trait]
impl<T: OutputType> OutputType for [T] {
    async fn resolve(&self) -> Value {
        let futures = self.iter().map(|item| item.resolve());
        let content = futures::future::join_all(futures).await;

        Value::Array(content)
    }
}
