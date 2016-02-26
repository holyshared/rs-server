use toml:: { Parser };
use std::default:: { Default };
use std::path:: { Path };
use std::fs::File;
use std::io:: { Read, Result };
//use std::io;
use std::borrow:: { Borrow };

pub struct Configuration {
    host: String,
    port: String
}

impl Configuration {
    pub fn fromFile(path: &Path) -> Self {
        from_toml_file(path).unwrap()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            host: "127.0.0.1".to_string(),
            port: "6767".to_string()
        }
    }
}

pub fn from_toml_file(path: &Path) -> Option<Configuration> {
    let content = read_file(path).unwrap();
    let mut parser = Parser::new(content.borrow());

    match parser.parse() {
        Some(table) => {
            let host = table.get(&"host".to_string()).unwrap();
            let port = table.get(&"port".to_string()).unwrap();

            Some(
                Configuration {
                    host: host.to_string(),
                    port: port.to_string()
                }
            )
        },
        None => None
    }
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    let _ = try!(file.read_to_string(&mut buffer));
    Ok(buffer.clone())
}
