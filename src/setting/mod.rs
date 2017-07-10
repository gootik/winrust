use config::{Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: i32,
    pub password: String,
    pub db: u8
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: i32
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub redis: Redis,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Self {
        let mut c = Config::new();

        // Load defaults
        c.merge(File::with_name("config/default")).unwrap();

        // Load Env overwrites
        c.merge(Environment::with_prefix("winrust")).unwrap();
        c.deserialize().unwrap()
    }
}

lazy_static! {
    pub static ref GLOBAL_SETTINGS: Settings = Settings::new();
}