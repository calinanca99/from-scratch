use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let addr = "http://127.0.0.1:4000";
    let server = TcpListener::bind("127.0.0.1:4000").expect("Cannot bind socket");

    println!("Listening on: {}", addr);

    for stream in server.incoming() {
        match stream {
            Ok(mut conn) => {
                let mut buf = [0; 1024];

                let bytes_read = conn.read(&mut buf).expect("Cannot read from socket");

                let request = String::from_utf8_lossy(&buf[..bytes_read]);
                let res = if let Some(start_line) = request.lines().nth(0) {
                    println!("start-line: {start_line}");
                    if let Some(route) = start_line.split_whitespace().nth(1) {
                        println!("route: {route}");
                        match route {
                            "/" | "/health_check" => "HTTP/1.1 200 OK",
                            _ => "HTTP/1.1 404 Not Found",
                        }
                    } else {
                        "HTTP/1.1 400 Bad Request"
                    }
                } else {
                    "HTTP/1.1 400 Bad Request"
                };

                conn.write_all(res.as_bytes().as_ref())
                    .expect("Cannot write to socket");
            }
            Err(_) => {
                println!("Connection failed")
            }
        }
    }
}
