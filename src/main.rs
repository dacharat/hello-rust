use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..];

    // dbg!(&string);
    // dbg!(string_slice);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
