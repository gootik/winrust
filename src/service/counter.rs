use redis;

pub struct CounterService {
}

impl CounterService {
    pub fn count(con: &redis::Connection, key: String, value: String) {
        redis::cmd("PFADD")
            .arg::<String>(key)
            .arg::<String>(value)
            .query(con)
            .expect("Could not count.")
    }
}
