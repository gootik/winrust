use redis;

use iron;
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
        let connection = req.redis();
        redis::cmd("SEsT")
            .arg("winrust")
            .arg("b")
            .query::<String>(connection.deref())
            .expect("Could not insert into redis.");

        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}
