use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let addr = "http://127.0.0.1:4000";
    let server = TcpListener::bind("127.0.0.1:4000").expect("Cannot bind socket");

    println!("Listening on: {}", addr);

    for stream in server.incoming() {
        match stream {
            Ok(mut conn) => handle_conn(&mut conn),
            Err(_) => {
                println!("Connection failed")
            }
        }
    }
}

fn handle_conn(conn: &mut TcpStream) {
    let mut buf = [0; 1024];
    let bytes_read = conn.read(&mut buf).expect("Cannot read from socket");
    let req = String::from_utf8(buf[..bytes_read].to_vec()).expect("Cannot read request");

    let res = match Request::try_from(req.as_ref()) {
        Ok(req) => handle_request(&req),
        Err(e) if &e == "method not allowed" => method_not_allowed(),
        Err(_) => bad_request(),
    };

    conn.write_all(res.as_bytes().as_ref())
        .expect("Cannot write to socket");
}

fn handle_request(req: &Request) -> &'static str {
    match req.method {
        Method::GET if &req.route == "/" => root(),
        Method::GET if &req.route == "/health_check" => health_check(),
        _ => fallback(),
    }
}

#[derive(Clone, Copy)]
enum Method {
    GET,
}

impl TryFrom<&str> for Method {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            _ => Err("method not allowed".to_string()),
        }
    }
}

struct Request {
    method: Method,
    route: String,
}

impl TryFrom<&str> for Request {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let method = Method::try_from(
            value
                .split_whitespace()
                .nth(0)
                .ok_or("bad request".to_string())?,
        )?;
        let route = value
            .split_whitespace()
            .nth(1)
            .ok_or("bad request".to_string())?;

        Ok(Self {
            method: method,
            route: route.to_string(),
        })
    }
}

fn root() -> &'static str {
    "HTTP/1.1 200 OK"
}

fn health_check() -> &'static str {
    "HTTP/1.1 200 OK"
}

fn bad_request() -> &'static str {
    "HTTP/1.1 400 Bad Request"
}

fn method_not_allowed() -> &'static str {
    "HTTP/1.1 405 Method Not Allowed"
}

fn fallback() -> &'static str {
    "HTTP/1.1 404 Not Found"
}
