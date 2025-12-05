use std::cmp::{max, min};
use std::io;
use std::fs::File;
use std::io::{BufRead};

pub fn build_range(path: &str) -> Vec<(i64, i64)> {
    let f = File::open(path).unwrap();
    let mut ranges = io::BufReader::new(f)
        .lines()
        .filter_map(|expire_range| {
            if let Ok(expire_range) = expire_range {
                let mut expires = expire_range.split("-");
                if let (Some(start_range), Some(end_range)) = (expires.next(), expires.next()) {
                    if let (Ok(start_range), Ok(end_range)) = (start_range.parse::<i64>(), end_range.parse::<i64>()) {
                        return Some((start_range, end_range));
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    ranges.sort();
    ranges
}

pub fn get_n_fresh(path: &str) -> i64 {
    let range = build_range(path);
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|line| {
            if let Ok(item) = line.unwrap().parse::<i64>() {
                return Some(item);
            }
            None
        })
        .map(|food_item| {
            if range
                .iter()
                .map(|range| (range.0..=range.1).contains(&food_item))
                .any(|fresh| fresh)
            { 1 } else { 0 }
        })
        .sum()
}

pub fn get_total_fresh(path: &str) -> i64 {
    let ranges = build_range(path);
    ranges
        .into_iter()
        .fold(Vec::new(), |mut out_ranges: Vec<(i64, i64)>, range| {
            match out_ranges.last_mut() {
                Some(last) if last.1 >= range.0 || last.0 == range.0 => *last = (min(last.0, range.0), max(last.1, range.1)),
                _ => out_ranges.push(range)
            }
            out_ranges
        })
        .iter()
        .map(|range| {
           range.1 - range.0 + 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day05;
    use day05::{ get_n_fresh, get_total_fresh };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", get_n_fresh("inputs/05/05-example.in")), @"3");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", get_n_fresh("inputs/05/05.in")), @"611");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", get_total_fresh("inputs/05/05-example.in")), @"14");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", get_total_fresh("inputs/05/05.in")), @"345995423801866");
    }
}
