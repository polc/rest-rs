# rest-rs (wip)

A web framework to create client-driven REST APIs.

## Features 

* Allow defining resources as a graph like a GraphQL server
* Implement the [Vulcain](https://github.com/dunglas/vulcain) specification to fetch graphs of resources efficiently

## Usage

For now, I use [h2](https://github.com/hyperium/h2), so `rest-rs` doesn't support HTTP 1.0 upgrades. 
We need to use an HTTP/2 only client such as `curl` with the `--http2-prior-knowledge` flag.

```bash
# Start server
cargo run --example books 

# Send request
curl --http2-prior-knowledge --trace-ascii - http://localhost:8080/Root/1
```

## Next

#### Planned features

* Rust macro to automatically implement the `Type` and `ObjectOutputType` traits
* Resource config (path, cache headers, sunset header, ...)
* Content negotiation with JSON-LD and OpenAPI support
* Write operations support :
    * `PATCH` method with `application/merge-patch+json` or maybe `application/json-patch+json`
    * `DELETE` method

#### Potential features (long-term)

* Support for Hydra or Siren or https://level3.rest/
* Native [Mercure](https://github.com/dunglas/mercure) support
* Proxy automatically to legacy APIs by reading OpenAPI schema or GraphQL schema

## Pitch 

`rest-rs` is for people who want to build standard and correct RESTful APIs. 
You will likely not get the best raw performances (reqs/s), but it will feel faster to clients taking full advantage of Vulcain. 

`rest-rs` is best suited for use-case where GraphQL was considered : Dashboards, SPAs, and any clients consuming large graph of resources.

`rest-rs` give developers the freedom of designing their APIs as they want, even in the smallest details :  
  * Resources of any shapes : object, list, string or number
  * URLs of any shapes (`/author-books/1`, `/authors/1/books/1`, ...)

