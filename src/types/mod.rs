pub mod list;
pub mod object;
pub mod resource;
pub mod scalars;

use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use futures::future::BoxFuture;
use serde_json::Value;
use std::borrow::Cow;

pub type ResourceList = Vec<BoxFuture<'static, ResolvedNode>>;
pub struct ResolvedNode(pub Value, pub ResourceList);

pub trait Type {
    fn type_id() -> Cow<'static, str>;

    fn type_metadata(schema: &mut Schema) -> TypeMetadata;
}

#[async_trait::async_trait]
pub trait OutputType: Type + Send + Sync {
    async fn resolve(&self, selection: &Selection) -> ResolvedNode;
}
