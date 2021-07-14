use crate::config::LastLoginConfig;
use crate::util;
use colored::Color;
use lazy_regex::regex;
use std::process::Command;

#[derive(Debug)]
struct Entry {
    username: String,
    location: String,
    start_time: String,
    end_time: String,
}

fn parse_entry<'a>(line: &'a str) -> Option<Entry> {
    let separator_regex = regex!(r"(?:\s{2,})|(?:\s-\s)");

    let items = separator_regex.split(line).collect::<Vec<_>>();

    if items.len() < 5 {
        return None;
    }

    Some(Entry {
        username: items[0].to_string(),
        location: items[2].to_string(),
        start_time: items[3].to_string(),
        end_time: items[4].to_string(),
    })
}

fn user_last_logins_output(username: &str, max_lines: usize, since: Option<&String>) -> String {
    let mut command = Command::new("last");
    command
        .arg("--ip")
        .arg("--time-format=full")
        .arg("--limit")
        .arg(max_lines.to_string());
    since.map(|since| {
        command.arg("--since").arg(since);
    });
    command.arg(username);
    let raw_output = command.output().expect("failed to execute process");
    let output = match String::from_utf8(raw_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    output
}

fn end_time_colour(end_time: &str) -> Option<Color> {
    match end_time {
        "still logged in" => Some(Color::Green),
        _ => None,
    }
}

pub fn print_last_login(cfg: &LastLoginConfig, indent: &str) {
    let header = ["Username", "Location", "Start", "End"];

    let mut entries = Vec::new();
    for (username, max_lines) in cfg.users.iter() {
        let output = user_last_logins_output(username, *max_lines, cfg.since.as_ref());

        entries.extend(output.lines().flat_map(|line| parse_entry(line)));
    }

    let column_widths = util::column_widths(
        &header,
        entries.iter().map(|entry| {
            vec![
                entry.username.len(),
                entry.location.len(),
                entry.start_time.len(),
                entry.end_time.len(),
            ]
        }),
    );

    println!("Logins:");
    util::print_row(header, &column_widths, Some(indent));
    entries.iter().for_each(|entry| {
        let formatted_cells = util::format_width(
            [
                entry.username.as_str(),
                entry.location.as_str(),
                entry.start_time.as_str(),
                entry.end_time.as_str(),
            ],
            &column_widths,
        );
        let colours = [None, None, None, end_time_colour(&entry.end_time)];
        util::print_row_colour(formatted_cells, colours, Some(indent))
    });
}
