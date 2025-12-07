use std::collections::{HashMap, HashSet};
use std::io;
use std::fs::File;
use std::io::{BufRead};

pub fn find_n_splits(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    let (_, n_splits) = io::BufReader::new(f)
        .lines()
        .filter_map(|l| {
            match l {
                Ok(l) => Some(l),
                Err(_) => None
            }
        })
        .fold((HashSet::new(), 0), |(mut beam_j, mut n_splits), line| {
            line.chars()
                .enumerate()
                .for_each(|(j, c)| {
                    match c {
                        '^' => {
                            if beam_j.contains(&j) {
                                beam_j.remove(&j);
                                beam_j.insert(j-1);
                                beam_j.insert(j+1);
                                n_splits += 1;
                            }
                        },
                        'S' => { beam_j.insert(j); },
                        _ => ()
                    }

                });

            (beam_j, n_splits)
        });
    n_splits
}

pub fn find_n_timelines(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    let timelines = io::BufReader::new(f)
        .lines()
        .filter_map(|l| {
            match l {
                Ok(l) => Some(l),
                Err(_) => None
            }
        })
        .fold(HashMap::new(), |mut beam_j, line| {
            line.chars()
                .enumerate()
                .for_each(|(j, c)| {
                    match c {
                        '^' => {
                            if beam_j.contains_key(&j) {
                                if let Some(val) = beam_j.remove(&j) {
                                    if let Some(prev_val) = beam_j.get(&(j-1)) {
                                        beam_j.insert(j-1, prev_val+val);
                                    } else { beam_j.insert(j-1, val); }

                                    if let Some(prev_val) = beam_j.get(&(j+1)) {
                                        beam_j.insert(j+1, prev_val+val);
                                    } else { beam_j.insert(j+1, val); }
                                }
                            }
                        },
                        'S' => { beam_j.insert(j, 1); },
                        _ => ()
                    }
                });
            beam_j
        });
    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day07;
    use day07::{ find_n_splits, find_n_timelines };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", find_n_splits("inputs/07/07-example.in")), @"21");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", find_n_splits("inputs/07/07.in")), @"1573");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", find_n_timelines("inputs/07/07-example.in")), @"40");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", find_n_timelines("inputs/07/07.in")), @"15093663987272");
    }
}
