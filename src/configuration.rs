use toml:: { Parser, Decoder, Value };
use rustc_serialize:: { Decodable };
use std::default:: { Default };
use std::path:: { Path };
use std::fs::File;
use std::io:: { Read, Result };
use std::borrow:: { Borrow };
use std::convert:: { From };

#[derive(RustcDecodable)]
pub struct Configuration {
    server: Server
}

#[derive(RustcDecodable)]
pub struct Server {
    host: String,
    port: i64
}

impl<'a> From<&'a Path> for Configuration {
    fn from(path: &'a Path) -> Self {
        from_toml_file(path).unwrap()
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            host: "127.0.0.1".to_string(),
            port: 6767
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            server: Server::default()
        }
    }
}

pub fn from_toml_file(path: &Path) -> Result<Configuration> {
    let content = read_file(path).unwrap();
    let toml = &content[..];
    let value = Parser::new(toml).parse().unwrap();
    let mut decoder = Decoder::new(Value::Table(value));
    let configuration = Configuration::decode(&mut decoder).unwrap();
    Ok(configuration)
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    let _ = try!(file.read_to_string(&mut buffer));
    Ok(buffer.clone())
}
