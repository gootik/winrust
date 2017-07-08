use std::ops::Deref;

use iron;
use iron::prelude::*;
use iron::Handler;

use params::{Params, Value, Map};

use middleware::redis::RedisReqExt;
use service::counter;

static COUNT_FIELDS: &'static [&str] = &[
    "event_id",
    "campaign_id",
];

static PREFIX: &'static str = "winrust:v1:";

pub struct WinHandler {}
impl WinHandler {
    pub fn new() -> WinHandler {
        WinHandler{}
    }
}

impl Handler for WinHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let connection = req.redis();

        let map = req.get_ref::<Params>().unwrap();

        for key in COUNT_FIELDS {
            match map.get(key.to_owned()) {
                Some(&Value::String(ref value)) => {
                    let mut count_key: String = PREFIX.to_owned();
                    count_key.push_str(key);

                    counter::count(connection.deref(), count_key, value.to_owned());
                }

                _ => {
                    println!("Did not have {}", key);
                }
            }
        }

        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}
