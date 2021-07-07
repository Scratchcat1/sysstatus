use crate::config::ServiceConfig;
use crate::util::select_colour_number;
use bytesize::ByteSize;
use colored::*;
use std::collections::HashMap;
use std::process::Command;
use std::str;

pub fn single_service(
    service_name: &str,
    service_status: &str,
    cfg: &ServiceConfig,
    service_name_align: usize,
) -> String {
    let mut active_state = None;
    let mut sub_state = None;
    let mut mem_current = None;
    let mut id = None;

    for line in service_status.split("\n") {
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

    let mem_colour = match (&cfg.memory_usage, mem_current) {
        (Some(mem_usage_cond), Some(mem_current)) => {
            select_colour_number(mem_current.as_u64(), mem_usage_cond)
        }
        _ => Color::White,
    };

    let state_color = match (active_state, sub_state) {
        (Some("active"), Some("running")) => Color::Green,
        _ => Color::Yellow,
    };

    let coloured_state = format!(
        "{} ({})",
        active_state.unwrap_or("Unknown active state"),
        sub_state.unwrap_or("Unknown sub state")
    )
    .color(state_color);

    format!(
        "    {:.<align$}: {:<18} Mem: {}",
        service_name,
        coloured_state,
        mem_current
            .unwrap_or(ByteSize::b(0))
            .to_string()
            .color(mem_colour),
        align = service_name_align + 3
    )
}

pub fn services(cfg: &HashMap<String, ServiceConfig>) -> String {
    let mut service_name_align = 0;
    let mut command = Command::new("systemctl");
    command.arg("show");

    let mut ordered_service_names = cfg.keys().collect::<Vec<&String>>();
    ordered_service_names.sort();
    for service_name in &ordered_service_names {
        service_name_align = service_name_align.max(service_name.len());
        command.arg(service_name);
    }
    let raw_output = command.output().expect("failed to execute process");
    let output = match str::from_utf8(&raw_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // Service outputs are separated by a single blank line/two consecutive newlines
    let mut service_statuses = output.split("\n\n");

    ordered_service_names
        .iter()
        .map(|service_name| {
            let service_cfg = cfg
                .get(*service_name)
                .expect("Cannot find value for key in hashmap");
            let service_status = service_statuses
                .next()
                .expect("Subcommand returned the incorrect number of services");
            single_service(
                service_name,
                service_status,
                service_cfg,
                service_name_align,
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
