use crate::config::ConditionalColour;
use crate::util::select_colour_number;
use bytesize::ByteSize;
use colored::*;
use std::str;
use sysinfo::{Disk, DiskExt};
use sysinfo::{System, SystemExt};

pub fn single_disk(disk: &Disk, cfg: &ConditionalColour<f32>) -> String {
    let used = ByteSize::b(disk.total_space() - disk.available_space());
    let bar_width = 60;
    let used_ratio = used.as_u64() as f32 / disk.total_space() as f32;
    let used_bar_width = (used_ratio * bar_width as f32) as usize;

    let file_system = match str::from_utf8(disk.file_system()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    format!(
        "   {:<8} {:#?} {:>5} {:>10} {:>10} \n[{}{}]",
        disk.mount_point()
            .to_str()
            .expect("Failed to get disk name"),
        disk.type_(),
        file_system,
        used,
        ByteSize::b(disk.total_space()),
        "=".repeat(used_bar_width)
            .color(select_colour_number(used_ratio, cfg)),
        "=".repeat(bar_width - used_bar_width)
    )
}

pub fn disks(sys: &mut System, cfg: &ConditionalColour<f32>) -> String {
    sys.refresh_disks_list();

    sys.disks()
        .iter()
        .map(|disk| single_disk(&disk, cfg))
        .collect::<Vec<String>>()
        .join("\n")
}
