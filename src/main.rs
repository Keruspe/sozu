#[macro_use] extern crate nom;
extern crate mio;

mod network;

fn main() {
  let (tx, jg) = network::start_listener("hello");
  jg.join();
}
