const DECRYPTION_KEY: i64 = 811589153;

pub fn main(input: &str) -> (i64, i64) {
    let mut coords: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    (
        {
            let mixed = mix_coords(&coords, 1);
            let index_of_0 = mixed
                .iter()
                .enumerate()
                .find_map(|(i, val)| if *val == 0 { Some(i) } else { None })
                .unwrap();

            mixed[(index_of_0 + 1000) % mixed.len()]
                + mixed[(index_of_0 + 2000) % mixed.len()]
                + mixed[(index_of_0 + 3000) % mixed.len()]
        },
        {
            for c in &mut coords {
                *c = *c * DECRYPTION_KEY;
            }
            let mixed = mix_coords(&coords, 10);
            let index_of_0 = mixed
                .iter()
                .enumerate()
                .find_map(|(i, val)| if *val == 0 { Some(i) } else { None })
                .unwrap();

            mixed[(index_of_0 + 1000) % mixed.len()]
                + mixed[(index_of_0 + 2000) % mixed.len()]
                + mixed[(index_of_0 + 3000) % mixed.len()]
        },
    )
}

fn mix_coords(input: &Vec<i64>, iterations: usize) -> Vec<i64> {
    let mut coords: Vec<(usize, i64)> = input.iter().copied().enumerate().collect();
    let len = coords.len() as i64;

    let mut i = 0;
    for _ in 0..iterations {
        let mut desired_original_index = 0;

        while desired_original_index < coords.len() {
            let (original_index, val) = coords[i];
            if original_index != desired_original_index {
                i = (i + 1) % coords.len();
                continue;
            }

            let transformation_start_index = i as i64;

            let sign = val.signum();
            let offset = ((val * sign) % (len - 1)) * sign;
            let new_index = transformation_start_index + offset;
            let new_index = if new_index < 0 {
                len + new_index - 1
            } else if new_index >= len {
                new_index % (len - 1)
            } else {
                new_index
            } as usize;

            if new_index == 0 && original_index != 0 {
                coords.remove(i);
                coords.push((original_index, val));
            } else {
                coords.remove(i);
                coords.insert(new_index, (original_index, val));
            }

            desired_original_index += 1;
        }
    }

    coords.into_iter().map(|(_, v)| v).collect()
}

#[test]
fn mixing_works() {
    assert_eq!(
        mix_coords(&vec![1, 2, -3, 3, -2, 0, 4], 1),
        vec![1, 2, -3, 4, 0, 3, -2]
    );

    assert_eq!(
        mix_coords(&vec![4, -2, 5, 6, 7, 8, 11], 1),
        vec![4, 7, -2, 11, 5, 8, 6]
    );

    assert_eq!(
        mix_coords(&vec![4, -2, 5, 6, 7, 8, 9], 1),
        vec![4, -2, 5, 8, 6, 7, 9]
    );
}
