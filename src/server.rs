extern crate simple_server;
use crate::storage::{read, write};
use simple_server::{Method, Server, StatusCode};

pub fn listen(host: String, ip: String) {
  let server = Server::new(|req, mut response| {
    println!("Request received. {} {}", req.method(), req.uri());
    match req.method() {
      &Method::GET => {
        println!("Tu vuoi che ti fetcho cose");
        let data = read().expect("Cannot read logs");
        response.header("content-type", "application/json".as_bytes());
        Ok(response.body(data.as_bytes().to_vec())?)
      }
      &Method::POST => {
        let data = String::from_utf8_lossy(req.body()).into_owned();
        println!("Sccrivo sta roba {}", data);
        write(data).expect("Cannot write log");
        Ok(response.body("Ok".as_bytes().to_vec())?)
      }
      _ => {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("Ok".as_bytes().to_vec())?)
      }
    }
  });

  server.listen(&host, &ip)
}
