
use super::server::Handler;
use super::http::{StatusCode, Request, Response, Method};
use std::fs;

pub struct WebsiteHandler {
	public_path: String
}

impl WebsiteHandler {
	pub fn new(public_path: String) -> Self {
		Self { public_path }
	}

	fn read_file(&self, file_name: &str) -> Option<String> {
		let file_path = format!("{}/{}", self.public_path, file_name);

		match fs::canonicalize(&file_path) {
			Ok(file_path) => {
				if file_path.starts_with(&self.public_path) {
					fs::read_to_string(file_path).ok()
				} else {
					println!("Directory Traversal Attack Attemped at {}", file_name);
					None
				}
			},
			Err(_) => None
		}
	}
}

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		// Response::new(StatusCode::Ok, Some("<h1>Helloooooooo</h1>".to_string()))
		match request.method() {
			Method::GET => {
				match request.path() {
					"/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
					"/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
					path => match self.read_file(path) {
						Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
						None => Response::new(StatusCode::BadRequest, None)
					}
				}
			}
			_ => Response::new(StatusCode::NotFound, None)
		}

	}
}