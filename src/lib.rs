extern crate toml;
#[macro_use] extern crate nickel;

use nickel::Nickel;
use self::routing::*;

mod routing;
mod middleware;
mod configuration;

pub fn server() {
    let mut server = Nickel::new();

    server.utilize(router());
    server.listen("127.0.0.1:6767");
}
