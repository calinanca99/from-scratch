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
    let request = String::from_utf8(buf[..bytes_read].to_vec()).expect("Cannot parse request");

    let res = match handle_request(&request) {
        Ok(res) => res,
        Err(_) => "HTTP/1.1 400 Bad Request",
    };

    conn.write_all(res.as_bytes().as_ref())
        .expect("Cannot write to socket");
}

fn handle_request(req: &str) -> Result<&'static str, String> {
    let start_line = req.lines().nth(0).ok_or("bad-request".to_string())?;
    let route = start_line
        .split_whitespace()
        .nth(1)
        .ok_or("bad-request".to_string())?;

    match route {
        "/" => root(),
        "/health_check" => health_check(),
        _ => fallback(),
    }
}

fn root() -> Result<&'static str, String> {
    Ok("HTTP/1.1 200 OK")
}

fn health_check() -> Result<&'static str, String> {
    Ok("HTTP/1.1 200 OK")
}

fn fallback() -> Result<&'static str, String> {
    Ok("HTTP/1.1 404 Not Found")
}
