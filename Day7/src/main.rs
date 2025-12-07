use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one(filename: &str) -> u64 {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::with_capacity(1024 * 1024, file);

    let mut beams: HashSet<i64> = HashSet::new();
    let mut total_splits: u64 = 0;
    let mut width: i64 = 0;
    let mut first_line = true;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if first_line {
            width = line.len() as i64;
            if let Some(pos) = line.find('S') {
                beams.insert(pos as i64);
            }
            first_line = false;
            continue;
        }

        if !line.contains('^') {
            continue;
        }

        let splitters: HashSet<i64> = line
            .bytes()
            .enumerate()
            .filter(|(_, c)| *c == b'^')
            .map(|(i, _)| i as i64)
            .collect();

        let mut new_beams: HashSet<i64> = HashSet::with_capacity(beams.len() * 2);

        for col in beams {
            if splitters.contains(&col) {
                total_splits += 1;

                let left = col - 1;
                let right = col + 1;
                if left >= 0 {
                    new_beams.insert(left);
                }
                if right < width {
                    new_beams.insert(right);
                }
            } else {
                new_beams.insert(col);
            }
        }

        beams = new_beams;

        if beams.is_empty() {
            break;
        }
    }

    total_splits
}

fn part_two(filename: &str) -> u128 {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::with_capacity(1024 * 1024, file);

    let mut timelines: HashMap<i64, u128> = HashMap::new();
    let mut width: i64 = 0;
    let mut first_line = true;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        if first_line {
            width = line.len() as i64;
            if let Some(pos) = line.find('S') {
                timelines.insert(pos as i64, 1);
            }
            first_line = false;
            continue;
        }

        if !line.contains('^') {
            continue;
        }

        let splitters: HashSet<_> = line
            .bytes()
            .enumerate()
            .filter(|(_, c)| *c == b'^')
            .map(|(i, _)| i as i64)
            .collect();

        let mut new_timelines: HashMap<i64, u128> = HashMap::with_capacity(timelines.len() * 2);

        for (col, count) in &timelines {
            if splitters.contains(col) {
                let left = col - 1;
                let right = col + 1;
                if left >= 0 {
                    *new_timelines.entry(left).or_insert(0) += count;
                }
                if right < width {
                    *new_timelines.entry(right).or_insert(0) += count;
                }
            } else {
                *new_timelines.entry(*col).or_insert(0) += count;
            }
        }

        timelines = new_timelines;

        if timelines.is_empty() {
            return 0;
        }
    }

    timelines.values().sum()
}

fn main() {
    // Beam is shot from S
    // if it encounters a ^ it will split to its immediate left and right.
    // The beam is blocked by .
    // We count the amount of beams that reached the bottom.
    let count = part_one("input");
    println!("Total splits: {}", count);
    // Need to track how many timelines (aka alt routes) that are possible
    // if the beam only splits left/right.
    let count_two = part_two("input");
    println!("Total timelines: {}", count_two);
}
