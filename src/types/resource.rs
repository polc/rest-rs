use crate::query::Selection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, Type};
use futures::future::BoxFuture;
use std::borrow::Cow;

pub struct Id(String);

pub trait Resource: Type {
    fn get_item<'a>(id: Id) -> BoxFuture<'a, Self>;
}

pub struct Link<T: Resource + Sync>(T);

impl<T: Resource + Sync + Send> From<T> for Link<T> {
    fn from(item: T) -> Self {
        Self(item)
    }
}

impl<T: Resource + Sync + Send> Type for Link<T> {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        T::type_metadata(schema)
    }
}

impl<'a, T: Resource + OutputType<'a> + Sync + Send + 'a> OutputType<'a> for Link<T> {
    fn resolve(self, parent_node: &'a Selection) -> BoxFuture<'a, ResolvedNode<'a>> {
        Box::pin(async move {
            let uri = "/my-resources/321".to_string();
            let content = serde_json::Value::String(uri);
            let children = vec![self.0.resolve(&parent_node)];

            ResolvedNode(content, children)
        })
    }
}
