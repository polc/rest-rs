pub mod list;
pub mod object;
pub mod resource;
pub mod scalars;

use crate::schema::{Schema, TypeMetadata};
use serde_json::Value;
use std::borrow::Cow;

pub trait Type {
    fn type_id() -> Cow<'static, str>;

    fn type_metadata(schema: &mut Schema) -> TypeMetadata;
}

#[async_trait::async_trait]
pub trait OutputType: Type + Send + Sync {
    async fn resolve(&self) -> Value;
}
