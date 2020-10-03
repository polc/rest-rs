use crate::types::{OutputType};
use serde_json::Value;
use crate::schema::Schema;

#[async_trait::async_trait]
pub trait ObjectOutputType: OutputType {
    async fn resolve_field(&self, field: &'static str) -> (&'static str, Value);
}

#[async_trait::async_trait]
impl<T: ObjectOutputType> OutputType for T {
    async fn resolve(&self, schema: &Schema) -> Value {
        let futures: Vec<_> = fields
            .iter()
            .map(|field| self.resolve_field(field))
            .collect();

        let mut object_content = serde_json::Map::with_capacity(futures.len());
        let resolved_fields = futures::future::join_all(futures).await;

        for (field_name, content) in resolved_fields {
            object_content.insert(field_name.to_string(), content);
        }

        object_content.into()
    }
}
