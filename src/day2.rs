use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut safe_count = 0;

    if let Ok(lines) = read_lines("tests/day2.txt") {
        'outer: for line in lines {
            if let Ok(text) = line {
                // Split the line by whitespace and collect into a vector
                let numbers: Vec<&str> = text.split_whitespace().collect();

                let mut previous = None;
                let mut previous_positive = None;

                for num in numbers {
                    match previous {
                        Some(previous_value) => {
                            let current = num.parse::<i64>().unwrap();

                            previous = Some(current);

                            // first time - determine if we are ascending or descending
                            // The levels are either all increasing or all decreasing.
                            // Any two adjacent levels differ by at least one and at most three.
                            let difference = current - previous_value;

                            if difference.abs() > 0 && difference.abs() <= 3 {
                                // it's ok
                                match previous_positive {
                                    Some(is_positive) => {
                                        if (is_positive && difference > 0)
                                            || (!is_positive && difference < 0)
                                        {
                                            // keep looping
                                        } else {
                                            //
                                            continue 'outer;
                                        }
                                    }
                                    None => previous_positive = Some(difference > 0),
                                }
                            } else {
                                continue 'outer;
                            }
                        }
                        None => {
                            previous = Some(num.parse::<i64>().unwrap());
                        }
                    }
                }

                safe_count += 1;
            }
        }
    }

    println!("{}", safe_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
