pub fn main(input: &str) -> (String, String) {
    let mut sum = 0;
    for line in input.lines() {
        sum += snafu_to_decimal(line);
    }

    (decimal_to_snafu(sum), "Merry Christmas!".to_string())
}

pub fn decimal_to_snafu(decimal: i64) -> String {
    let mut snafu = String::new();

    let mut len = 0;
    let mut scratch = decimal;

    while scratch > 0 {
        len += 1;
        if scratch == 4 || scratch == 3 {
            len += 1;
        }
        scratch /= 5;
    }

    let mut scratch = decimal;
    for digit in 0..len {
        let place = len - digit as u32;
        let digit_val = 5i64.pow(place - 1);
        let place_val = if digit_val == 1 {
            scratch
        } else {
            let base = (scratch as f64 / digit_val as f64).round() as i64;
            if (scratch - base * digit_val) / (digit_val / 5) > 2 {
                base + 1
            } else {
                base
            }
        };

        snafu.push(match place_val {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!("bad place_val {place_val} for decimal {decimal}"),
        });

        scratch -= place_val * digit_val;
    }

    snafu
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut result = 0;
    let len = snafu.len() as u32 - 1;
    for (digit, c) in snafu.chars().enumerate() {
        let place = len - digit as u32;
        let digit_val = 5i64.pow(place);
        let place_val = match c {
            '2' => 2 * digit_val,
            '1' => digit_val,
            '0' => 0,
            '-' => -1 * digit_val,
            '=' => -2 * digit_val,
            _ => unreachable!("bad snafu {snafu}"),
        };

        result += place_val;
    }

    result as i64
}

#[test]
fn decimal_to_snafu_works() {
    let table = "
  Decimal          SNAFU
        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       15            1=0
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0"
        .trim();

    for line in table.lines().skip(1) {
        let mut chunks = line.trim().split_whitespace();
        let decimal = chunks.next().unwrap().parse().unwrap();
        let snafu = chunks.next().unwrap();
        assert_eq!(decimal_to_snafu(decimal), snafu, "{}", line);
    }
}

#[test]
fn snafu_to_decimal_works() {
    let table = "
 SNAFU  Decimal
1=-0-2     1747
 12111      906
  2=0=      198
    21       11
  2=01      201
   111       31
 20012     1257
   112       32
 1=-1=      353
  1-12      107
    12        7
    1=        3
   122       37"
        .trim();

    for line in table.lines().skip(1) {
        let mut chunks = line.trim().split_whitespace();
        let snafu = chunks.next().unwrap();
        let decimal: i64 = chunks.next().unwrap().parse().unwrap();
        assert_eq!(snafu_to_decimal(snafu), decimal, "{}", line);
    }
}
