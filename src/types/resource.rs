use crate::query::NodeSelection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{ObjectOutputType, OutputType, ResolvedNode, ResourceList, Type};

use futures::future::BoxFuture;
use serde_json::Value;
use std::borrow::Cow;

pub trait Resource: OutputType {
    type Id;

    fn get_item(id: Self::Id) -> BoxFuture<'static, Self>;
}

pub struct Link<T: Resource + Clone + 'static>(pub T);

impl<T: Resource + Clone + 'static> Type for Link<T> {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("Resource:{}", T::type_name()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        TypeMetadata::Resource {
            name: Self::type_name().to_string(),
            item_type: schema.register_type::<T>(),
        }
    }
}

#[async_trait::async_trait]
impl<T: Resource + Clone + 'static> OutputType for Link<T> {
    async fn resolve(&self, selection: &NodeSelection) -> ResolvedNode {
        let content = Value::String("/my-resources/321".to_string());

        let node_clone = self.0.clone();
        let selection_clone = selection.clone();

        let children: ResourceList = vec![Box::pin(async move {
            node_clone.resolve(&selection_clone).await
        })];

        ResolvedNode(content, children)
    }
}
