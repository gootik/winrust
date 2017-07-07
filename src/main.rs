extern crate iron;
extern crate serde_json;
extern crate redis;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate router;

mod handler;
use handler::win::WinHandler;
use handler::status::StatusHandler;

mod middleware;
use middleware::redis::RedisMiddleware;

use iron::prelude::*;

fn main() {
    let router = router!(
        status: get "/status" => StatusHandler::new(),
        win: get "/win" => WinHandler::new(),
    );

    let mut chain = Chain::new(router);
    chain.link_before(RedisMiddleware::new());

    Iron::new(chain)
        .http("localhost:7790")
        .unwrap();
}
