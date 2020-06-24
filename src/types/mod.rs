pub mod list;
pub mod resource;
pub mod scalars;

use crate::query::NodeSelection;
use crate::schema::{Schema, TypeMetadata};
use futures::future::BoxFuture;
use serde_json::Value;
use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;

pub type ResourceList = Vec<BoxFuture<'static, ResolvedNode>>;
pub struct ResolvedNode(pub Value, pub ResourceList);

pub trait Type {
    fn type_name() -> Cow<'static, str>;

    fn type_metadata(schema: &mut Schema) -> TypeMetadata;
}

#[async_trait::async_trait]
pub trait OutputType: Type + Send + Sync {
    async fn resolve(&self, selection: &NodeSelection) -> ResolvedNode;
}

#[async_trait::async_trait]
pub trait ObjectOutputType: OutputType {
    async fn resolve_field(&self, selection: &NodeSelection) -> (&'static str, ResolvedNode);
}

#[async_trait::async_trait]
impl<T: ObjectOutputType> OutputType for T {
    async fn resolve(&self, selection: &NodeSelection) -> ResolvedNode {
        let mut futures = Vec::with_capacity(selection.nodes.len());
        for node in &selection.nodes {
            futures.push(self.resolve_field(&node));
        }

        let mut object_content = serde_json::Map::with_capacity(futures.len());
        let mut object_children = Vec::with_capacity(futures.len());
        let objects = futures::future::join_all(futures).await;

        for (field_name, ResolvedNode(content, mut children)) in objects {
            object_content.insert(field_name.to_string(), content);
            object_children.append(&mut children);
        }

        ResolvedNode(object_content.into(), object_children)
    }
}
