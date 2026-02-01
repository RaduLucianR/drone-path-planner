mod utils;

use std::error::Error;
use std::fs;

use utils::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let default_config_path = "./config/config.toml";
    let config_str = fs::read_to_string(default_config_path)?;
    let config: Config = toml::from_str(&config_str).expect("Failed to parse TOML");
    println!("{:?}", config);

    Ok(())
}
