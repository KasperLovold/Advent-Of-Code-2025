use std::fs;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false;
        }
        let (small, large) = if self.size[rx] < self.size[ry] {
            (rx, ry)
        } else {
            (ry, rx)
        };
        self.parent[small] = large;
        self.size[large] += self.size[small];
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        (0..self.parent.len())
            .filter_map(|i| (self.find(i) == i).then_some(self.size[i]))
            .collect()
    }
}

fn parse(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

fn sorted_pairs(boxes: &[(i64, i64, i64)]) -> Vec<(i64, usize, usize)> {
    let mut pairs: Vec<_> = (0..boxes.len())
        .flat_map(|i| {
            (i + 1..boxes.len()).map(move |j| {
                let (dx, dy, dz) = (
                    boxes[i].0 - boxes[j].0,
                    boxes[i].1 - boxes[j].1,
                    boxes[i].2 - boxes[j].2,
                );
                (dx * dx + dy * dy + dz * dz, i, j)
            })
        })
        .collect();
    pairs.sort_unstable_by_key(|&(dist, _, _)| dist);
    pairs
}

fn part1(boxes: &[(i64, i64, i64)], pairs: &[(i64, usize, usize)]) -> usize {
    let mut uf = UnionFind::new(boxes.len());

    for &(_, i, j) in pairs.iter().take(1000) {
        uf.union(i, j);
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn part2(boxes: &[(i64, i64, i64)], pairs: &[(i64, usize, usize)]) -> i64 {
    let mut uf = UnionFind::new(boxes.len());
    let mut last_pair = (0, 0);

    for &(_, i, j) in pairs {
        if uf.union(i, j) {
            last_pair = (i, j);
        }
    }

    let (i, j) = last_pair;
    boxes[i].0 * boxes[j].0
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let boxes = parse(&input);
    let pairs = sorted_pairs(&boxes);

    println!("Part 1: {}", part1(&boxes, &pairs));
    println!("Part 2: {}", part2(&boxes, &pairs));
}
