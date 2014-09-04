extern crate rust_demo;
extern crate iron;

use std::io::net::ip::Ipv4Addr;
use rust_demo::app;
use iron::Iron;

fn main() {
    let iron = Iron::new(app::mount());
    iron.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
