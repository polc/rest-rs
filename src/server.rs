use crate::query::Selection;
use crate::schema::{RouteDefinition, Schema};

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server as HyperServer};
use hyper::service::{make_service_fn, service_fn};

pub struct Server {
    pub schema: Schema,
}

impl Server {
    pub fn new(schema: Schema) -> Self {
        Server { schema }
    }

    pub async fn run(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(async move |req: Request<Body>| {
                Ok::<_, Infallible>(Response::<Body>::new("Hello, World".into()))
            }))
        });

        let server = HyperServer::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}
