use crate::http::{request, Request};
use std::convert::{TryFrom, TryInto};
use std::{io::Read, net::TcpListener};

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
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed with error {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read Connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
