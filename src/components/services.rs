use crate::config::ServiceConfig;
use crate::util;
use bytesize::ByteSize;
use colored::*;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Default)]
struct Entry<'a> {
    service_name: &'a str,
    active_state: &'a str,
    sub_state: &'a str,
    mem_current: Option<ByteSize>,
}

fn parse_entry<'a>(service_name: &'a str, status_output: &'a str) -> Entry<'a> {
    let mut active_state = None;
    let mut sub_state = None;
    let mut mem_current = None;
    let mut id = None;

    for line in status_output.split("\n") {
        if let Some(new_active_state) = line.strip_prefix("ActiveState=") {
            active_state = Some(new_active_state);
        } else if let Some(new_sub_state) = line.strip_prefix("SubState=") {
            sub_state = Some(new_sub_state);
        } else if let Some(new_mem_current) = line.strip_prefix("MemoryCurrent=") {
            mem_current = Some(ByteSize::b(new_mem_current.parse::<u64>().unwrap_or(0)));
        } else if let Some(new_id) = line.strip_prefix("Id=") {
            id = Some(new_id);
        }
    }
    assert!(id.expect("No service Id").contains(service_name));

    Entry {
        service_name,
        active_state: active_state.unwrap_or("Unknown active state"),
        sub_state: sub_state.unwrap_or("Unknown sub state"),
        mem_current: mem_current,
    }
}

pub fn format_service_state(active_state: &str, sub_state: &str, width: usize) -> String {
    let state_color = match (active_state, sub_state) {
        ("active", "running") => Color::Green,
        _ => Color::Yellow,
    };

    let state = format!("{} ({})", active_state, sub_state);
    format!("{: <size$}", state, size = width)
        .color(state_color)
        .to_string()
}

pub fn format_mem_current(mem_current: &Option<ByteSize>, cfg: &ServiceConfig) -> String {
    let mem_colour = match (&cfg.memory_usage, mem_current) {
        (Some(mem_usage_cond), Some(mem_current)) => {
            util::select_colour_number(mem_current.as_u64(), mem_usage_cond)
        }
        _ => Color::White,
    };
    mem_current
        .unwrap_or(ByteSize::b(0))
        .to_string()
        .color(mem_colour)
        .to_string()
}

pub fn systemd_show(service_names: &[&str]) -> String {
    let mut command = Command::new("systemctl");
    command.arg("show");

    for service_name in service_names {
        command.arg(service_name);
    }
    let raw_output = command.output().expect("failed to execute process");
    let output = match String::from_utf8(raw_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    output
}

pub fn print_services(cfg: &HashMap<String, ServiceConfig>, indent: &str) {
    let header = ["Service", "Status", "Memory Usage"];

    let mut ordered_service_names = cfg.keys().map(|name| name.as_str()).collect::<Vec<&str>>();
    ordered_service_names.sort();

    // Service outputs are separated by a single blank line/two consecutive newlines
    let systemd_show_output = systemd_show(&ordered_service_names);
    let mut service_statuses = systemd_show_output.split("\n\n");

    let entries = &ordered_service_names
        .iter()
        .map(|service_name| {
            let service_status = service_statuses
                .next()
                .expect("Subcommand returned the incorrect number of services");
            parse_entry(service_name, service_status)
        })
        .collect::<Vec<Entry>>();

    let column_widths = util::column_widths(
        &header,
        entries.iter().map(|entry| {
            vec![
                entry.service_name.len(),
                entry.active_state.len() + entry.sub_state.len() + 3,
                entry
                    .mem_current
                    .unwrap_or(ByteSize::b(0))
                    .to_string()
                    .len(),
            ]
        }),
    );

    println!("System services:");
    util::print_row(header, &column_widths, Some(indent));
    entries.iter().for_each(|entry| {
        let service_cfg = cfg
            .get(entry.service_name)
            .expect("Cannot find value for key in hashmap");
        util::print_row(
            [
                entry.service_name,
                &format_service_state(entry.active_state, entry.sub_state, column_widths[1]),
                &format_mem_current(&entry.mem_current, service_cfg),
            ],
            &column_widths,
            Some(indent),
        );
    });
}
