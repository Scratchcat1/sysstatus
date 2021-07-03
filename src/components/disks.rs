use crate::config::GeneralInfoConfig;
use crate::util::select_colour_number;
use bytesize::ByteSize;
use colored::*;
use std::str;
use sysinfo::ProcessorExt;
use sysinfo::{Disk, DiskExt};
use sysinfo::{System, SystemExt};

pub fn format_disk(disk: &Disk, cfg: &GeneralInfoConfig) -> String {
    let used = ByteSize::b(disk.total_space() - disk.available_space());
    let bar_width = 40;
    let used_bar_width =
        ((used.as_u64() as f32 / disk.total_space() as f32) * bar_width as f32) as usize;
    // let utilised = format!(
    //     "{} ({:.4}%) used out of {}",
    //     used,
    //     (used_ratio * 100.0).to_string(),
    //     ByteSize::b(disk.total_space()),
    // );

    let file_system = match str::from_utf8(disk.file_system()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    format!(
        "{:<8} {:#?} {:>5} {:>10} {:>10} \n[{}{}]",
        disk.mount_point()
            .to_str()
            .expect("Failed to get disk name"),
        disk.type_(),
        file_system,
        used,
        ByteSize::b(disk.total_space()),
        "=".repeat(used_bar_width),
        "=".repeat(bar_width - used_bar_width)
    )
}

pub fn disks(sys: &mut System, cfg: &GeneralInfoConfig) -> String {
    sys.refresh_disks_list();
    println!("HI {}", sys.disks().len());
    for disk in sys.disks() {
        println!("{}", format_disk(&disk, cfg));
    }
    "".to_string()
}
