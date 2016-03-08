use std::path:: { Path };
use std::env:: { current_dir };
use clap:: { App, Arg };
use nickel::Nickel;
use configuration:: { Configuration };
use routing:: { router };

pub struct Application<'a> {
    cli: App<'a, 'a>
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        let app = App::new("Nickel application server")
            .version("1.0")
            .author("Noritaka Horio <holy.shared.design@gmail.com>")
            .arg(Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("configuration file")
                    .takes_value(true));

        Application {
            cli: app
        }
    }
    pub fn run(self) {
        let matches = self.cli.get_matches();
        let mut config = Configuration::default();

        if let Some(config_file) = matches.value_of("config") {
            let cwd = current_dir().unwrap();
            let config_path = Path::new(&cwd).join(config_file);
            config = Configuration::from(config_path.as_path());
        }

        let mut server = Nickel::new();
        server.utilize(router());
        server.listen(config.server_address());
    }
}
