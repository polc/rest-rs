extern crate rest_rs;

use rest_rs::{
    query::NodeSelection,
    schema::{Schema, TypeMetadata},
    server::Server,
    types::{
        resource::{Link, Resource},
        ObjectOutputType, OutputType, ResolvedNode, Type,
    },
};

use futures::core_reexport::convert::TryFrom;
use rest_rs::types::resource::Route;
use route_recognizer::Params;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Root {}

impl<'a> Root {
    async fn field_str(&self) -> String {
        "I'm Root !".to_string()
    }

    async fn field_link(&self) -> Link<Book> {
        Link(Book {})
    }
}

impl Type for Root {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Root")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            TypeMetadata::new_field::<Link<Book>>(schema, "field_link"),
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

pub struct RootRoute {}

impl TryFrom<Params> for RootRoute {
    type Error = ();

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        match value.iter().count() {
            0 => Ok(RootRoute {}),
            _ => Err(()),
        }
    }
}

impl Route for RootRoute {
    fn iri(&self) -> String {
        "/".into()
    }

    fn path_pattern() -> &'static str {
        "/"
    }
}

#[async_trait::async_trait]
impl Resource for Root {
    type Route = RootRoute;

    fn route(&self) -> Self::Route {
        RootRoute {}
    }

    async fn fetch(route: Self::Route) -> Option<Self> {
        Some(Self {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Book {}

impl Book {
    async fn field_str(&self) -> String {
        "I'm a book !".to_string()
    }

    async fn field_recursive(&self) -> Link<Book> {
        Link(Book {})
    }
}

impl Type for Book {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Book")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            // TypeMetadata::new_field::<Link<Book>>(schema, "field_recursive"),
        ];

        TypeMetadata::new_object::<Book>(fields)
    }
}

#[async_trait::async_trait]
impl ObjectOutputType for Book {
    async fn resolve_field(&self, selection: &NodeSelection) -> (&'static str, ResolvedNode) {
        let resolved_node = match selection.name {
            "field_str" => self.field_str().await.resolve(&selection).await,
            "field_recursive" => self.field_recursive().await.resolve(&selection).await,
            _ => panic!("Field {} not found on type Book", selection.name),
        };

        (selection.name, resolved_node)
    }
}

pub struct BookRoute {
    id: String,
}

impl<'a> TryFrom<Params> for BookRoute {
    type Error = ();

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        match value.find("id") {
            Some(id) => Ok(BookRoute { id: id.into() }),
            _ => Err(()),
        }
    }
}

impl<'a> Route for BookRoute {
    fn iri(&self) -> String {
        format!("/books/{}", self.id)
    }

    fn path_pattern() -> &'static str {
        "/books/:id"
    }
}

#[async_trait::async_trait]
impl Resource for Book {
    type Route = BookRoute;

    fn route(&self) -> Self::Route {
        BookRoute {
            id: "book-123".into(),
        }
    }

    async fn fetch(route: Self::Route) -> Option<Self> {
        if route.id.eq("book-123") {
            Some(Self {})
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Arc::new(Schema::new::<Root>());
    let server = Server { schema };

    server.run("127.0.0.1:8080").await;
}
