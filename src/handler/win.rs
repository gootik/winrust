extern crate iron;
extern crate redis;

use iron::prelude::*;
use iron::Handler;

use middleware::redis::RedisReqExt;

use std::ops::Deref;

pub struct WinHandler {}
impl WinHandler {
    pub fn new() -> WinHandler {
        WinHandler{}
    }
}

impl Handler for WinHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let connection = req.get_redis().deref();

        redis::cmd("INFO").query(connection);

        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}