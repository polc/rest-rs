use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, ResourceList, Type};

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
    async fn resolve(&self, selection: &Selection) -> ResolvedNode {
        match selection {
            Selection::List(_, item_selection) => {
                let futures = self.iter().map(|item| item.resolve(item_selection));
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
            _ => ResolvedNode(Value::Null, vec![]),
        }
    }
}
