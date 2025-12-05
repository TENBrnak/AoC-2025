use std::io;
use std::fs::File;
use std::io::{BufRead};

pub fn get_joltage_2_sum(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|bank| {
            if let Ok(bank) = bank {
                let (largest, following) = &bank[..bank.len()-1].chars()
                    .into_iter()
                    .filter_map(|char_digit| {
                        char_digit.to_digit(10)
                    })
                    .fold((0, 0), |(mut largest, mut following), next| {
                        if next > largest {
                            largest = next;
                            following = 0;
                        } else if next > following {
                            following = next;
                        }
                        (largest, following)
                    });
                let last = &bank[bank.len()-1..].parse::<u32>().unwrap();
                let following = if following.lt(last) { last } else { following };

                if let Ok(joltage) = format!("{}{}", largest, following).parse::<i64>() {
                    return Some(joltage)
                }
            }
            None
        }).sum()
}

pub fn get_joltage_12_sum(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::BufReader::new(f)
        .lines()
        .filter_map(|bank| {
            if let Ok(bank) = bank {
                let (_, out) =  (0..12)
                    .fold((0, [0; 12]), |(starting_pos, mut out), i| {
                        let (maximum, max_index) = bank[starting_pos..=bank.len()-12+i]
                            .chars()
                            .filter_map(|digit| {
                                digit.to_digit(10)
                            })
                            .enumerate()
                            .fold((0, 0), |(maximum, max_index), (i, next)| {
                                if maximum < next {
                                    return (next, starting_pos + i)
                                }
                                (maximum, max_index)
                            });
                        out[i] = maximum;
                        (max_index + 1, out)
                    });

                if let Ok(joltage) = out.iter().map(|n| { n.to_string() }).collect::<Vec<_>>().join("").parse::<i64>() {
                    return Some(joltage)
                }
            }
            None
        }).sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day03;
    use day03::{ get_joltage_2_sum, get_joltage_12_sum };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", get_joltage_2_sum("inputs/03/03-example.in")), @"357");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", get_joltage_2_sum("inputs/03/03.in")), @"17166");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", get_joltage_12_sum("inputs/03/03-example.in")), @"3121910778619");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", get_joltage_12_sum("inputs/03/03.in")), @"169077317650774");
    }
}
