use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use http_server::{open_file, Method, Request, Response, Status};

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
        Method::GET if req.route() == "/" => index(),
        Method::GET if req.route() == "/health_check" => health_check(),
        _ => fallback(),
    }
}

fn index() -> String {
    let body = open_file("public/index.html");
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
    let body = open_file("public/not_found.html");
    Response::new(Status::NotFound, Some(body)).into()
}
