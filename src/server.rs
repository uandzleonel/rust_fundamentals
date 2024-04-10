use std::net::TcpListener;

#[derive(Debug)]
pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self {
      address
    }
  }

  pub fn run(&self) {
    let _listener = TcpListener::bind(&self.address).unwrap();
    println!("Listening on {:?}", self.address);
    loop {};
  }
}
