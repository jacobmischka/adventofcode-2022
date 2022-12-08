use std::collections::HashSet;

pub fn main(input: &str) -> (u32, u32) {
    let trees: Vec<Vec<i8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        })
        .collect();

    let mut edge_visible: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..trees.len() {
        let mut prev_height = -1;
        for x in 0..trees[y].len() {
            if trees[y][x] > prev_height {
                prev_height = trees[y][x];
                edge_visible.insert((y, x));
            }

            if prev_height == 9 {
                break;
            }
        }

        prev_height = -1;
        for x in (0..trees[y].len()).rev() {
            if trees[y][x] > prev_height {
                prev_height = trees[y][x];
                edge_visible.insert((y, x));
            }

            if prev_height == 9 {
                break;
            }
        }
    }

    for x in 0..trees[0].len() {
        let mut prev_height = -1;
        for y in 0..trees.len() {
            if trees[y][x] > prev_height {
                prev_height = trees[y][x];
                edge_visible.insert((y, x));
            }

            if prev_height == 9 {
                break;
            }
        }

        prev_height = -1;
        for y in (0..trees.len()).rev() {
            if trees[y][x] > prev_height {
                prev_height = trees[y][x];
                edge_visible.insert((y, x));
            }

            if prev_height == 9 {
                break;
            }
        }
    }

    if cfg!(feature = "debug") {
        for y in 0..trees.len() {
            for x in 0..trees[y].len() {
                if edge_visible.contains(&(y, x)) {
                    print!("[{}]", trees[y][x])
                } else {
                    print!(" {} ", trees[y][x])
                }
            }
            println!();
        }
    }

    let mut highest_scenic_score = 0;

    for start_y in 1..(trees.len() - 1) {
        for start_x in 1..(trees[start_y].len() - 1) {
            let current_height = trees[start_y][start_x];
            let mut visible_left = 0;
            let mut x = start_x;
            while x > 0 {
                x -= 1;
                visible_left += 1;
                if trees[start_y][x] >= current_height {
                    break;
                }
            }

            x = start_x;
            let mut visible_right = 0;
            while x < trees[start_y].len() - 1 {
                x += 1;
                visible_right += 1;
                if trees[start_y][x] >= current_height {
                    break;
                }
            }

            let mut visible_up = 0;
            let mut y = start_y;
            while y > 0 {
                y -= 1;
                visible_up += 1;
                if trees[y][start_x] >= current_height {
                    break;
                }
            }

            y = start_y;
            let mut visible_down = 0;
            while y < trees.len() - 1 {
                y += 1;
                visible_down += 1;
                if trees[y][start_x] >= current_height {
                    break;
                }
            }

            let scenic_score = visible_up * visible_down * visible_left * visible_right;
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }

            if cfg!(feature = "debug") {
                print!("{scenic_score}\t");
            }
        }
        if cfg!(feature = "debug") {
            println!();
        }
    }

    (edge_visible.len() as u32, highest_scenic_score as u32)
}
