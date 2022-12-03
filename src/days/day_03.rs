use std::collections::HashSet;

pub fn main(input: &str) -> (u32, u32) {
    return (check_rucksacks(input), check_badges(input));
}

fn check_badges(input: &str) -> u32 {
    let all_sacks: Vec<&str> = input.lines().collect();
    let mut i = 0;

    let mut priority_sum = 0;
    while i < all_sacks.len() {
        let sack1: Vec<char> = all_sacks[i].chars().collect();
        let sack2: HashSet<char> = all_sacks[i + 1].chars().collect();
        let sack3: HashSet<char> = all_sacks[i + 2].chars().collect();

        for item in sack1 {
            if sack2.contains(&item) && sack3.contains(&item) {
                priority_sum += get_priority(item);
                break;
            }
        }

        i += 3;
    }

    priority_sum
}

fn check_rucksacks(input: &str) -> u32 {
    let rucksacks: Vec<&str> = input.lines().collect();
    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let common_item = find_common_item(rucksack).expect("no common item");
        priority_sum += get_priority(common_item);
    }

    priority_sum
}

fn find_common_item(sack: &str) -> Option<char> {
    let items: Vec<char> = sack.chars().collect();
    let (left, right) = items.split_at(items.len() / 2);
    let right: HashSet<char> = right.iter().copied().collect();
    for item in left {
        if right.contains(item) {
            return Some(*item);
        }
    }

    return None;
}

fn get_priority(item: char) -> u32 {
    let item_val: u32 = item.into();
    let lower_start_val: u32 = 'a'.into();
    let upper_start_val: u32 = 'A'.into();
    let lower_start_val = lower_start_val - 1;
    let upper_start_val = upper_start_val - 27;
    item_val
        - if item.is_ascii_lowercase() {
            lower_start_val
        } else {
            upper_start_val
        }
}
