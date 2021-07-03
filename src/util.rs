use crate::config::ConditionalColour;
use colored::Color;

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
