extern crate rest_rs;

use rest_rs::{
    schema::{Schema, TypeMetadata},
    server::Server,
    types::{
        object::ObjectOutputType,
        resource::{Iri, ResourceRef, Resource},
        OutputType, ResolvedNode, Type,
    },
};

use route_recognizer::Params;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use rest_rs::query::ObjectField;

#[derive(Debug, Clone, Copy)]
pub struct Root {}

impl<'a> Root {
    async fn field_str(&self) -> String {
        "I'm Root !".to_string()
    }

    async fn field_link(&self) -> ResourceRef<Book> {
        ResourceRef(Book {})
    }
}

impl Type for Root {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("Root")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            TypeMetadata::new_field::<ResourceRef<Book>>(schema, "field_link"),
        ];

        TypeMetadata::new_object::<Root>(fields)
    }
}

#[async_trait::async_trait]
impl ObjectOutputType for Root {
    async fn resolve_field(&self, field: &ObjectField) -> (&'static str, ResolvedNode) {
        let resolved_node = match field.name {
            "field_str" => {
                self.field_str()
                    .await
                    .resolve(&field.selection)
                    .await
            }
            "field_link" => {
                self.field_link()
                    .await
                    .resolve(&field.selection)
                    .await
            }
            _ => panic!("Field {} not found on type Root", field.name),
        };

        (field.name, resolved_node)
    }
}

pub struct RootIri {}

impl TryFrom<Params> for RootIri {
    type Error = ();

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        match value.iter().count() {
            0 => Ok(RootIri {}),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RootIri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")
    }
}

impl Iri for RootIri {
    fn path_pattern() -> &'static str {
        "/"
    }
}

#[async_trait::async_trait]
impl Resource for Root {
    type Iri = RootIri;

    fn iri(&self) -> Self::Iri {
        RootIri {}
    }

    async fn fetch(_: Self::Iri) -> Option<Self> {
        Some(Self {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Book {}

impl Book {
    async fn field_str(&self) -> &str {
        "I'm a book !"
    }

    async fn field_recursive(&self) -> ResourceRef<Book> {
        ResourceRef(Book {})
    }
}

impl Type for Book {
    fn type_id() -> Cow<'static, str> {
        Cow::Borrowed("Book")
    }

    fn type_metadata(schema: &mut Schema) -> TypeMetadata {
        let fields = &[
            TypeMetadata::new_field::<&str>(schema, "field_str"),
            TypeMetadata::new_field::<ResourceRef<Book>>(schema, "field_recursive"),
        ];

        TypeMetadata::new_object::<Book>(fields)
    }
}

#[async_trait::async_trait]
impl ObjectOutputType for Book {
    async fn resolve_field(&self, field: &ObjectField) -> (&'static str, ResolvedNode) {
        let resolved_node = match field.name {
            "field_str" => {
                self.field_str()
                    .await
                    .resolve(&field.selection)
                    .await
            }
            "field_recursive" => {
                self.field_recursive()
                    .await
                    .resolve(&field.selection)
                    .await
            }
            _ => panic!("Field {} not found on type Book", field.name),
        };

        (field.name, resolved_node)
    }
}

pub struct BookIri {
    id: String,
}

impl<'a> TryFrom<Params> for BookIri {
    type Error = ();

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        match value.find("id") {
            Some(id) => Ok(BookIri { id: id.into() }),
            _ => Err(()),
        }
    }
}

impl fmt::Display for BookIri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/books/{}", self.id)
    }
}

impl Iri for BookIri {
    fn path_pattern() -> &'static str {
        "/books/:id"
    }
}

#[async_trait::async_trait]
impl Resource for Book {
    type Iri = BookIri;

    fn iri(&self) -> Self::Iri {
        BookIri {
            id: "book-123".into(),
        }
    }

    async fn fetch(route: Self::Iri) -> Option<Self> {
        if route.id.eq("book-123") {
            Some(Self {})
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new::<Root>();
    let server = Server::new(schema);

    server.run("127.0.0.1:8080").await;
}
