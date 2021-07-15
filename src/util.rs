use crate::config::ConditionalColour;
use colored::{Color, Colorize};
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
    prefix: Option<&str>,
) {
    println!(
        "{}{}",
        prefix.unwrap_or(""),
        items_iter
            .into_iter()
            .zip(column_sizes.into_iter())
            .map(|(name, size)| format!("{: <size$}", name, size = size))
            .collect::<Vec<String>>()
            .join(&"  ")
    );
}

pub fn format_width<'a>(
    items_iter: impl IntoIterator<Item = &'a str>,
    column_sizes: impl IntoIterator<Item = &'a usize>,
) -> Vec<String> {
    items_iter
        .into_iter()
        .zip(column_sizes.into_iter())
        .map(|(name, size)| format!("{: <size$}", name, size = size))
        .collect::<Vec<String>>()
}

pub fn print_row_colour(
    items_iter: impl IntoIterator<Item = String>,
    colours_iter: impl IntoIterator<Item = Option<Color>>,
    prefix: Option<&str>,
) {
    println!(
        "{}{}",
        prefix.unwrap_or(""),
        items_iter
            .into_iter()
            .zip(colours_iter.into_iter())
            .map(|(name, colour)| match colour {
                Some(colour) => name.color(colour).to_string(),
                None => name,
            })
            .collect::<Vec<String>>()
            .join(&"  ")
    );
}

pub fn column_widths(header: &[&str], entries: impl Iterator<Item = Vec<usize>>) -> Vec<usize> {
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

#[cfg(test)]
mod test {
    use crate::config::ColouringLevel;
    use crate::config::ConditionalColour;
    use crate::util;
    use colored::Color;

    #[test]
    fn test_select_colour_number() {
        let cc = ConditionalColour {
            default_colour: Color::White,
            levels: vec![
                ColouringLevel {
                    min: 1,
                    colour: Color::Green,
                },
                ColouringLevel {
                    min: 5,
                    colour: Color::Yellow,
                },
                ColouringLevel {
                    min: 10,
                    colour: Color::Red,
                },
            ],
        };
        assert_eq!(util::select_colour_number(0, &cc), Color::White);
        assert_eq!(util::select_colour_number(1, &cc), Color::Green);
        assert_eq!(util::select_colour_number(4, &cc), Color::Green);
        assert_eq!(util::select_colour_number(5, &cc), Color::Yellow);
        assert_eq!(util::select_colour_number(9, &cc), Color::Yellow);
        assert_eq!(util::select_colour_number(10, &cc), Color::Red);
    }
}
