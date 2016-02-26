use toml:: { Parser };
use std::default:: { Default };
use std::path:: { Path };
use std::fs::File;
use std::io:: { Read };
use std::str:: { from_utf8 };
//let mut f = try!(File::create("foo.txt"));
//try!(f.write_all(b"Hello, world!"));


pub struct Configuration {
    host: String,
    port: String
}
/*
impl Configuration {
    pub fn fromFile(path: Path) -> Self {



        let mut f = try!(File::open("foo.txt"));
        let mut s = String::new();
        f.read_to_string(s);

        match from_utf8(s.as_bytes()) {
            Ok(v) => {
                let mut parser = Parser::new(v);
                match parser.parse() {
                    Some(table) => {
                        let host = table.get(&"host").unwrap();
                        let port = table.get(&"port").unwrap();

                        Configuration {
                            host: host.to_string(),
                            port: port.to_string()
                        }
                    },
                    None => println!("parse errors: {:?}", parser.errors)
                };
            },
            Err(err) => println!("{}", err)
        };




//        Configuration { host: "127.0.0.1".to_string(), port: "6767".to_string() }
    }
}

*/

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            host: "127.0.0.1".to_string(),
            port: "6767".to_string()
        }
    }
}


/*
pub fn loadconfig() {
    let mut parser = Parser::new(toml);
    match parser.parse() {
        Some(value) => println!("found toml: {:?}", value),
        None => {
            println!("parse errors: {:?}", parser.errors);
        }
    }
}
*/
