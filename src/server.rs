extern crate simple_server;
use simple_server::Server;

pub fn listen(host: String, ip: String) {
  let server = Server::new(|req, mut response| {
    println!("Received a request with some content");
    let data = String::from_utf8_lossy(req.body()).into_owned();
    println!("{}", data);
    Ok(response.body("Hello, world!".as_bytes().to_vec())?)
  });

  server.listen(&host, &ip)
}
