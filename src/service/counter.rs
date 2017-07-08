use redis;

static PREFIX: &'static str = "winrust:v1:";

pub fn count(con: &redis::Connection, key: String, value: String) {
    redis::cmd("PFADD")
        .arg::<String>(PREFIX.to_owned() + &key)
        .arg::<String>(value)
        .query(con)
        .expect("Could not count.")
}
