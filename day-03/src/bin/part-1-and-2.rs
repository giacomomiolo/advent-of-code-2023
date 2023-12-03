use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use tracing::{info, error};
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    match get_matrix("input-2023-03.txt") {
        Ok(matrix) => {
            match sum_part_numbers(&matrix) {
                Ok(sum) => info!("Total sum of part numbers: {}", sum),
                Err(e) => error!("Error calculating part numbers: {}", e),
            }

            match sum_gear_ratios(&matrix) {
                Ok(gear_ratios_sum) => info!("Total sum of gear ratios: {}", gear_ratios_sum),
                Err(e) => error!("Error calculating gear ratios: {}", e),
            }
        },
        Err(e) => error!("Error reading matrix: {}", e),
    }

    Ok(())
}

fn sum_part_numbers(matrix: &[Vec<u8>]) -> Result<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut total = 0;
    let mut seen = HashSet::new();

    for i in 0..m {
        for j in 0..n {
            if is_symbol(matrix[i][j]) {
                for dir in [(-1, -1), (-1, 0), (-1, 1),
                            (0, -1), /* current cell */ (0, 1),
                            (1, -1), (1, 0), (1, 1)] {
                    if let Some((i2, j2)) = checked_index(i, j, dir, m, n) {
                        if matrix[i2][j2].is_ascii_digit() {
                            let (num, k, l) = grab_number(&matrix[i2], j2);
                            if seen.insert((i2, k, l)) {
                                total += num;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(total)
}

fn sum_gear_ratios(matrix: &[Vec<u8>]) -> Result<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut total = 0;
    let mut seen = HashSet::new();
    let mut nums = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == b'*' {
                nums.clear();
                for dir in [(-1, -1), (-1, 0), (-1, 1),
                            (0, -1), /* current cell */ (0, 1),
                            (1, -1), (1, 0), (1, 1)] {
                    if let Some((i2, j2)) = checked_index(i, j, dir, m, n) {
                        if matrix[i2][j2].is_ascii_digit() {
                            let (num, k, l) = grab_number(&matrix[i2], j2);
                            if seen.insert((i2, k, l)) {
                                nums.push(num);
                            }
                        }
                    }
                }
                if nums.len() == 2 {
                    total += nums[0] * nums[1];
                }
            }
        }
    }
    Ok(total)
}

fn get_matrix(file_path: &str) -> Result<Vec<Vec<u8>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let matrix = reader.lines()
                       .collect::<std::io::Result<Vec<_>>>()?
                       .into_iter()
                       .map(|s| Ok(s.into_bytes()))
                       .collect::<Result<Vec<Vec<u8>>>>()?;
    Ok(matrix)
}

fn checked_index(i: usize, j: usize, dir: (isize, isize), m: usize, n: usize) -> Option<(usize, usize)> {
    let (di, dj) = dir;
    let new_i = i as isize + di;
    let new_j = j as isize + dj;
    if new_i >= 0 && new_i < m as isize && new_j >= 0 && new_j < n as isize {
        Some((new_i as usize, new_j as usize))
    } else {
        None
    }
}

fn is_symbol(b: u8) -> bool {
    !(b == b'.' || b.is_ascii_digit())
}

fn grab_number(row: &[u8], index: usize) -> (i32, usize, usize) {
    let mut start = index;
    while start > 0 && row[start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = index + 1;
    while end < row.len() && row[end].is_ascii_digit() {
        end += 1;
    }
    let num_str = std::str::from_utf8(&row[start..end]).unwrap();
    let num = num_str.parse::<i32>().unwrap();
    (num, start, end)
}
