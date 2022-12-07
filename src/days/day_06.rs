use std::collections::HashSet;

pub fn main(input: &str) -> (u32, u32) {
    let chars: Vec<char> = input.chars().collect();
    let packet_marker = find_marker(&chars, 4).unwrap();
    let message_marker = find_marker(&chars, 14).unwrap();

    (packet_marker as _, message_marker as _)
}

fn find_marker(chars: &Vec<char>, window_size: usize) -> Option<usize> {
    for (i, window) in chars.windows(window_size).enumerate() {
        let set: HashSet<char> = window.into_iter().copied().collect();
        if set.len() == window_size && i + window_size < chars.len() {
            return Some(i + window_size);
        }
    }

    None
}
