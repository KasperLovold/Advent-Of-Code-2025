use std::fs::read_to_string;

fn main() {
    let test_input = read_to_string("input").unwrap();
    println!("{}", part_one(&test_input));
    println!("{}", part_two(&test_input));
}

fn part_one(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let (number_lines, operator_line) = lines.split_at(lines.len() - 1);

    let number_rows: Vec<Vec<i64>> = number_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    let operators: Vec<char> = operator_line[0]
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    let num_cols = operators.len();
    let mut sum = 0i64;

    for col in 0..num_cols {
        let numbers: Vec<i64> = number_rows.iter().map(|row| row[col]).collect();
        let op = operators[col];

        let result = match op {
            '*' => numbers.iter().copied().product::<i64>(),
            '+' => numbers.iter().copied().sum::<i64>(),
            _ => panic!("Unknown operator: {}", op),
        };

        sum += result;
    }

    sum
}

fn part_two(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = grid.len();
    let num_cols = max_len;

    let mut problems: Vec<(Vec<i64>, char)> = Vec::new();
    let mut current_numbers: Vec<i64> = Vec::new();
    let mut current_operator: Option<char> = None;

    for col in (0..num_cols).rev() {
        let column: Vec<char> = grid.iter().map(|row| row[col]).collect();
        let is_separator = column.iter().all(|&c| c == ' ');

        if is_separator {
            if !current_numbers.is_empty() {
                if let Some(op) = current_operator {
                    problems.push((current_numbers.clone(), op));
                }
                current_numbers.clear();
                current_operator = None;
            }
        } else {
            let op_char = column[num_rows - 1];
            if op_char != ' ' {
                current_operator = Some(op_char);
            }

            let digits: String = column[..num_rows - 1]
                .iter()
                .filter(|&&c| c != ' ')
                .collect();

            if let Ok(num) = digits.parse::<i64>() {
                current_numbers.push(num);
            }
        }
    }

    if !current_numbers.is_empty() {
        if let Some(op) = current_operator {
            problems.push((current_numbers, op));
        }
    }

    let mut total = 0i64;
    for (numbers, op) in &problems {
        let result: i64 = match op {
            '*' => numbers.iter().copied().product::<i64>(),
            '+' => numbers.iter().copied().sum::<i64>(),
            _ => panic!("Unknown operator: {}", op),
        };
        total += result;
    }

    total
}
