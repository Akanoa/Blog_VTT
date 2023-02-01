use crate::errors::MyError;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub working_directory: String,
    pub session_key: String,
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        // On récupère le chemin courant
        let current_dir = env::current_dir()
            .expect("Unable to found current dir")
            .to_str()
            // le chemin peut ne pas être de l'UTF-8
            .expect("Bad UTF-8 string")
            .to_string();

        Config {
            working_directory: current_dir.to_string(),
            database_path: format!("{}/database.db", current_dir),
            session_key:
                "je suis une clef très secrète et très longue pour être suffisamment sécurisée"
                    .to_string(),
        }
    }
}

pub fn get_configuration(configuration_file: Option<&String>) -> Result<Config, MyError> {
    match configuration_file {
        None => Ok(Figment::from(Serialized::defaults(Config::default()))
            .merge(Env::prefixed("BLOG_"))
            .extract()?),
        Some(configuration_file) => Ok(Figment::new()
            .merge(Toml::file(configuration_file))
            .merge(Env::prefixed("BLOG_"))
            .extract()?),
    }
}
