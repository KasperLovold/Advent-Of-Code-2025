use std::fs::read_to_string;

type Pos = (i32, i32);

fn main() {
    let test_input = read_to_string("input").unwrap();
    let map: Vec<Vec<bool>> = test_input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect();

    let count = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, is_at)| {
                    **is_at && !has_four_or_more_adjacent(&map, (*x as i32, y as i32))
                })
                .count()
        })
        .sum::<usize>();

    let part2 = remove_accessible(&mut map.clone());

    println!("{}", count);
    println!("{}", part2);
}

fn has_four_or_more_adjacent(map: &Vec<Vec<bool>>, pos: Pos) -> bool {
    let (x, y) = pos;
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let count = directions
        .iter()
        .filter(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;

            ny >= 0
                && ny < map.len() as i32
                && nx >= 0
                && nx < map[0].len() as i32
                && map[ny as usize][nx as usize]
        })
        .count();

    count >= 4
}

fn remove_accessible(map: &mut Vec<Vec<bool>>) -> usize {
    let height = map.len();
    if height == 0 {
        return 0;
    }
    let width = map[0].len();

    let mut total_removed = 0;
    let mut candidates: Vec<(usize, usize)> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&(x, y)| map[y][x] && !has_four_or_more_adjacent(map, (x as i32, y as i32)))
        .collect();

    while !candidates.is_empty() {
        for &(x, y) in &candidates {
            map[y][x] = false;
        }
        total_removed += candidates.len();

        let mut seen = vec![vec![false; width]; height];
        let next: Vec<(usize, usize)> = candidates
            .iter()
            .flat_map(|&(x, y)| neighbors(x, y, width, height))
            .filter(|&(nx, ny)| {
                if map[ny][nx] && !seen[ny][nx] {
                    seen[ny][nx] = true;
                    !has_four_or_more_adjacent(map, (nx as i32, ny as i32))
                } else {
                    false
                }
            })
            .collect();

        candidates = next;
    }
    total_removed
}

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1i32, -1i32),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}
