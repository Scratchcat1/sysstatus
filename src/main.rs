use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use sysinfo::{System, SystemExt};
use thiserror::Error;

mod components;
mod config;
mod util;

use config::SysStatusConfig;

#[derive(StructOpt)]
struct Cli {
    /// Path to configuration file. Default is $HOME/.config/sysstatus/config.json
    #[structopt(short, long)]
    config_file_path: Option<PathBuf>,

    /// Create a default config file and then exit
    #[structopt(long)]
    default_config: bool,
}

fn load_config(path: &Path) -> Result<SysStatusConfig, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::ConfigNotFound);
    }
    if path.is_dir() {
        return Err(ConfigError::ConfigIsDir);
    }
    let reader = File::open(path)?;
    let pimon_config = serde_json::from_reader(&reader)?;
    Ok(pimon_config)
}

fn get_config_path(override_path: &Option<PathBuf>) -> Result<PathBuf, ConfigError> {
    let path = match override_path {
        Some(override_path) => override_path.clone(),
        None => {
            let config_base = env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME")? + "/.config");
            Path::new(&config_base).join(Path::new("sysstatus/config.json"))
        }
    };
    Ok(path)
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(
        "Configuration file could not be found.
    Create one or copy the default found at [repo]/resources/config.json.
    By default config file location is expected to be in \"$HOME/.config/sysstatus/config.json\""
    )]
    ConfigNotFound,

    #[error("Configuration location is a directory not a file.")]
    ConfigIsDir,

    #[error(transparent)]
    ConfigHomeError(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigParseError(#[from] serde_json::Error),
}

fn main() {
    // Parse command line arguments
    let args = Cli::from_args();
    let config_path = get_config_path(&args.config_file_path).unwrap();

    match args.default_config {
        true => {
            println!(
                "Creating default config at \"{}\"",
                config_path.to_string_lossy()
            );
            fs::create_dir_all(config_path.parent().expect("No parent directory of file")).unwrap();
            let mut file = File::create(&config_path).unwrap();
            writeln!(&mut file, "{}", include_str!("../resources/config.json")).unwrap();
        }
        false => {
            let cfg = load_config(&config_path);
            match cfg {
                Ok(cfg) => {
                    let mut sys = System::new();
                    let indent = "    ";
                    println!(
                        "{}",
                        components::general_info::general_info(&mut sys, &cfg.general_info)
                    );
                    components::disks::print_disks(&mut sys, &cfg.storage, indent);
                    println!(
                        "Temperatures:\n{}\n",
                        components::temperature::temperature(&mut sys, &cfg.temperature)
                    );
                    // "Services:\n{}\n",
                    components::services::print_services(&cfg.services, indent);
                    components::last_login::print_last_login(&cfg.last_login, indent);
                }
                Err(e) => eprintln!("Config error: {}", e),
            }
        }
    }
}
