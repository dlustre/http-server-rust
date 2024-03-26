use std::{
    fmt::format,
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
};

use http::Response;

mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_request(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: &mut TcpStream) {
    let mut buf_reader = io::BufReader::new(&mut stream);
    let mut request_buffer = vec![];

    if let Ok(_) = buf_reader.read_until(b'\n', &mut request_buffer) {
        let request = http::parse_http(&request_buffer);

        let response = match request.method {
            http::Method::GET => match request.path.as_str() {
                "/" => Response {
                    status: http::Status::Ok,
                    version: request.version,
                    body: None,
                },
                path => {
                    if path.starts_with("/echo/") {
                        Response {
                            status: http::Status::Ok,
                            version: request.version,
                            body: Some(path.strip_prefix("/echo/").unwrap_or_default().to_string()),
                        }
                    } else {
                        Response {
                            status: http::Status::NotFound,
                            version: request.version,
                            body: None,
                        }
                    }
                }
            },
            http::Method::POST => todo!(),
        };

        println!("{}", response.to_string());

        let response_str = format!("{}", response);
        stream.write_all(response_str.as_bytes()).unwrap();
    } else {
        println!("Error reading from stream");
    }
}
