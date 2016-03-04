extern crate toml;
extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use nickel::Nickel;
use self::routing::*;
use self::configuration:: { Configuration };

mod routing;
mod middleware;
mod configuration;

pub fn server() {
    let mut server = Nickel::new();
    let config = Configuration::default();

    server.utilize(router());
    server.listen(config.server_address());
}
