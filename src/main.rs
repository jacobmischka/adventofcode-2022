use adventofcode_2022::days;
use std::{
    env,
    io::{self, Read, Result},
};

fn main() {
    let day: u32 = env::args()
        .skip(1)
        .next()
        .expect("missing day argument")
        .parse()
        .unwrap();
    let input = get_input().unwrap();
    let trimmed = input.trim();

    let results = match day {
        1 => stringify_u32(days::day_01::main(trimmed)),
        2 => stringify_u32(days::day_02::main(trimmed)),
        3 => stringify_u32(days::day_03::main(trimmed)),
        4 => stringify_u32(days::day_04::main(trimmed)),
        5 => days::day_05::main(trimmed),
        6 => stringify_u32(days::day_06::main(trimmed)),
        7 => stringify_u32(days::day_07::main(trimmed)),
        8 => stringify_u32(days::day_08::main(trimmed)),
        9 => stringify_u32(days::day_09::main(trimmed)),
        10 => days::day_10::main(trimmed),
        11 => stringify_u64(days::day_11::main(trimmed)),
        12 => stringify_u32(days::day_12::main(trimmed)),
        13 => stringify_u32(days::day_13::main(trimmed)),
        14 => stringify_u32(days::day_14::main(trimmed)),
        15 => stringify_u64(days::day_15::main(trimmed)),
        16 => stringify_u32(days::day_16::main(trimmed)),
        17 => stringify_u64(days::day_17::main(trimmed)),
        18 => stringify_u32(days::day_18::main(trimmed)),
        19 => stringify_u32(days::day_19::main(trimmed)),
        20 => stringify_i64(days::day_20::main(trimmed)),
        _ => panic!("unsupported day {day}"),
    };

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn stringify_u32(int_results: (u32, u32)) -> (String, String) {
    (int_results.0.to_string(), int_results.1.to_string())
}

fn stringify_u64(int_results: (u64, u64)) -> (String, String) {
    (int_results.0.to_string(), int_results.1.to_string())
}

#[allow(unused)]
fn stringify_i32(int_results: (i32, i32)) -> (String, String) {
    (int_results.0.to_string(), int_results.1.to_string())
}

fn stringify_i64(int_results: (i64, i64)) -> (String, String) {
    (int_results.0.to_string(), int_results.1.to_string())
}

fn get_input() -> Result<String> {
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut s)?;
    Ok(s)
}
