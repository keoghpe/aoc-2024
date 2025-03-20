// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

use std::ops::Mul;

#[derive(Debug, PartialEq)]
enum Operation {
    Mul { left: i64, right: i64 },
    Do,
    Dont,
}

fn main() {
    if let Ok(contents) = std::fs::read_to_string("tests/day3.txt") {
        println!("{}", process_contents(&contents))
    }
}

fn process_contents(contents: &String) -> i64 {
    let instructions = parse_corrupted_memory(contents);
    process_instructions(&instructions)
}

fn parse_corrupted_memory(contents: &String) -> Vec<Operation> {
    // scan until you get to mul(
    // scan 1-3 digits
    // scan a comma
    // scan 1-3 digits
    // scan )
    // add digits to vec
    // TODO extract a struct and a function to reset the state
    let mut consumed_chars = 0;
    let mut left = "".to_string();
    let mut right = "".to_string();
    let mut parsed_comma = false;

    // let reset_state = || {
    //     consumed_chars = 0;
    //     left = "".to_string();
    //     right = "".to_string();
    //     parsed_comma = false;
    // };

    let mut result = vec![];

    for (_, current_char) in contents.chars().enumerate() {
        // println!(
        //     "chars consumed: {}, current: {}, left: {}, right: {}, parsed_comma: {}",
        //     consumed_chars, current_char, left, right, parsed_comma
        // );

        match (consumed_chars, current_char) {
            (0, 'm') | (1, 'u') | (2, 'l') | (3, '(') => {
                consumed_chars += 1;
            }
            // mul(123,123)
            (4..=10, '0'..='9') => {
                // technically, this allows numbers with leading 0s
                // we could disallow this by checking that the left or right is empty before appending

                if !parsed_comma {
                    if left.len() < 3 {
                        left.push_str(&current_char.to_string());
                        consumed_chars += 1;
                    } else {
                        // println!("BREAKING LEFT");
                        // break
                        consumed_chars = 0;
                        left = "".to_string();
                        right = "".to_string();
                        parsed_comma = false;
                    }
                } else {
                    // println!("PARSED COMMA");
                    if right.len() < 3 {
                        println!("{} {}", consumed_chars, current_char);
                        right.push_str(&current_char.to_string());
                        consumed_chars += 1;
                    } else {
                        consumed_chars = 0;
                        left = "".to_string();
                        right = "".to_string();
                        parsed_comma = false;
                    }
                }
            }
            (5..=9, ',') => {
                // println!("PARSING COMMA");
                if !parsed_comma {
                    // println!("SETTING PARSED COMMA");
                    parsed_comma = true;
                    consumed_chars += 1;
                } else {
                    consumed_chars = 0;
                    left = "".to_string();
                    right = "".to_string();
                    parsed_comma = false;
                }
            }
            (6..=12, ')') => {
                // we have successfully parsed an instruction
                // append to the vector

                match (left.parse::<i64>(), right.parse::<i64>()) {
                    (Ok(left_parsed), Ok(right_parsed)) => {
                        println!("push to result");
                        result.push(Operation::Mul {
                            left: left_parsed,
                            right: right_parsed,
                        });
                    }
                    (_, _) => {
                        println!("parsing error");
                    }
                }

                consumed_chars = 0;
                left = "".to_string();
                right = "".to_string();
                parsed_comma = false;
            }
            (_, _) => {
                consumed_chars = 0;
                left = "".to_string();
                right = "".to_string();
                parsed_comma = false;
            }
        }
    }

    result
}

fn process_instructions(instructions: &Vec<Operation>) -> i64 {
    let mut should_process = true;

    instructions
        .iter()
        .map(|operation| match operation {
            Operation::Mul { left, right } => {
                if should_process {
                    left * right
                } else {
                    0
                }
            }
            Operation::Do => {
                should_process = true;
                0
            }
            Operation::Dont => {
                should_process = false;
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse_corrupted_memory, process_contents, Operation};

    #[test]
    fn test_process_instructions() {
        use super::*;
        assert_eq!(0, process_instructions(&vec![]));
        assert_eq!(
            4,
            process_instructions(&vec![Operation::Mul { left: 2, right: 2 }])
        );
        assert_eq!(
            10,
            process_instructions(&vec![
                Operation::Mul { left: 2, right: 2 },
                Operation::Mul { left: 2, right: 2 },
                Operation::Mul { left: 1, right: 2 }
            ])
        );

        assert_eq!(
            6,
            process_instructions(&vec![
                Operation::Mul { left: 2, right: 2 },
                Operation::Dont,
                Operation::Mul { left: 2, right: 2 },
                Operation::Do,
                Operation::Mul { left: 1, right: 2 }
            ])
        );
    }
    #[test]
    fn test_parse_corrupted_memory() {
        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"sometotallyrandomnonsense".to_string())
        );

        assert_eq!(
            vec![Operation::Mul { left: 2, right: 4 }],
            parse_corrupted_memory(&"mul(2,4)".to_string())
        );

        assert_eq!(
            vec![Operation::Mul { left: 2, right: 4 }],
            parse_corrupted_memory(&"mulmul(2,4)".to_string())
        );

        assert_eq!(
            vec![
                Operation::Mul { left: 2, right: 4 },
                Operation::Mul { left: 2, right: 4 }
            ],
            parse_corrupted_memory(&"mul(2,4)mul(2,4)".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"mul(4*".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"mul(6,9!".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"?(12,34)".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"mul ( 2 , 4 )".to_string())
        );

        assert_eq!(
            Vec::<Operation>::new(),
            parse_corrupted_memory(&"mul(200,)".to_string())
        );

        assert_eq!(
            vec![
                Operation::Mul { left: 2, right: 4 },
                Operation::Mul { left: 5, right: 5 },
                Operation::Mul { left: 11, right: 8 },
                Operation::Mul { left: 8, right: 5 }
            ],
            parse_corrupted_memory(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            )
        );

        assert_eq!(
            vec![Operation::Mul { left:815, right: 266}, Operation::Mul { left:392, right: 42}, Operation::Mul { left:640, right: 124}, Operation::Mul { left:96, right: 4}, Operation::Mul { left:371, right: 890}],
            parse_corrupted_memory(
                &"mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_process_contents() {
        assert_eq!(
            161,
            process_contents(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            )
        );
    }
}
