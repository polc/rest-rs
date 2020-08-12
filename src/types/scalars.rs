use crate::query::NodeSelection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, Type};
use serde_json::Value;
use std::borrow::Cow;

impl Type for String {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar {
            name: Self::type_name().to_string(),
        }
    }
}

#[async_trait::async_trait]
impl OutputType for String {
    async fn resolve(&self, _: &NodeSelection) -> ResolvedNode {
        ResolvedNode(Value::String(self.clone()), vec![])
    }
}

impl Type for &str {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }

    fn type_metadata(_schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Scalar {
            name: Self::type_name().to_string(),
        }
    }
}

#[async_trait::async_trait]
impl OutputType for &str {
    async fn resolve(&self, _: &NodeSelection) -> ResolvedNode {
        ResolvedNode(Value::String(self.to_string()), vec![])
    }
}
