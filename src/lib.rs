#[macro_use] extern crate nickel;

use nickel::Nickel;

pub fn server() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/" => |_req, _res| {
            "Hello world!"
        }
        get "/foo" => |_req, _res| {
            "Hello world!2"
        }
    });

    server.listen("127.0.0.1:6767");
}
