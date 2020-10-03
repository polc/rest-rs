use crate::query::{ObjectField, Selection};
use crate::types::{OutputType, ResolvedNode};
use serde_json::Value;

#[async_trait::async_trait]
pub trait ObjectOutputType: OutputType {
    async fn resolve_field(&self, field: &ObjectField) -> (&'static str, ResolvedNode);
}

#[async_trait::async_trait]
impl<T: ObjectOutputType> OutputType for T {
    async fn resolve(&self, selection: &Selection) -> ResolvedNode {
        match selection {
            Selection::Object(fields) => {
                let futures: Vec<_> = fields
                    .iter()
                    .map(|field| self.resolve_field(field))
                    .collect();

                let mut object_content = serde_json::Map::with_capacity(futures.len());
                let mut object_children = Vec::with_capacity(futures.len());
                let resolved_fields = futures::future::join_all(futures).await;

                for (field_name, ResolvedNode(content, mut children)) in resolved_fields {
                    object_content.insert(field_name.to_string(), content);
                    object_children.append(&mut children);
                }

                ResolvedNode(object_content.into(), object_children)
            },
            _ => ResolvedNode(Value::Null, vec![]),
        }
    }
}
