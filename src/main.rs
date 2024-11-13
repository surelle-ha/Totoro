mod visual;
use colored::Colorize;
use serde::Deserialize;
use config::{Config, ConfigError, File};
use visual::core;

#[derive(Debug, Deserialize)]
struct AppConfig {
    no_adv_banner: bool,
    no_bsc_banner: bool,
}

impl AppConfig {
    fn from_file() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config").required(true))
            .build()?;
        
        settings.try_deserialize::<AppConfig>()
    }
}

fn main() {
    let config = match AppConfig::from_file() {
        Ok(config) => {
            println!("Loaded config: {:?}", config);
            config
        }
        Err(e) => {
            eprintln!("Failed to load config: {:?}", e);
            return;
        }
    };

    if !config.no_adv_banner {
        core::banner_advance();
    }
    if !config.no_bsc_banner {
        core::banner_basic();
    }

    println!(
        "{placeholder}: {command} [{option}] <{attribute}>",
        placeholder = "Usage".blue(),
        command = "totoro".red(),
        option = "option".red(),
        attribute = "attribute".red()
    );
}
