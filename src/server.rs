use crate::query::NodeSelection;
use crate::schema::{ResourceRoute, Schema};
use crate::types::ResolvedNode;
use futures::future::BoxFuture;
use h2::server;
use h2::server::SendResponse;
use http::{Method, Response, StatusCode};
use hyper::body::Bytes;
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    pub schema: Arc<Schema>,
}

impl Server {
    pub async fn run(&self, addr: &str) {
        let mut listener = TcpListener::bind(addr).await.unwrap();

        loop {
            if let Ok((socket, _peer_addr)) = listener.accept().await {
                let schema = self.schema.clone();

                tokio::spawn(async move {
                    if let Err(error) = handle(socket, schema).await {
                        println!("{:?}", error);
                    }
                });
            }
        }
    }
}

async fn handle(socket: TcpStream, schema: Arc<Schema>) -> Result<(), Box<dyn Error>> {
    let mut connection = server::handshake(socket).await?;

    while let Some(result) = connection.accept().await {
        let (req, mut stream) = result?;

        match schema.router.recognize(req.uri().path()) {
            Ok(route_recognizer) => match req.method() {
                &Method::GET => {
                    let params = route_recognizer.params;
                    let ResourceRoute { name, resolver } = route_recognizer.handler;

                    let type_metadata = schema.type_metadata(name.as_str());
                    let selection = NodeSelection::new("root", type_metadata, &schema);
                    let resolved_node = (resolver)(params, &selection).await;

                    if let Some(resolved_node) = resolved_node {
                        send_root_node(resolved_node, &mut stream).await?;
                    } else {
                        let response = Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(())
                            .unwrap();
                        stream.send_response(response, true).unwrap();
                    }
                }
                _ => {
                    let response = Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(())
                        .unwrap();
                    stream.send_response(response, true).unwrap();
                }
            },
            _ => {
                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(())
                    .unwrap();
                stream.send_response(response, true).unwrap();
            }
        }
    }

    Ok(())
}

async fn send_root_node(
    root: ResolvedNode,
    stream: &mut SendResponse<Bytes>,
) -> Result<(), Box<dyn Error>> {
    let ResolvedNode(content, children_futures) = root;
    let children = futures::future::join_all(children_futures).await;

    for ResolvedNode(child_content, _) in &children {
        println!("Push-Promise : {:#}", child_content);
    }

    println!("Send Response : {:#}", content);
    let res = Response::builder().status(StatusCode::OK).body(()).unwrap();
    let mut send = stream.send_response(res, false)?;

    let content_bytes = Bytes::from(serde_json::to_vec(&content).unwrap());
    send.send_data(content_bytes, true)?;

    let mut futures = Vec::with_capacity(children.len());
    for child in children {
        futures.push(send_node(child));
    }
    futures::future::join_all(futures).await;

    Ok(())
}

pub fn send_node<'a>(node: ResolvedNode) -> BoxFuture<'a, ()> {
    Box::pin(async move {
        let ResolvedNode(content, children_futures) = node;
        let children = futures::future::join_all(children_futures).await;

        for ResolvedNode(child_content, _) in &children {
            println!("Push-Promise : {:#}", child_content);
        }

        println!("Server Push : {:#}", content);

        let mut futures = Vec::with_capacity(children.len());
        for child in children {
            futures.push(send_node(child));
        }
        futures::future::join_all(futures).await;
    })
}
