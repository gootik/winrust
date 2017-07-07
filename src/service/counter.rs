extern crate redis;

use redis::Commands;

pub struct CounterService {
    connection: ()
}

impl CounterService {
    fn new() -> CounterService {

        let client = redis::Client::open("redis://127.0.0.1/")
            .unwrap();
        CounterService {
            connection: client.get_connection().unwrap(),
        }
    }

    fn count(&self, key: String, value: String) {
        self.connection.pfcount(key, value);
    }

}

