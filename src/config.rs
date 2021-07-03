use colored::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SysStatusConfig {
    pub general_info: GeneralInfoConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralInfoConfig {
    pub load_avg: ConditionalColour<f64>,
    pub free_memory: ConditionalColour<f32>,
    pub cpu_frequency: ConditionalColour<u64>,
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
#[derive(Deserialize)]
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