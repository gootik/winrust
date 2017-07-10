use std::sync::Arc;
use std::default::Default;
use std::ops::Deref;

use r2d2;
use r2d2_redis::RedisConnectionManager;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

use setting;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type RedisConnection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct RedisMiddleware {
    pool: Arc<RedisPool>,
}

impl RedisMiddleware {
    pub fn new() -> RedisMiddleware {
        let redis_endpoint = format!(
            "redis://{password}{host}:{port}/{database}",
            password = match setting::GLOBAL_SETTINGS.redis.password.is_empty() {
                true => {
                    "".to_string()
                }
                _ => {
                    format!("{}@", setting::GLOBAL_SETTINGS.redis.password)
                }
            },
            host = setting::GLOBAL_SETTINGS.redis.host,
            port = setting::GLOBAL_SETTINGS.redis.port.to_string(),
            database = setting::GLOBAL_SETTINGS.redis.db,
        );

        let config = Default::default();
        let manager = RedisConnectionManager::new(redis_endpoint.deref()).unwrap();
        let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

        println!("Connecting to redis @ {:?}", redis_endpoint);

        RedisMiddleware {
            pool: pool
        }
    }
}


pub struct Value(Arc<RedisPool>);

impl typemap::Key for RedisMiddleware { type Value = Value; }

impl BeforeMiddleware for RedisMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<RedisMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait RedisReqExt {
    fn redis(&self) -> RedisConnection;
}

impl<'a, 'b> RedisReqExt for Request<'a, 'b> {
    fn redis(&self) -> RedisConnection {
        let &Value(ref pool) = self.extensions.get::<RedisMiddleware>().unwrap();

        return pool.get().unwrap()
    }
}
