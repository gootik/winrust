extern crate iron;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use self::r2d2_redis::RedisConnectionManager;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

use redis::Commands;

use std::default::Default;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type RedisConnection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct RedisMiddleware {
    pool: RedisPool,
}

impl RedisMiddleware {
    pub fn new() -> RedisMiddleware {
        let config = Default::default();
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = r2d2::Pool::new(config, manager).unwrap();


        RedisMiddleware {
            pool: pool
        }
    }
}


pub struct Value(RedisPool);
impl typemap::Key for RedisMiddleware { type Value = Value; }

impl BeforeMiddleware for RedisMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<RedisMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait RedisReqExt {
    fn get_redis(&self) -> RedisConnection;
}

impl<'a, 'b> RedisReqExt for Request<'a, 'b> {
    fn get_redis(&self) -> RedisConnection {
        let &Value(ref pool) = self.extensions.get::<RedisMiddleware>().unwrap();

        return pool.get().unwrap()
    }
}