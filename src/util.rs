use crate::config::ConditionalColour;
use colored::Color;
use std::{cmp, iter};

pub fn select_colour_number<T: PartialOrd>(val: T, cond_colour: &ConditionalColour<T>) -> Color {
    let mut prev_colour = cond_colour.default_colour;
    for level in &cond_colour.levels {
        if val < level.min {
            break;
        }
        prev_colour = level.colour;
    }
    prev_colour
}

pub fn print_row<'a>(
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

pub fn column_widths<'a>(header: &[&str], entries: impl Iterator<Item = Vec<usize>>) -> Vec<usize> {
    entries
        .into_iter()
        .chain(iter::once(header.iter().map(|x| x.len()).collect()))
        .fold(vec![0; header.len()], |acc, x: Vec<usize>| {
            x.iter()
                .zip(acc.iter())
                .map(|(a, b)| cmp::max(a, &b).to_owned())
                .collect()
        })
}
