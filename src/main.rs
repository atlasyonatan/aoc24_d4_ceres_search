use array2d::Array2D;
use std::{
    io::{self, Read},
    iter::repeat,
    vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("input:\n{}", &input);
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
    println!("{}", repeat('-').take(arr.row_len()).collect::<String>());

    let str_patterns = vec!["M.S.A.M.S", "M.M.A.S.S", "S.M.A.S.M", "S.S.A.M.M"];
    let patterns = str_patterns
        .into_iter()
        .map(|p| {
            p.chars().map(|c| match c {
                '.' => None,
                _ => Some(c),
            })
        })
        .map(|p| Array2D::from_iter_row_major(p, 3, 3).unwrap());

    let mut total_match_arr = Array2D::filled_with(None, arr.num_rows(), arr.num_columns());
    let result: u32 = patterns
        .inspect(|p| println!("pattern arr:\n{}", pattern_arr_to_str(&p)))
        .map(|p| word_search_pattern2d(&arr, &p))
        .inspect(|(match_arr, amount)| {
            println!("match arr:\n{}", pattern_arr_to_str(&match_arr));
            println!("amount: {}", amount);

            for (row, column) in arr.indices_row_major() {
                if let Some(c) = match_arr.get(row, column).unwrap() {
                    total_match_arr.set(row, column, Some(*c)).unwrap();
                }
            }
        })
        .map(|(_, amount)| amount)
        .sum();

    println!("total match arr:\n{}", pattern_arr_to_str(&total_match_arr));
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

fn word_search_pattern2d(
    arr: &Array2D<char>,
    pattern: &Array2D<Option<char>>,
) -> (Array2D<Option<char>>, u32) {
    let mut match_arr: Array2D<Option<char>> =
        Array2D::filled_with(None, arr.num_rows(), arr.num_columns());
    let mut matches_found = 0;

    for row in 0..=(arr.num_rows() - pattern.num_rows()) {
        'col: for column in 0..=(arr.num_columns() - pattern.num_columns()) {
            for (pattern_row, pattern_column) in pattern.indices_row_major() {
                let pattern_char = match pattern.get(pattern_row, pattern_column).unwrap() {
                    None => continue,
                    Some(c) => *c,
                };

                let (arr_row, arr_column) = (row + pattern_row, column + pattern_column);
                let arr_char = *arr.get(arr_row, arr_column).unwrap();
                if arr_char != pattern_char {
                    continue 'col;
                }
            }

            matches_found += 1;

            //record found pattern in match_arr
            for (pattern_row, pattern_column) in pattern.indices_row_major() {
                let (arr_row, arr_column) = (row + pattern_row, column + pattern_column);
                if let Some(c) = pattern.get(pattern_row, pattern_column).unwrap() {
                    match_arr.set(arr_row, arr_column, Some(*c)).unwrap();
                }
            }
        }
    }

    return (match_arr, matches_found);
}

fn pattern_arr_to_str(pattern_arr: &Array2D<Option<char>>) -> String {
    arr_to_str(pattern_arr, |cell| match cell {
        Some(c) => *c,
        None => '.',
    })
}

fn arr_to_str<T, F>(arr: &Array2D<T>, f: F) -> String
where
    F: Fn(&T) -> char,
{
    let mut s = String::new();

    for row in arr.rows_iter() {
        for cell in row {
            let c = f(cell);
            s.push(c);
        }
        s.push('\n');
    }

    return s;
}
