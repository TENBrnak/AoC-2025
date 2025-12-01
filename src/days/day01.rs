use std::io;
use std::fs::File;
use std::io::BufRead;

pub fn find_password(path: &str) -> i64 {
    let f = File::open(path).unwrap();

    let lines  = io::BufReader::new(f)
        .lines()
        .filter_map(|rot| {
            if let Ok(res) = rot {
                let n = &res[1..].parse::<i64>().unwrap();
                return if res.starts_with("R") { Some(*n) } else { Some(100-*n) }
            }
            None
        });

    let mut acc = 50;
    let mut passwrd = 0;
    for n in lines {
        acc += n;
        acc %= 100;
        if acc == 0 {
            passwrd += 1;
        }
    }
    passwrd
}

pub fn find_password_new_protocol(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    let lines  = io::BufReader::new(f)
        .lines()
        .filter_map(|rot| {
            if let Ok(res) = rot {
                let n = &res[1..].parse::<i64>().unwrap();
                return if res.starts_with("R") { Some(*n) } else { Some(-1 * *n) }
            }
            None
        });

    let mut acc = 50;
    let mut passwrd = 0;
    for n in lines {
        println!("{acc}");
        passwrd += (n / 100).abs();
        let eff_rot = n - (n / 100) * 100;

        if acc == 0 && eff_rot == 0 { continue; }
        let prev_acc = acc;
        acc += eff_rot;

        if 0 < acc && acc < 100 { continue; }
        if acc >= 100 {
            acc %= 100;
        } else if acc == 0 {
        } else if acc < 0 {
            acc = 100 + acc % 100;
        }

        if prev_acc != 0 {
            passwrd += 1;
        }
    }
    println!("{acc}");
    passwrd
}

#[cfg(test)]
mod tests {
    use crate::days::day01;
    use day01::{ find_password, find_password_new_protocol };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", find_password("inputs/01.in")), @"1139");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", find_password_new_protocol("inputs/01.in")), @"6684");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", find_password_new_protocol("inputs/01-example.in")), @"6");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", find_password("inputs/01-example.in")), @"3");
    }
}
