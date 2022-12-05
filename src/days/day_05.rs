pub fn main(input: &str) -> (String, String) {
    let mut crates_data: Vec<&str> = Vec::new();
    let mut crates_9000: Vec<Vec<char>> = Vec::new();
    let mut crates_9001: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            let num_crates = crates_data.pop().unwrap().split_whitespace().count();
            crates_9000 = vec![Vec::new(); num_crates];

            while let Some(row) = crates_data.pop() {
                let chars: Vec<char> = row.chars().collect();
                let mut i = 0;
                for crate_index in 0..num_crates {
                    if chars[i] == '[' {
                        crates_9000[crate_index].push(chars[i + 1]);
                    }
                    i += 4;
                }
            }
            crates_9001 = crates_9000.clone();
        } else if line.starts_with("move") {
            let mut words = line.split_whitespace();
            let _ = words.next();
            let num: usize = words.next().unwrap().parse().unwrap();
            let _ = words.next();
            let src: usize = words.next().unwrap().parse().unwrap();
            let _ = words.next();
            let dest: usize = words.next().unwrap().parse().unwrap();

            let mut buf = Vec::new();
            for _ in 0..num {
                let c9000 = crates_9000[src - 1].pop().unwrap();
                crates_9000[dest - 1].push(c9000);

                let c9001 = crates_9001[src - 1].pop().unwrap();
                buf.push(c9001);
            }
            for c in buf.into_iter().rev() {
                crates_9001[dest - 1].push(c);
            }
        } else {
            crates_data.push(line);
        }
    }

    let p1: String = crates_9000.iter().map(|c| c.last().unwrap()).collect();
    let p2: String = crates_9001.iter().map(|c| c.last().unwrap()).collect();

    (p1, p2)
}
