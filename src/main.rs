use array2d::Array2D;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let chars: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let arr = Array2D::from_rows(&chars).unwrap();

    let pattern = "XMAS";
    let directions: Vec<(isize, isize)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let result = word_search_pattern_amount(&arr, &pattern, &directions);
    println!("part 1: {}", result);

    let str_patterns = vec!["M.S.A.M.S", "M.M.A.S.S", "S.M.A.S.M", "S.S.A.M.M"];

    let result: u32 = str_patterns
        .into_iter()
        .map(|p| {
            p.chars().map(|c| match c {
                '.' => None,
                _ => Some(c),
            })
        })
        .map(|p| Array2D::from_iter_row_major(p, 3, 3).unwrap())
        .map(|p| word_search_pattern2d_amount(&arr, &p))
        .sum();

    println!("part 2: {}", result);
}

fn word_search_pattern_amount(
    arr: &Array2D<char>,
    pattern: &str,
    directions: &[(isize, isize)],
) -> u32 {
    let pattern_start = pattern.chars().next().unwrap();

    let mut result = 0;

    for (row, column) in arr.indices_row_major() {
        match arr.get(row, column) {
            Some(cell_char) if *cell_char == pattern_start => {}
            _ => continue,
        }

        'dir: for (d_row, d_column) in directions.iter() {
            let mut pattern_rest_iter = pattern.chars().skip(1);

            let mut scale = 1;

            while let Some(pattern_char) = pattern_rest_iter.next() {
                let (new_row, new_column) = match (
                    row.checked_add_signed(d_row * scale),
                    column.checked_add_signed(d_column * scale),
                ) {
                    (Some(r), Some(c)) => (r, c),
                    _ => continue 'dir,
                };

                match arr.get(new_row, new_column) {
                    Some(arr_char) if *arr_char == pattern_char => {}
                    _ => continue 'dir,
                }

                scale += 1;
            }

            result += 1;
        }
    }
    return result;
}

fn word_search_pattern2d_amount(arr: &Array2D<char>, pattern: &Array2D<Option<char>>) -> u32 {
    let mut result = 0;
    for row in 0..(arr.column_len() - pattern.column_len()) {
        'col: for column in 0..(arr.row_len() - pattern.row_len()) {
            for (pattern_row, pattern_column) in pattern.indices_row_major() {
                let pattern_char = match pattern.get(pattern_row, pattern_column).unwrap() {
                    None => continue,
                    Some(c) => *c,
                };

                let (arr_row, arr_column) = (row + pattern_row, column + pattern_column);
                if *arr.get(arr_row, arr_column).unwrap() != pattern_char {
                    continue 'col;
                }
            }

            result += 1;
        }
    }

    return result;
}
