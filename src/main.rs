extern crate toml;
extern crate rustc_serialize;
extern crate clap;
#[macro_use] extern crate nickel;

use self::server:: { Application };

mod server;
mod routing;
mod middleware;
mod configuration;

fn main() {
    Application::new().run();
}
