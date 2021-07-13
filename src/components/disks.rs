use crate::config::{ConditionalColour, StorageConfig};
use crate::util::select_colour_number;
use bytesize::ByteSize;
use colored::*;
use std::cmp;
use std::iter;
use std::str;
use sysinfo::{Disk, DiskExt, DiskType};
use sysinfo::{System, SystemExt};

#[derive(Debug)]
struct Entry<'a> {
    mount_point: &'a str,
    disk_type: &'a str,
    fs_type: &'a str,
    used_percent: u64,
    used: ByteSize,
    total: ByteSize,
}

fn entry<'a>(disk: &'a Disk) -> Entry<'a> {
    let used = ByteSize::b(disk.total_space() - disk.available_space());
    let total = ByteSize::b(disk.total_space());
    let used_percent = (100 * used.as_u64()) / total.as_u64();

    let file_system = match str::from_utf8(disk.file_system()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let disk_type = match disk.type_() {
        DiskType::HDD => "HDD",
        DiskType::SSD => "SSD",
        DiskType::Unknown(_) => "N/A",
    };

    Entry {
        mount_point: disk
            .mount_point()
            .to_str()
            .expect("Failed to get disk name"),
        disk_type,
        fs_type: file_system,
        used_percent,
        used,
        total,
    }
}

fn print_entry_bar(entry: &Entry, bar_width: usize, cfg: &ConditionalColour<f32>) {
    let used_ratio = entry.used.as_u64() as f32 / entry.total.as_u64() as f32;
    let used_bar_width = (used_ratio * bar_width as f32) as usize;
    println!(
        "    [{}{}]",
        "=".repeat(used_bar_width)
            .color(select_colour_number(used_ratio, cfg)),
        "=".repeat(bar_width - used_bar_width)
    );
}

fn print_row<'a>(
    items_iter: impl IntoIterator<Item = &'a str>,
    column_sizes: impl IntoIterator<Item = &'a usize>,
) {
    println!(
        "{}",
        items_iter
            .into_iter()
            .zip(column_sizes.into_iter())
            .map(|(name, size)| format!("{: <size$}", name, size = size))
            .collect::<Vec<String>>()
            .join(&"  ")
    );
}

pub fn print_disks(sys: &mut System, cfg: &StorageConfig) {
    let header = ["Mount", "Type", "Filesystem", "Used(%)", "Used", "Total"];

    sys.refresh_disks_list();

    let entries = sys
        .disks()
        .iter()
        .filter(|disk| {
            let path = disk.mount_point().to_string_lossy();
            !cfg.exclude_prefixes
                .iter()
                .any(|prefix| path.starts_with(prefix))
        })
        .map(|disk| entry(&disk))
        .collect::<Vec<Entry>>();

    let column_widths = entries
        .iter()
        .map(|entry| {
            vec![
                entry.mount_point.len(),
                entry.disk_type.len(),
                entry.fs_type.len(),
                entry.used_percent.to_string().len(),
                entry.used.to_string().len(),
                entry.total.to_string().len(),
            ]
        })
        .chain(iter::once(header.iter().map(|x| x.len()).collect()))
        .fold(vec![0; header.len()], |acc, x| {
            x.iter()
                .zip(acc.iter())
                .map(|(a, b)| cmp::max(a, &b).to_owned())
                .collect()
        });
    let bar_width = column_widths.iter().sum::<usize>() + (header.len() - 2);

    print_row(header, &column_widths);
    entries.iter().for_each(|entry| {
        print_row(
            [
                entry.mount_point,
                entry.disk_type,
                entry.fs_type,
                &entry.used_percent.to_string(),
                &entry.used.to_string(),
                &entry.total.to_string(),
            ],
            &column_widths,
        );
        print_entry_bar(&entry, bar_width, &cfg.usage_colouring);
    });
}
