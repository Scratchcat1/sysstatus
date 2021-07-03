use serde_json;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use sysinfo::{System, SystemExt};

mod components;
mod config;
mod util;

use config::SysStatusConfig;

#[derive(StructOpt)]
struct Cli {
    /// Path to configuration file
    #[structopt(short, long, default_value("resources/config.json"))]
    config_file_path: PathBuf,
}

pub fn load_config_from_json(path: &PathBuf) -> Result<SysStatusConfig, Box<dyn Error>> {
    let f = File::open(path).expect("Configuration file not found");
    let pimon_config: SysStatusConfig = serde_json::from_reader(&f)?;
    Ok(SysStatusConfig::from(pimon_config))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Cli::from_args();

    let cfg = load_config_from_json(&args.config_file_path)?;
    let mut sys = System::new();

    println!(
        "{}\n{}",
        components::general_info::general_info(&mut sys, &cfg.general_info),
        components::disks::disks(&mut sys, &cfg.general_info)
    );
    Ok(())
}
