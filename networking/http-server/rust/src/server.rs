use std::{
    net::{TcpListener, TcpStream},
    thread,
};

pub struct Server {
    addr: String,
    conn_handler: fn(&mut TcpStream),
}

impl Server {
    pub fn new(addr: &str, conn_handler: fn(&mut TcpStream)) -> Self {
        Self {
            addr: addr.to_string(),
            conn_handler,
        }
    }

    pub fn start(self) {
        let server = TcpListener::bind(&self.addr).expect("Cannot bind socket");

        println!("Listening on: http://{}", self.addr);

        for stream in server.incoming() {
            thread::spawn(move || match stream {
                Ok(mut conn) => (self.conn_handler)(&mut conn),
                Err(_) => {
                    println!("Connection failed")
                }
            });
        }
    }
}
