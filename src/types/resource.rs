use crate::query::NodeSelection;
use crate::schema::{Schema, TypeMetadata};
use crate::types::{OutputType, ResolvedNode, ResourceList, Type};

use route_recognizer::Params;
use serde_json::Value;
use std::borrow::Cow;
use std::convert::TryFrom;

pub trait Route: TryFrom<Params, Error = ()> + Send {
    fn iri(&self) -> String;
    fn path_pattern() -> &'static str;
}

#[async_trait::async_trait]
pub trait Resource: OutputType + Sized {
    type Route: Route;

    fn route(&self) -> Self::Route;

    async fn fetch(route: Self::Route) -> Option<Self>;
}

pub struct Link<T: Resource + Clone + 'static>(pub T);

impl<T: Resource + Clone + 'static> Type for Link<T> {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("Resource:{}", T::type_name()))
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        schema.register_resource::<T>();

        TypeMetadata::Resource {
            name: Self::type_name().to_string(),
            item_type: schema.register_type::<T>(),
        }
    }
}

#[async_trait::async_trait]
impl<T: Resource + Clone + 'static> OutputType for Link<T> {
    async fn resolve(&self, selection: &NodeSelection) -> ResolvedNode {
        let node_clone = self.0.clone();
        let selection_clone = selection.clone();

        let content = Value::String(node_clone.route().iri());
        let children: ResourceList = vec![Box::pin(async move {
            node_clone.resolve(&selection_clone).await
        })];

        ResolvedNode(content, children)
    }
}
