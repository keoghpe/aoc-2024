use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left = vec![];
    let mut right = vec![];

    if let Ok(lines) = read_lines("tests/day1.txt") {
        for line in lines {
            if let Ok(text) = line {
                // Split the line by whitespace and collect into a vector
                let parts: Vec<&str> = text.split_whitespace().collect();

                left.push(parts[0].parse::<i64>().unwrap());
                right.push(parts[1].parse::<i64>().unwrap());
            }
        }
    }

    left.sort();
    let right_counts = right.iter().fold(HashMap::new(), |mut counts, item| {
        *counts.entry(item).or_insert(0) += 1;
        counts
    });

    let mut sum = 0;

    for (i, l) in left.iter().enumerate() {
        match right_counts.get(l) {
            Some(c) => sum += l * c,
            None => (),
        }
    }

    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
