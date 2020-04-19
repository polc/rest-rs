use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, Type};
use futures::future::BoxFuture;
use std::borrow::Cow;

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

impl<'a> OutputType<'a> for &'a str {
    fn resolve(self, _parent_node: &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>> {
        Box::pin(async move { ResolvedNode(serde_json::Value::String(self.to_string()), vec![]) })
    }
}
