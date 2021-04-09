const DATABASE_URL_ENV_VAR_NAME: &str = "DATABASE_URL";
const SECRET_ENV_VAR_NAME: &str = "SECRET";

pub struct Config {
    pub database_url: String,
    pub secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            database_url: std::env::var(DATABASE_URL_ENV_VAR_NAME)?,
            secret: std::env::var(SECRET_ENV_VAR_NAME)?,
        })
    }
}
