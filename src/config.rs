use colored::Color;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SysStatusConfig {
    /// Configuration for the general section.
    pub general_info: GeneralInfoConfig,
    /// Configuration for the storage section.
    pub storage: StorageConfig,
    /// Configuration for the temperature section.
    /// Conditionally colours the temperatures presented.
    pub temperature: ConditionalColour<f32>,
    /// Configuration for the services section.
    /// Map from the service name (without a suffix of .service) to the configuration for the service.
    pub services: HashMap<String, ServiceConfig>,
    /// Configuration for the last login section.
    pub last_login: LastLoginConfig,
}

/// Configuration for the general section.
#[derive(Debug, Deserialize)]
pub struct GeneralInfoConfig {
    /// Conditionally colour the load averages.
    /// Comparison is made to the load average divided by the core count e.g. a load of 3.0 on 4 cores will generate the comparison value 0.75.
    /// This allows configurations to be machine independent while also allowing sensible colouring.
    pub load_avg: ConditionalColour<f64>,
    /// Conditionally colour the used memory value.
    /// Comparison value will be the used memory divided by the total memory for a scale from 0-1 e.g. 4GB used on a 16GB machine generates a comparison value of 0.25.
    /// This allows configurations to be machine independent while also allowing sensible colouring.
    pub free_memory: ConditionalColour<f32>,
    /// Conditionally colour the current CPU frequency.
    /// Comparison value is the CPU frequency in MHz.
    pub cpu_frequency: ConditionalColour<u64>,
}

/// Configuration for the storage section.
#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    /// Conditionally colour the usage bar of each disk.
    /// Comparison value a value from 0-1 calculated from used divided by total storage space.
    pub usage_colouring: ConditionalColour<f32>,
    /// Vector of path prefixes to exclude. Any disk mounts matching this path will be ignored.
    /// Use cases include docker volumes which appear in `/var/lib/docker` and often are the same as the disk they are located on.
    ///
    /// Currently uses the `.starts_with(prefix)` function to match so all directories should be terminated with a path separator.
    /// Otherwise `/example` will exclude both `/example/` and `/example2/` while `/example/` will only match the first.
    pub exclude_prefixes: Vec<String>,
}

/// Configuration for a systemd service.
#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    /// Conditionally colour the memory usage of the service.
    /// Comparison value is the memory usage of the service in bytes.
    pub memory_usage: Option<ConditionalColour<u64>>,
}

/// Configuration for the last login section.
#[derive(Debug, Deserialize)]
pub struct LastLoginConfig {
    /// Optionally only include logins which occur before a certain time
    ///
    /// Accepts any value which `last --since` accepts e.g. `+5days`, `yesterday`.
    pub since: Option<String>,

    pub users: HashMap<String, UserLastLoginConfig>,
}

#[derive(Debug, Deserialize)]
pub struct UserLastLoginConfig {
    #[serde(default, with = "opt_color")]
    pub username_colour: Option<Color>,
    pub max_lines: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ConditionalColour<T: PartialOrd> {
    #[serde(with = "LocalColor")]
    pub default_colour: Color,
    pub levels: Vec<ColouringLevel<T>>,
}

#[derive(Debug, Deserialize)]
pub struct ColouringLevel<T: PartialOrd> {
    pub min: T,
    #[serde(with = "LocalColor")]
    pub colour: Color,
}

// Serde calls this the definition of the remote type. It is just a copy of the
// remote data structure. The `remote` attribute gives the path to the actual
// type we intend to derive code for.
#[derive(Debug, Deserialize)]
#[serde(remote = "Color")]
pub enum LocalColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

mod opt_color {
    use super::LocalColor;
    use colored::Color;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "LocalColor")] Color);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}
