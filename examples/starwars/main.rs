extern crate rest_rs;

use rest_rs::{
    parse, resolve,
    resource::{Id, Link, Resource},
    ObjectOutputType, OutputType, ResolvedNode, Schema, Selection, Type, TypeMetadata,
};

use futures::future::BoxFuture;
use http::Request;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct Root {}

#[derive(Debug, Clone, Copy)]
pub struct Children {}

impl<'a> Root {
    async fn field_str(&self) -> &'a str {
        "I'm Root !"
    }

    async fn field_link(&self) -> Link<Children> {
        Children {}.into()
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

impl<'a> ObjectOutputType<'a> for Root {
    fn resolve_field(
        self,
        parent_node: &'a Selection,
    ) -> BoxFuture<'a, (&'a str, ResolvedNode<'a>)> {
        Box::pin(async move {
            let resource_value = match parent_node.name {
                "field_str" => self.field_str().await.resolve(&parent_node).await,
                "field_link" => self.field_link().await.resolve(&parent_node).await,
                _ => panic!("Field {} not found on type Root", parent_node.name),
            };

            (parent_node.name, resource_value)
        })
    }
}

impl Resource for Root {
    fn get_item<'a>(id: Id) -> BoxFuture<'a, Self> {
        Box::pin(async { Self {} })
    }
}

impl<'a> Children {
    async fn field_str(&self) -> &'a str {
        "I'm Children !"
    }

    async fn field_recursive(&self) -> Link<Children> {
        Children {}.into()
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

impl<'a> ObjectOutputType<'a> for Children {
    fn resolve_field(
        self,
        parent_node: &'a Selection,
    ) -> BoxFuture<'a, (&'a str, ResolvedNode<'a>)> {
        Box::pin(async move {
            let resource_value = match parent_node.name {
                "field_str" => self.field_str().await.resolve(&parent_node).await,
                "field_recursive" => self.field_recursive().await.resolve(&parent_node).await,
                _ => panic!("Field {} not found on type Children", parent_node.name),
            };

            (parent_node.name, resource_value)
        })
    }
}

impl Resource for Children {
    fn get_item<'a>(id: Id) -> BoxFuture<'a, Self> {
        Box::pin(async { Self {} })
    }
}

#[tokio::main]
async fn main() {
    let request = Request::builder()
        .uri("https://192.168.0.1/children-item/recursive-field")
        .header("Preload", "/recursive_field/recursive_field")
        .body(())
        .unwrap();

    let schema = Schema::new::<Root>();
    let root_node = parse(request, &schema).expect("Unable to parse query.");

    // println!("{:#?}", schema);
    let root_type: Root = Root {};
    let root_resource = root_type.resolve(&root_node).await;

    resolve(root_resource).await;
}
