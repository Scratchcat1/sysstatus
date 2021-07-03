use crate::config::ConditionalColour;
use crate::util::select_colour_number;
use colored::*;
use sysinfo::{Component, ComponentExt};
use sysinfo::{System, SystemExt};

pub fn single_sensor(
    cmpt: &Component,
    cfg: &ConditionalColour<f32>,
    align_length: usize,
) -> String {
    let coloured_temp = cmpt
        .temperature()
        .to_string()
        .color(select_colour_number(cmpt.temperature(), cfg));
    format!(
        "   {:.<algn$}: {}°C",
        cmpt.label(),
        coloured_temp,
        algn = align_length
    )
}

pub fn temperature(sys: &mut System, cfg: &ConditionalColour<f32>) -> String {
    sys.refresh_components_list();
    let align_length = sys
        .components()
        .iter()
        .map(|component| component.label().len())
        .max()
        .unwrap_or(0)
        + 3;

    sys.components()
        .iter()
        .map(|component| single_sensor(&component, cfg, align_length))
        .collect::<Vec<String>>()
        .join("\n")
}
