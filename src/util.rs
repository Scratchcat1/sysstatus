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

// // Convert a number of bytes into a human readable format
// // Will have no more than 4 digits
// pub fn human_readable_bytesize(size: u64) -> String {
//     if size == 0 {
//         return "0B".to_string();
//     }
//     let prefix_index = (64 - size.leading_zeros() - 1) / 10;
//     let prefixes = ["", "Ki", "Mi", "Gi", "Ti", "Pi"];
//     let new_size = size as f32 / (1 << prefix_index * 10) as f32;
//     format!(
//         "{:.4}{}B",
//         new_size.to_string(),
//         prefixes[prefix_index as usize]
//     )
// }

// mod test {
//     use crate::util::human_readable_bytesize;

//     #[test]
//     fn test_human_readable_bytesize() {
//         assert_eq!(human_readable_bytesize(0), "0B");
//         assert_eq!(human_readable_bytesize(1000), "1000B");
//         assert_eq!(human_readable_bytesize(1023), "1023B");
//         assert_eq!(human_readable_bytesize(1024), "1KiB");
//         assert_eq!(human_readable_bytesize(1025), "1.00KiB");
//         assert_eq!(human_readable_bytesize(1024 * 1024 - 1), "1023KiB");
//         assert_eq!(human_readable_bytesize(1024 * 1024), "1MiB");
//     }
// }
