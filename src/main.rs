#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nickel;
extern crate serde_json;

use nickel::{ MiddlewareResult, Nickel, Request, Response};

mod services;
mod dtos;
pub mod logger;
mod router;


fn main() {
    logger::init().expect("Error initializing logger");
    let mut server = Nickel::new();
    server.utilize(router::enable_cors);
    server.utilize(add_json_header);
    server.utilize(router::get_router());

    server.listen("127.0.0.1:6767").unwrap();
    info!("Froovie successfully started in debud mode");

}

fn add_json_header<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.headers_mut().set_raw("Content-Type", vec![b"Application/json,".to_vec()]);
    res.next_middleware()
}