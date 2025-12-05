use std::io;
use std::fs::File;
use std::io::BufRead;

pub fn make_table(path: &str) -> Vec<Vec<i32>> {
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                return Some(line);
            }
            None
        })
        .map(|line| {
            line.chars().map(|char| {
                if char == '@' {
                    return 1;
                }
                0
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
}

pub fn get_n_accessible_rolls(path: &str) -> i64 {
    let table = make_table(path);
    table
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line
                .iter()
                .enumerate()
                .map(|(j, elem)| {
                    if elem != &1 { return 0 }
                    if get_sum_around(&table, i as i32, j as i32) < 4 { 1 } else { 0 }
                }).sum::<i64>()
        })
        .sum()
}

pub fn can_remove_rolls(path: &str) -> i64 {
    let mut table = make_table(path);

    let mut n_removed_rolls = 0;
    loop {
        let mut next_table = table.clone();
        let n_accessible_rolls = table.iter()
            .enumerate()
            .map(|(i, line)| {
                line
                    .iter()
                    .enumerate()
                    .map(|(j, elem)| {
                        if elem != &1 { return 0 }
                        if get_sum_around(&table, i as i32, j as i32) < 4 {
                            next_table[i][j] = 0;
                            n_removed_rolls += 1;
                            1
                        } else {
                            0
                        }
                    }).sum::<i64>()
            })
            .sum::<i64>();
        table = next_table;
        if n_accessible_rolls == 0 { break; }
    };
    n_removed_rolls
}

pub fn get_sum_around(table: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let n_rows = table.len();
    let n_cols = table.get(i as usize).expect("No column??").len();
    let mut sum_around = 0;

    for k in i-1..=i+1 {
        if 0 <= k && k < n_rows as i32 {
            if j + 1 < n_cols as i32 {
                sum_around += table[k as usize][j as usize + 1];
            }
            if j - 1 >= 0 {
                sum_around += table[k as usize][j as usize - 1];
            }
            if k != i {
                sum_around += table[k as usize][j as usize];
            }
        }
    }
    sum_around
}

#[cfg(test)]
mod tests {
    use crate::days::day04;
    use day04::{ get_n_accessible_rolls, can_remove_rolls };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", get_n_accessible_rolls("inputs/04/04-example.in")), @"13");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", get_n_accessible_rolls("inputs/04/04.in")), @"1474");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", can_remove_rolls("inputs/04/04-example.in")), @"43");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", can_remove_rolls("inputs/04/04.in")), @"8910");
    }
}
