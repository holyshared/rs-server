use toml:: { Parser, Decoder, Value };
use rustc_serialize:: { Decodable };
use std::default:: { Default };
use std::path:: { Path };
use std::fs::File;
use std::io:: { Read, Result };
use std::convert:: { From };

#[derive(RustcDecodable)]
pub struct Configuration {
    server: Server
}

#[derive(RustcDecodable)]
pub struct Server {
    host: String,
    port: u16
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

#[cfg(test)]
mod tests {
    use std::path:: { Path };
    use std::env:: { current_dir };
    use configuration:: { read_file, Server, Configuration };

    #[test]
    fn test_read_file() {
        let cwd = current_dir().unwrap();
        let path = Path::new(&cwd).join("tests/fixtures/server.toml");
        let content = read_file(path.as_path());
        let expected = r#"[server]
host = "127.0.0.1"
port = 3000
"#;

        assert_eq!(content.unwrap(), expected.to_string());
    }

    #[test]
    fn test_server_default() {
        let server = Server::default();

        assert_eq!(server.host, "127.0.0.1".to_string());
        assert_eq!(server.port, 6767);
    }

    #[test]
    fn test_config_default() {
        let config = Configuration::default();

        assert_eq!(config.server.host, "127.0.0.1".to_string());
        assert_eq!(config.server.port, 6767);
    }

}
