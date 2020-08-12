use crate::query::NodeSelection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, ResourceList, Type};

use serde_json::Value;
use std::borrow::Cow;

impl<T: Type> Type for [T] {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("[{}]", T::type_name()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::List {
            name: Self::type_name().to_string(),
            item_type: schema.register_type::<T>(),
        }
    }
}

#[async_trait::async_trait]
impl<T: OutputType> OutputType for [T] {
    async fn resolve(&self, selection: &NodeSelection) -> ResolvedNode {
        let futures = self.iter().map(|item| item.resolve(selection));
        let nodes = futures::future::join_all(futures).await;
        let (content, children): (Vec<Value>, Vec<ResourceList>) = nodes
            .into_iter()
            .map(|ResolvedNode(content, children)| (content, children))
            .unzip();

        ResolvedNode(
            Value::Array(content),
            children.into_iter().flatten().collect(),
        )
    }
}
