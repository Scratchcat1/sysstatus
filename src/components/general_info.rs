use crate::config::GeneralInfoConfig;
use crate::util::select_colour_number;
use bytesize::ByteSize;
use colored::*;
use sysinfo::{CpuExt, System, SystemExt};

pub fn load(sys: &mut System, cfg: &GeneralInfoConfig) -> String {
    sys.refresh_cpu();

    let load_avg = sys.load_average();
    let coloured_loads: Vec<ColoredString> = [load_avg.one, load_avg.five, load_avg.fifteen]
        .iter()
        .map(|load| {
            load.to_string().color(select_colour_number(
                *load / sys.cpus().len() as f64,
                &cfg.load_avg,
            ))
        })
        .collect();
    format!(
        "{} (1m), {} (5m), {} (15m)",
        coloured_loads[0], coloured_loads[1], coloured_loads[2],
    )
}

pub fn memory(sys: &mut System, cfg: &GeneralInfoConfig) -> String {
    sys.refresh_memory();
    let used_mem = sys.used_memory();
    let total_mem = sys.total_memory();
    let available_mem = sys.available_memory();

    let colour = select_colour_number(used_mem as f32 / total_mem as f32, &cfg.memory);
    format!(
        "{} used, {} available, {} total",
        ByteSize::kb(used_mem).to_string().color(colour),
        ByteSize::kb(available_mem),
        ByteSize::kb(total_mem)
    )
}

pub fn cpu(sys: &mut System, cfg: &GeneralInfoConfig) -> String {
    sys.refresh_cpu();
    let processors = sys.cpus();
    let freq_colour = select_colour_number(processors[0].frequency(), &cfg.cpu_frequency);
    format!(
        "{} - {} MHz",
        processors[0].brand(),
        processors[0].frequency().to_string().color(freq_colour)
    )
}

pub fn uptime(sys: &mut System) -> String {
    let uptime = sys.uptime();
    let seconds = uptime % 60;
    let minutes = (uptime / 60) % 60;
    let hours = (uptime / (60 * 60)) % 24;
    let days = (uptime / (24 * 60 * 60)) % 7;
    let weeks = uptime / (24 * 60 * 60 * 7);
    format!(
        "{} weeks, {} days, {} hours, {} minutes, {} seconds",
        weeks, days, hours, minutes, seconds
    )
}

pub fn general_info(sys: &mut System, cfg: &GeneralInfoConfig) -> String {
    format!(
        "General:
    {:.<max$}: {}
    {:.<max$}: {}

    {:.<max$}: {}
    {:.<max$}: {}

    {:.<max$}: {}
    {:.<max$}: {}
    ",
        "OS",
        sys.long_os_version().unwrap_or("N/A".to_string()),
        "Kernel",
        sys.kernel_version().unwrap_or("N/A".to_string()),
        "Uptime",
        uptime(sys),
        "Load",
        load(sys, cfg),
        "Memory",
        memory(sys, cfg),
        "CPU",
        cpu(sys, cfg),
        max = 9
    )
}
