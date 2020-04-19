# rest-rs 

A web framework to create client-driven REST APIs.

## Features 

* Hypermedia links
* Resolve and send graphs of resources using [Vulcain](https://github.com/dunglas/vulcain)

## What's next 

#### Planned features

* Per resource headers (cache, sunset, ...)
* Content negotiation with JSON-LD and OpenAPI support

#### Potential features (long-term)

* Support for Hydra or Siren or https://level3.rest/
* Native [Mercure](https://github.com/dunglas/mercure) support
* Proxy automatically to legacy APIs by reading OpenAPI schema or GraphQL schema

#### Documentation article ideas

* Why and how to use a cache proxy
* How to build authentication/authorization using Cookies and not JWT
* How to build a password reset workflow with Paseto (https://paseto.io/)
* How to generate clients from OpenAPI schemas, how to deal with deployments where clients and API version diverge
