use std::{
    io::{Read, Write},
    net::TcpStream,
};

use http_server::{read_file, server::Server, Method, Request, Response, Status};

fn main() {
    Server::new("127.0.0.1:4000", handle_conn).start();
}

fn handle_conn(conn: &mut TcpStream) {
    let mut buf = vec![0; 1024];
    let _ = conn.read(&mut buf).expect("Cannot read from socket");
    let req = String::from_utf8(buf).expect("Cannot read request");

    let res = match Request::try_from(req.as_str()) {
        Ok(req) => handle_request(&req),
        Err(e) if &e == "method not allowed" => method_not_allowed(),
        Err(_) => bad_request(),
    };

    conn.write_all(res.as_bytes())
        .expect("Cannot write to socket");
}

fn handle_request(req: &Request) -> String {
    match req.method() {
        Method::Get if req.route() == "/" => index(),
        Method::Get if req.route() == "/health_check" => health_check(),
        _ => fallback(),
    }
}

fn index() -> String {
    let body = read_file("public/index.html");
    Response::new(Status::Ok, Some(body)).into()
}

fn health_check() -> String {
    Response::new(Status::Ok, None).into()
}

fn bad_request() -> String {
    Response::new(Status::BadRequest, None).into()
}

fn method_not_allowed() -> String {
    Response::new(Status::MethodNotAllowed, None).into()
}

fn fallback() -> String {
    let body = read_file("public/not_found.html");
    Response::new(Status::NotFound, Some(body)).into()
}
