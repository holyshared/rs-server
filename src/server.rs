use std::path:: { Path };
use clap:: { App, Arg, ArgMatches };
use nickel::Nickel;
use nickel:: { StaticFilesHandler };
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
        Configuration::from(Path::new(config_file))
    }
    fn start(&self, config: &Configuration) {
        let mut server = Nickel::new();
        server.utilize(router());
        server.utilize(StaticFilesHandler::new(Path::new(".")));
        server.listen(config.server());
    }
}
