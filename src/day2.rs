use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut safe_count = 0;

    if let Ok(lines) = read_lines("tests/day2.txt") {
        for line in lines {
            if let Ok(text) = line {
                let numbers: Vec<i64> = text
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();

                if is_safe_with_damper(&numbers) {
                    safe_count += 1;
                }
            }
        }
    }
}

fn is_safe_with_damper(numbers: &Vec<i64>) -> bool {
    let total = numbers.len();

    // This works, but is a really hacky solution
    // Ideally, I would detect correctly the outlier
    for i in 0..total {
        println!("{}", i);

        let mut new_vec = numbers.to_vec();
        new_vec.remove(i);

        if is_safe(&new_vec) {
            return true;
        }
    }
    false
}

fn is_safe(numbers: &Vec<i64>) -> bool {
    let mut previous = None;
    let mut previous_is_positive = None;

    for current in numbers {
        match previous {
            Some(previous_value) => {
                previous = Some(current);

                // first time - determine if we are ascending or descending
                // The levels are either all increasing or all decreasing.
                // Any two adjacent levels differ by at least one and at most three.
                let difference = current - previous_value;

                if difference.abs() > 0 && difference.abs() <= 3 {
                    // it's ok
                    match previous_is_positive {
                        Some(previous_positive) => {
                            if (previous_positive && difference > 0)
                                || (!previous_positive && difference < 0)
                            {
                                // keep looping
                            } else {
                                return false;
                            }
                        }
                        None => previous_is_positive = Some(difference > 0),
                    }
                } else {
                    return false;
                }
            }
            None => {
                previous = Some(current);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_safe() {
        use super::*;
        // Your test code here
        // 1 1 1 1
        assert!(is_safe(&vec![1, 2, 3, 4, 5]));
        // -1 -1 -1 -1
        assert!(is_safe(&vec![5, 4, 3, 2, 1]));
        //
        assert!(!is_safe(&vec![1, 5, 10, 20, 100]));
        // -4 1 1 1
        assert!(is_safe_with_damper(&vec![5, 1, 2, 3, 4]));
        // 4 -3 1 1
        assert!(is_safe_with_damper(&vec![1, 5, 2, 3, 4]));
        // 2 -2 4 1
        assert!(is_safe_with_damper(&vec![1, 3, 1, 5, 6]));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
