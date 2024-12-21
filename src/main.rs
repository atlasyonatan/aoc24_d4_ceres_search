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

    let result = word_search_amount(arr, &pattern, &directions);
    println!("part 1: {}", result);
}

fn word_search_amount(arr: Array2D<char>, pattern: &str, directions: &[(isize, isize)]) -> u32 {
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
