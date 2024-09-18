use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Config {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            db_host: env::var("DB_HOST").expect("DB_HOST must be set"),
            db_port: env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse()
                .expect("DB_PORT must be a number"),
            db_username: env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
            db_name: env::var("DB_NAME").expect("DB_NAME must be set"),
        }
    }
}
