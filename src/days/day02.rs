use std::io;
use std::fs::File;

pub fn get_invalid_id_sum(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::read_to_string(f).unwrap()
        .split(",")
        .filter_map(|id_range| {
            let mut range_bounds = id_range.split("-");
            if let (Some(start), Some(end)) = (range_bounds.next(), range_bounds.next()) {
                if let (Ok(start), Ok(end)) = (start.parse::<i64>(), end.parse::<i64>()) {
                    return Some(start..=end)
                }
            }
            None
        })
        .flatten()
        .filter_map(|id| {
            if is_invalid_id(format!("{}", id)) {
                return Some(id);
            }
            None
        })
        .sum()
}

pub fn is_invalid_id(id: String) -> bool {
    if id.len() % 2 == 1 {
        return false;
    }
    id.starts_with(&id[(id.len()/2)..])
}

pub fn get_new_invalid_id_sum(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    io::read_to_string(f).unwrap()
        .split(",")
        .filter_map(|id_range| {
            let mut range_bounds = id_range.split("-");
            if let (Some(start), Some(end)) = (range_bounds.next(), range_bounds.next()) {
                if let (Ok(start), Ok(end)) = (start.parse::<i64>(), end.parse::<i64>()) {
                    return Some(start..=end)
                }
            }
            None
        })
        .flatten()
        .filter_map(|id| {
            if is_new_invalid_id(format!("{}", id)) {
                return Some(id);
            }
            None
        })
        .sum()
}

pub fn is_new_invalid_id(id: String) -> bool {
    (0..(id.len()/2+1)).map(|split_n| {
        &id[..split_n]
    }).map(|prefix| {
        id.trim_start_matches(prefix).is_empty()
    }).any(|repeating| repeating)
}


#[cfg(test)]
mod tests {
    use crate::days::day02;
    use day02::{ get_invalid_id_sum, is_invalid_id, get_new_invalid_id_sum, is_new_invalid_id };
    use insta;

    #[test]
    fn test1() {
        insta::assert_snapshot!(format!("{}", get_invalid_id_sum("inputs/02/02-example.in")), @"1227775554");
    }

    #[test]
    fn test_id_validity() {
        insta::assert_snapshot!(format!("{}", is_invalid_id("38593859".to_string())), @"true");
    }

    #[test]
    fn test2() {
        insta::assert_snapshot!(format!("{}", get_invalid_id_sum("inputs/02/02.in")), @"31000881061");
    }

    #[test]
    fn test3() {
        insta::assert_snapshot!(format!("{}", get_new_invalid_id_sum("inputs/02/02-example.in")), @"4174379265");
    }

    #[test]
    fn test4() {
        insta::assert_snapshot!(format!("{}", get_new_invalid_id_sum("inputs/02/02.in")), @"46769308485");
    }
    #[test]
    fn playground() {
        insta::assert_snapshot!(format!("{}", is_new_invalid_id("2121212121".to_string())), @"true");
    }
}
