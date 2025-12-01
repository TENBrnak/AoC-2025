use std::io;
use std::fs::File;
use std::io::BufRead;

pub fn find_password(path: &str) -> i64 {
    let f = File::open(path).unwrap();

    let (_, pwd)  = io::BufReader::new(f)
        .lines()
        .filter_map(|rot| {
            if let Ok(res) = rot {
                let n = &res[1..].parse::<i64>().unwrap();
                return if res.starts_with("R") { Some(*n) } else { Some(100-*n) }
            }
            None
        })
        .fold((50, 0),|(acc, pwd), next| {
            let acc = (acc + next) % 100;
            return match acc {
                0 => (acc, pwd + 1),
                _ => (acc, pwd)
            }
        });
    pwd
}

pub fn find_password_new_protocol(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    let (_, pwd)  = io::BufReader::new(f)
        .lines()
        .filter_map(|rot| {
            if let Ok(res) = rot {
                let n = &res[1..].parse::<i64>().unwrap();
                return if res.starts_with("R") { Some(*n) } else { Some(-1 * *n) }
            }
            None
        })
        .fold((50, 0), |(acc, pwd), next| {
            let mut pwd = pwd + (next / 100).abs();
            let eff_rot = next - (next / 100) * 100;

            if (acc != 0) && (acc + eff_rot <= 0 ||  acc + eff_rot >= 100) { pwd += 1; }

            let mut acc = (acc + eff_rot) % 100;
            if acc < 0 { acc += 100; }

            (acc, pwd)
        });

    pwd
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
