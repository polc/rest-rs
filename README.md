# rest-rs (wip)

A web framework to create client-driven REST APIs.

## Features 

* Allow defining resources as a graph like a GraphQL server
* Allow querying a subset of the resource graph (for now by building a `query::NodeSelection` by hand)

## Pitch

`rest-rs` is for people who want to build standard and correct RESTful APIs.
You will likely not get the best raw performances (reqs/s), but it will feel faster to clients taking full advantage of Vulcain.

`rest-rs` is best suited for use-case where GraphQL was considered : Dashboards, SPAs, and any clients consuming large graph of resources.

`rest-rs` give developers the freedom of designing their APIs as they want, even in the smallest details :
  * Resources of any shapes : object, list, string, number, ...
  * URLs of any shapes (`/author-books/1`, `/authors/1/books/1`, ...)

## What it will look like

Here is how I would like the final code to look like. If you prefer a working example in the current state of the code, checkout [examples/books](./examples/books).

First we define a `Root` resource which will be the home page (`/`) and just contain a link to the authors (`/authors`).

Notice the `async fn authors(&self) -> Link<Collection<Author>>` function, which allow to fetch all authors in case a `Preload: /authors` header is present.

```rust
//
// Defining the Root resource
//

#[derive(Debug, Clone, Copy)]
struct Root {}

rest_rs::route!(struct RootRoute = "/");

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

#[rest_rs::resource]
impl Root {
    async fn authors(&self) -> Link<Collection<Author>> {
        let authors = ... // fetch all authors

        Link(authors)
    }
}
```

Then we define a `Collection<Author>` (`/authors`) and a `Author` (`/authors/some-id`) resources.

```rust
#[derive(Debug, Clone, Copy)]
struct Author {
    id: u32,
    name: String,
}

//
// Defining the AuthorCollection resource
//

rest_rs::route!(struct AuthorCollectionRoute = "/authors");

#[async_trait::async_trait]
impl Resource for Collection<Author> {
    type Route = AuthorCollectionRoute;

    fn route(&self) -> Self::Route {
        AuthorCollectionRoute {}
    }

    async fn fetch(route: Self::Route) -> Option<Self> {
        let authors = ... // fetch all authors

        Some(authors)
    }
}

//
// Defining the Author resource
//

rest_rs::route!(struct AuthorRoute = "/author/:id");

#[async_trait::async_trait]
impl Resource for Author {
    type Route = AuthorRoute;

    fn route(&self) -> Self::Route {
        AuthorRoute {
            id: self.id.into(),
        }
    }

    async fn fetch(route: Self::Route) -> Option<Self> {
        let author_id = route.id;
        let author = ... // fetch author by id

        Some(author)
    }
}

#[rest_rs::resource]
impl Author {
    async fn name(&self) -> String {
        self.name
    }

    // not called if filtered by a Fields header
    async fn expensive_field(&self) -> u32 {
        42
    }
}
```

Finally we create a schema from the `Root` resource, and start a new server.

```rust
#[tokio::main]
async fn main() {
    let schema = Arc::new(Schema::new::<Root>());
    let server = Server { schema };

    server.run("127.0.0.1:8080").await;
}
```

## Next

#### Planned features

* Create a `query::NodeSelection` from a HTTP request (by reading Vulcain headers)
* Rust macros (a lot of code in `examples/books/` will be auto-generated)
* Content negotiation with JSON-LD and OpenAPI support
* Write operations support :
    * `PATCH` method with `application/merge-patch+json` or maybe `application/json-patch+json`
    * `DELETE` method

#### Potential features (long-term)

* Support for Hydra or Siren or https://level3.rest/
* Native [Mercure](https://github.com/dunglas/mercure) support
* Proxy automatically to legacy APIs by reading OpenAPI schema or GraphQL schema
