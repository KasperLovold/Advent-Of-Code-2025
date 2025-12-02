use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    let sum: i128 = input
        .split(',')
        .filter_map(|pair| {
            let (l, r) = pair.trim().split_once('-')?;
            Some((l.parse::<i128>().ok()?, r.parse::<i128>().ok()?))
        })
        .flat_map(|(l, r)| l..=r)
        .filter(|&n| is_invalid_two(&n.to_string()))
        .sum();

    println!("{sum}");
}

fn is_invalid_one(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

fn is_invalid_two(s: &str) -> bool {
    let len = s.len();
    (1..=len / 2).any(|pattern_len| {
        if len % pattern_len != 0 {
            return false;
        }
        let pattern = &s[..pattern_len];
        pattern.repeat(len / pattern_len) == s
    })
}
