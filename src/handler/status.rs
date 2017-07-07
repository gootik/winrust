extern crate serde_json;
extern crate iron;

use iron::prelude::*;
use iron::Handler;

#[derive(Serialize)]
pub struct Status {
    health: String,
}

pub struct StatusHandler {}

impl StatusHandler {
    pub fn new() -> StatusHandler {
        StatusHandler {}
    }
}

impl Handler for StatusHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let result = Status {
            health: String::from("OK"),
        };

        let result = serde_json::to_string(&result)
            .unwrap();

        Ok(Response::with((iron::status::Ok, result)))
    }
}