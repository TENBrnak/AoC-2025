use std::io;
use std::fs::File;
use std::io::{BufRead};

pub fn solve_math(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|l| {
            match l {
                Ok(l) => Some(l),
                Err(_) => None
            }
        })
        .fold(Vec::new(), |mut table: Vec<Vec<i64>>, line| {
            line.split(" ")
                .filter(|&split| {
                    !split.eq("")
                })
                .enumerate()
                .for_each(|(i, c)| {
                    if let Some(v) = table.get_mut(i) {
                        if c.eq("+") {
                            *v = vec![v.iter().sum()]
                        } else if c.eq("*") {
                            *v = vec![v.iter().product()]
                        } else if let Ok(num) = c.parse::<i64>(){
                            v.push(num);
                        }
                    } else {
                        if let Ok(num) = c.parse::<i64>(){
                            table.push(vec![num])
                        }
                    }
                });
            table
        })
        .iter()
        .filter_map(|v| { v.first() })
        .sum()
}



pub fn solve_math_col_by_col(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|l| {
            match l {
                Ok(l) => Some(l),
                Err(_) => None
            }
        })
        .fold(Vec::new(), |mut table: Vec<Vec<char>>, line| {
            line.chars()
                .into_iter()
                .enumerate()
                .for_each(|(i, c)| {
                    if let Some(v) = table.get_mut(i) {
                        v.push(c);
                    } else {
                        table.push(vec![c])
                    }
                });
            table
        })
        .split(|v| {
            v.iter().all(|c| { c.eq(&' ') })
        })
        .map(|exercise| {
            let sign = *exercise[0].last().unwrap();
            let numbers = exercise
                .iter()
                .enumerate()
                .filter_map(|(i, col)| {
                    let col_string = col.iter().collect::<String>();
                    let no_sign = if i == 0 {
                        &col_string[..col_string.len()-1]
                    } else {
                        &col_string
                    }.trim().parse::<i64>();
                    if let Ok(n) = no_sign {
                        return Some(n);
                    }
                    None
                });
            return if sign == '+' { numbers.sum::<i64>() } else { numbers.product::<i64>() }
        }).sum::<i64>()
}


#[cfg(test)]
mod tests {
    use crate::days::day06;
    use day06::{ solve_math, solve_math_col_by_col };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", solve_math("inputs/06/06-example.in")), @"4277556");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", solve_math("inputs/06/06.in")), @"6209956042374");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", solve_math_col_by_col("inputs/06/06-example.in")), @"3263827");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", solve_math_col_by_col("inputs/06/06.in")), @"12608160008022");
    }
}
