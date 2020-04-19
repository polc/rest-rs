use crate::types::ResolvedNode;
use futures::future::BoxFuture;

pub fn resolve(parent: ResolvedNode) -> BoxFuture<()> {
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
