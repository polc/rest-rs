use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, Type};
use serde_json::Value;
use std::borrow::Cow;

impl Type for String {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar
    }
}

#[async_trait::async_trait]
impl OutputType for String {
    async fn resolve(&self) -> Value {
        Value::String(self.clone())
    }
}

impl Type for &str {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar
    }
}

#[async_trait::async_trait]
impl OutputType for &str {
    async fn resolve(&self,) -> Value {
        Value::String(self.to_string())
    }
}
