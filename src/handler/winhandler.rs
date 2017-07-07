extern crate iron;

use iron::prelude::*;
use iron::Handler;

pub struct WinHandler {}
impl WinHandler {
    pub fn new() -> WinHandler {
        WinHandler{}
    }
}

impl Handler for WinHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}