use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
  fn handle_resquest(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, err: &ParseError) -> Response {
    println!("Failed to parse a requst: {}", err);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn run(self, mut handler: impl Handler) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));

              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  // Response::new(StatusCode::Ok, Some("<h1>It works!!</h1>".to_string()))
                  // write!(stream, "{}", reponse);
                  handler.handle_resquest(&request)
                }
                Err(err) => {
                  // println!("Failed to parse a requst: {}", err);
                  // Response::new(
                  //   StatusCode::BadRequest,
                  //   Some("<h1>It works!!</h1>".to_string()),
                  // )
                  handler.handle_bad_request(&err)
                }
              };

              if let Err(err) = response.send(&mut stream) {
                println!("fail to send response: {}", err);
              }

              let res: &Result<Request, _> = &buffer[..].try_into();
            }
            Err(err) => println!("Failed to read from connection: {}", err),
          }
        }
        // _ => {} // ignore error
        Err(err) => println!("Failed to establish a connection: {}", err),
      }

      // let res = listener.accept();

      // if res.is_err() {
      //   continue;
      // }

      // let (stream, addr) = res.unwrap();
    }
  }
}
