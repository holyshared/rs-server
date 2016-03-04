extern crate toml;
extern crate rustc_serialize;
extern crate argparse;
#[macro_use] extern crate nickel;

use nickel::Nickel;
use self::routing::*;
use self::configuration:: { Configuration };
use argparse::{ ArgumentParser, Store };
use std::path:: { Path };
use std::env:: { current_dir };

mod routing;
mod middleware;
mod configuration;

pub fn server() {
    let mut config = Configuration::default();
    let mut config_file = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Nickel application server");
        ap.refer(&mut config_file).add_option(&["-c", "--config"], Store, "configuration file");
        ap.parse_args_or_exit();
    }

    if !config_file.is_empty() {
        let cwd = current_dir().unwrap();
        let config_path = Path::new(&cwd).join(config_file);
        config = Configuration::from(config_path.as_path());
    }

    let mut server = Nickel::new();
    server.utilize(router());
    server.listen(config.server_address());
}
