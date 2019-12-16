
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "{\n\tmessage: \"Hello! Sincerely, Jimbob's Rusty Nickel\"\n}"
        }
    });

    server.listen("127.0.0.1:6767").unwrap();
}