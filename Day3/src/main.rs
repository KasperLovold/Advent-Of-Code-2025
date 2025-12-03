use std::fs::read_to_string;

fn main() {
    let test_file = read_to_string("input").unwrap();
    let part1: i32 = part_one(&test_file);
    let part2: i64 = part_two(&test_file);

    print!("{}\n", part1);
    print!("{}", part2);
}

fn part_one(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u8> = line
                .bytes()
                .filter_map(|b| b.is_ascii_digit().then(|| b - b'0'))
                .collect();

            let mut max_after = *digits.last().unwrap();
            let mut best = 0i32;

            for &d in digits.iter().rev().skip(1) {
                let val = (d as i32) * 10 + (max_after as i32);
                best = best.max(val);
                max_after = max_after.max(d);
            }

            best
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u8> = line
                .bytes()
                .filter_map(|b| b.is_ascii_digit().then(|| b - b'0'))
                .collect();

            let k = 12;
            let mut to_remove = digits.len() - k;
            let mut stack: Vec<u8> = Vec::with_capacity(k + 1);

            for d in digits {
                while to_remove > 0 && stack.last().is_some_and(|&top| top < d) {
                    stack.pop();
                    to_remove -= 1;
                }
                stack.push(d);
            }

            stack.truncate(k);

            stack.iter().fold(0i64, |acc, &d| acc * 10 + d as i64)
        })
        .sum()
}
