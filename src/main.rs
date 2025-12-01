use std::fs::read_to_string;

fn main() {
    // Turn knob left or right, count amount of times it goes 0
    // if it goes over 99 reset to 0
    // if it goes below 0, reset to 99
    // the dial starts at 50
    // R increments
    // L decrements
    let input = read_to_string("input").unwrap();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse_moves(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines().map(|line| {
        let line = line.trim();
        let num: i32 = line[1..].parse().unwrap();
        if line.starts_with('L') { -num } else { num }
    })
}

fn part_one(input: &str) -> i32 {
    parse_moves(input)
        .fold((50, 0), |(pos, count), num| {
            let new_pos = (pos + num).rem_euclid(100);
            (new_pos, count + if new_pos == 0 { 1 } else { 0 })
        })
        .1
}

fn part_two(input: &str) -> i32 {
    let mut pos = 50;
    let mut password = 0;

    for num in parse_moves(input) {
        let raw = pos + num;

        password += if num > 0 {
            raw / 100
        } else {
            let first_zero = if pos > 0 { 0 } else { -100 };
            if raw <= first_zero {
                1 + (first_zero - raw) / 100
            } else {
                0
            }
        };

        pos = raw.rem_euclid(100);
    }

    password
}
