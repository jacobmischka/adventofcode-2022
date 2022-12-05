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

    let results = match day {
        1 => stringify_u32(days::day_01::main(&input)),
        2 => stringify_u32(days::day_02::main(&input)),
        3 => stringify_u32(days::day_03::main(&input)),
        4 => stringify_u32(days::day_04::main(&input)),
        5 => days::day_05::main(&input),
        _ => panic!("unsupported day {day}"),
    };

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn stringify_u32(int_results: (u32, u32)) -> (String, String) {
    (int_results.0.to_string(), int_results.1.to_string())
}

fn get_input() -> Result<String> {
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut s)?;
    Ok(s)
}
