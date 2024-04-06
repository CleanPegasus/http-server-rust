use crate::http::{Request, Response, StatusCode};
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Write};
use std::net::TcpListener;


pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("The server is listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok, 
                                        Some("<h1> Hello World </h1>".to_string())
                                    )
                                    
                                }
                                Err(e) => {
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read Connection: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
