use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, ResourceList, Type};

use futures::future::BoxFuture;
use std::borrow::Cow;

impl<T: Type> Type for Vec<T> {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("[{}]", T::type_name()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::List {
            name: Self::type_name().to_string(),
            item_type: schema.add_type::<T>(),
        }
    }
}

impl<'a, T: OutputType<'a> + Sync + Send + Copy + 'a> OutputType<'a> for Vec<T> {
    fn resolve(self, parent_node: &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>> {
        Box::pin(async move {
            let futures = self.iter().map(|item| item.resolve(&parent_node));

            let nodes = futures::future::join_all(futures).await;
            let (content, children): (Vec<serde_json::Value>, Vec<ResourceList>) = nodes
                .into_iter()
                .map(|ResolvedNode(content, children)| (content, children))
                .unzip();

            ResolvedNode(
                serde_json::Value::Array(content),
                children.into_iter().flatten().collect(),
            )
        })
    }
}
