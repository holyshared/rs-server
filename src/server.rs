use std::path:: { Path };
use std::env:: { current_dir };
use clap:: { App, Arg, ArgMatches };
use nickel::Nickel;
use configuration:: { Configuration };
use routing:: { router };

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Application
    }
    pub fn run(&self) {
        let matches = self.parse_args();
        let config = match matches.value_of("config") {
            Some(config_file) => self.load_configuration_from_file(config_file),
            None => Configuration::default()
        };
        self.start(&config);
    }
    fn parse_args<'a>(&self) -> ArgMatches<'a> {
        App::new("Nickel application server")
            .version("1.0")
            .author("Noritaka Horio <holy.shared.design@gmail.com>")
            .arg(Arg::from_usage("-c --config=[CONFIG] 'configuration file'"))
            .get_matches()
    }
    fn load_configuration_from_file(&self, config_file: &str) -> Configuration {
        let cwd = current_dir().unwrap();
        let config_path = Path::new(&cwd).join(config_file);
        Configuration::from(config_path.as_path())
    }
    fn start(&self, config: &Configuration) {
        let mut server = Nickel::new();
        server.utilize(router());
        server.listen(config.server());
    }
}
