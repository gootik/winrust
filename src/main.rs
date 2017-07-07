extern crate iron;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate router;

mod handler;
use handler::winhandler::WinHandler;
use handler::statushandler::StatusHandler;

use iron::prelude::*;

fn main() {
    let router = router!(
        status: get "/status" => StatusHandler::new(),
        win: get "/win" => WinHandler::new(),
    );

    Iron::new(router)
        .http("localhost:7790")
        .unwrap();
}
