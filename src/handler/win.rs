use iron;
use iron::prelude::*;
use iron::Handler;

use middleware::redis::RedisReqExt;
use service::counter::CounterService;

use std::ops::Deref;

use params::Params;

pub struct WinHandler {}
impl WinHandler {
    pub fn new() -> WinHandler {
        WinHandler{}
    }
}

impl Handler for WinHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let connection = req.redis();

        CounterService::count(connection.deref(), String::from("a"), String::from("b"));

        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}
