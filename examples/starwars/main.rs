extern crate rest_rs;

use rest_rs::{
    query::NodeSelection,
    schema::{Schema, TypeMetadata},
    server::{resolve, Server},
    types::{
        resource::{Link, Resource},
        ObjectOutputType, OutputType, ResolvedNode, Type,
    },
};

use futures::future::BoxFuture;
use http::Request;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct Root {}

#[derive(Debug, Clone, Copy)]
pub struct Children {}

impl<'a> Root {
    async fn field_str(&self) -> String {
        "I'm Root !".to_string()
    }

    async fn field_link(&self) -> Link<Children> {
        Link(Children {})
    }
}

impl Type for Root {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Root")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            TypeMetadata::new_field::<Link<Children>>(schema, "field_link"),
        ];

        TypeMetadata::new_object::<Root>(fields)
    }
}

#[async_trait::async_trait]
impl ObjectOutputType for Root {
    async fn resolve_field(&self, selection: &NodeSelection) -> (&'static str, ResolvedNode) {
        let resolved_node = match selection.name {
            "field_str" => self.field_str().await.resolve(&selection).await,
            "field_link" => self.field_link().await.resolve(&selection).await,
            _ => panic!("Field {} not found on type Root", selection.name),
        };

        (selection.name, resolved_node)
    }
}

impl Resource for Root {
    type Id = String;

    fn get_item(id: Self::Id) -> BoxFuture<'static, Self> {
        Box::pin(async { Self {} })
    }
}

impl Children {
    async fn field_str(&self) -> String {
        "I'm Children !".to_string()
    }

    async fn field_recursive(&self) -> Link<Children> {
        Link(Children {})
    }
}

impl Type for Children {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Children")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            // TypeMetadata::new_field::<Link<Children>>(schema, "field_recursive"),
        ];

        TypeMetadata::new_object::<Children>(fields)
    }
}

#[async_trait::async_trait]
impl ObjectOutputType for Children {
    async fn resolve_field(&self, selection: &NodeSelection) -> (&'static str, ResolvedNode) {
        let resolved_node = match selection.name {
            "field_str" => self.field_str().await.resolve(&selection).await,
            "field_recursive" => self.field_recursive().await.resolve(&selection).await,
            _ => panic!("Field {} not found on type Children", selection.name),
        };

        (selection.name, resolved_node)
    }
}

impl Resource for Children {
    type Id = String;

    fn get_item(id: Self::Id) -> BoxFuture<'static, Self> {
        Box::pin(async { Self {} })
    }
}

#[tokio::main]
async fn main() {
    let mut schema = Schema::new::<Root>();
    let root_selection = NodeSelection::new("root", schema.type_metadata("Root"), &schema);

    let root_type: Root = Root {};
    let root_resource = root_type.resolve(&root_selection).await;

    resolve(root_resource).await;

    let server = Server { schema };
    server.run("127.0.0.1:8080").await;
}
