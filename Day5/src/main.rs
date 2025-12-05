use std::fs::read_to_string;

fn main() {
    let test_input = read_to_string("input").unwrap();
    let lines: Vec<_> = test_input.lines().collect();
    let empty_idx = lines.iter().position(|l| l.trim().is_empty()).unwrap();

    let ranges: Vec<(i64, i64)> = lines[..empty_idx]
        .iter()
        .map(|range| {
            let (l, h) = range.trim().split_once('-').unwrap();
            (l.parse::<i64>().unwrap(), h.parse::<i64>().unwrap())
        })
        .collect();

    let count = lines[empty_idx + 1..]
        .iter()
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .filter(|n| ranges.iter().any(|(l, h)| n >= l && n <= h))
        .count();

    let merged_ranges = merge_ranges(ranges);
    let total: i64 = merged_ranges.iter().map(|(l, h)| h - l + 1).sum();

    println!("{}", count);
    println!("{}", total);
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = vec![ranges[0]];

    for (start, end) in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}
