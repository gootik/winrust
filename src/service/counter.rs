extern crate redis;

use redis::Commands;

pub struct CounterService {
}

impl CounterService {
    fn count(&self, key: String, value: String) {
        self.connection.pfcount(key, value);
    }

}
