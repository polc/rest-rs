use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, Type};
use serde_json::Value;
use std::borrow::Cow;

impl Type for String {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar
    }
}

#[async_trait::async_trait]
impl OutputType for String {
    async fn resolve(&self, selection: &Selection) -> ResolvedNode {
        let content = match selection {
            Selection::Scalar => Value::String(self.clone()),
            _ => Value::Null,
        };

        ResolvedNode(content, vec![])
    }
}

impl Type for &str {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar
    }
}

#[async_trait::async_trait]
impl OutputType for &str {
    async fn resolve(&self, selection: &Selection) -> ResolvedNode {
        let content = match selection {
            Selection::Scalar => Value::String(self.to_string()),
            _ => Value::Null,
        };

        ResolvedNode(content, vec![])
    }
}
