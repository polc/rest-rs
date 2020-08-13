# Books example

## Usage

For now, I use [h2](https://github.com/hyperium/h2), so `rest-rs` doesn't support HTTP 1.0 upgrades. 
We need to use an HTTP/2 only client such as `curl` with the `--http2-prior-knowledge` flag.

```bash
# Start server
cargo run --example books 

# Send request
curl --http2-prior-knowledge --trace-ascii - http://localhost:8080/books/book-123
```
