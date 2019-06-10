#[macro_use]
extern crate nickel;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use nickel::{HttpRouter, Nickel, Request, Response, MiddlewareResult, JsonBody, MediaType};

mod movies_client;
mod dtos;
mod mappers;
mod router;


fn main() {
    let mut server = Nickel::new();
    server.utilize(add_json_header);

    server.utilize(router::get_router());

    server.listen("127.0.0.1:6767").unwrap();
}

fn add_json_header<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.headers_mut().set_raw("Content-Type", vec![b"Application/json,".to_vec()]);
    res.next_middleware()
}