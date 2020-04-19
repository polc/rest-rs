pub mod list;
pub mod resource;
pub mod scalars;

use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use futures::future::BoxFuture;
use std::borrow::Cow;

pub type ResourceList<'a> = Vec<BoxFuture<'a, ResolvedNode<'a>>>;
pub struct ResolvedNode<'a>(pub serde_json::Value, pub ResourceList<'a>);

pub trait Type {
    fn type_name() -> Cow<'static, str>;

    fn type_metadata(schema: &mut Schema) -> TypeMetadata;
}

pub trait OutputType<'a>: Type {
    fn resolve(self, node: &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>>;
}

pub trait ObjectOutputType<'a>: Type {
    fn resolve_field(
        self,
        parent_node: &'a Selection,
    ) -> BoxFuture<'a, (&'a str, ResolvedNode<'a>)>;
}

impl<'a, T: ObjectOutputType<'a> + Sync + Send + Copy + 'a> OutputType<'a> for T {
    fn resolve(self, parent_node: &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>> {
        Box::pin(async move {
            let mut futures: Vec<BoxFuture<(&'a str, ResolvedNode<'a>)>> =
                Vec::with_capacity(parent_node.selection_set.nodes.len());

            for node in &parent_node.selection_set.nodes {
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
        })
    }
}
