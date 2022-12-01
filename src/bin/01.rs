use adventofcode_2022::get_input;

fn main() {
    let input = get_input().unwrap();

    let mut cals_counts: Vec<u32> = Vec::new();
    let mut current_sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            cals_counts.push(current_sum);
            current_sum = 0;
            continue;
        }

        let cals: u32 = line.parse().unwrap();
        current_sum += cals;
    }

    cals_counts.push(current_sum);
    cals_counts.sort_by(|a, b| b.cmp(&a));

    println!("Part 1: {}", cals_counts[0]);
    println!(
        "Part 2: {}",
        cals_counts[0] + cals_counts[1] + cals_counts[2]
    );
}
