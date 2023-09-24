# `HTTP Server`

Multithreaded HTTP server written from scratch using the `std::net` primitives. The following endpoints are available:

- GET /
- GET /health_check

## Running

```
cargo run --release
```

Now you can visit: http://127.0.0.1:4000/

## Using `curl`

```
curl --verbose http://localhost:4000
```
