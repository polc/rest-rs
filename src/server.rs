use crate::types::ResolvedNode;
use crate::schema::Schema;
use futures::future::BoxFuture;
use h2::{server, RecvStream};
use http::{Method, Request, Response, StatusCode};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    pub schema: Schema,
}

impl Server {
    pub async fn run(&self, addr: &str) {
        let mut listener = TcpListener::bind(addr).await.unwrap();

        loop {
            if let Ok((socket, _peer_addr)) = listener.accept().await {
                // let schema = &self.schema;

                // Spawn a new task to process each connection.
                tokio::spawn(async move {
                    // Start the HTTP/2.0 connection handshake
                    let mut h2 = server::handshake(socket).await.unwrap();

                    // Accept all inbound HTTP/2.0 streams sent over the connection.
                    while let Some(request) = h2.accept().await {
                        let (request, mut respond) = request.unwrap();
                        println!("{:?} {:?}", request.method(), request.uri());

                        let (response, body): (Response<()>, Option<serde_json::Value>) =
                            match request.method() {
                                &Method::GET => {
                                    // let route = schema.router.recognize(request.uri().path());

                                    (
                                        Response::builder()
                                            .status(StatusCode::OK)
                                            .body(())
                                            .unwrap(),
                                        None,
                                    )
                                }
                                _ => (
                                    Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(())
                                        .unwrap(),
                                    None,
                                ),
                            };

                        respond.send_response(response, true).unwrap();
                    }
                });
            }
        }
    }
}

pub fn resolve(parent: ResolvedNode) -> BoxFuture<'static, ()> {
    Box::pin(async move {
        let ResolvedNode(content, children_futures) = parent;
        let children = futures::future::join_all(children_futures).await;

        for ResolvedNode(child_content, _) in &children {
            println!("Push-Promise : {:#}", child_content);
        }

        println!("Server Push : {:#}", content);

        let mut futures = Vec::with_capacity(children.len());
        for child in children {
            futures.push(resolve(child));
        }
        futures::future::join_all(futures).await;
    })
}
