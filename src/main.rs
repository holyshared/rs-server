extern crate toml;
extern crate rustc_serialize;
extern crate clap;
#[macro_use] extern crate nickel;

use nickel::Nickel;
use self::routing::*;
use self::configuration:: { Configuration };
use std::path:: { Path };
use std::env:: { current_dir };
use clap:: { App, Arg };

mod routing;
mod middleware;
mod configuration;

fn main() {
    let matches = App::new("Nickel application server")
        .version("1.0")
        .author("Noritaka Horio <holy.shared.design@gmail.com>")
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("configuration file")
                .takes_value(true)
        ).get_matches();

    let config_file = matches.value_of("config").unwrap();
    let mut config = Configuration::default();

    if !config_file.is_empty() {
        let cwd = current_dir().unwrap();
        let config_path = Path::new(&cwd).join(config_file);
        config = Configuration::from(config_path.as_path());
    }

    let mut server = Nickel::new();
    server.utilize(router());
    server.listen(config.server_address());
}
