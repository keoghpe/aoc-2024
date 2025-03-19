// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

fn main() {
    if let Ok(contents) = std::fs::read_to_string("tests/day3.txt") {
        println!("{}", process_contents(&contents))
    }
}

fn process_contents(contents: &String) -> i64 {
    let instructions = parse_corrupted_memory(contents);
    process_instructions(instructions)
}

fn parse_corrupted_memory(contents: &String) -> Vec<(i64, i64)> {
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
                        result.push((left_parsed, right_parsed));
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

fn process_instructions(instructions: Vec<(i64, i64)>) -> i64 {
    instructions
        .iter()
        .map(|&(insta, instb)| insta * instb)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse_corrupted_memory, process_contents};

    #[test]
    fn test_process_instructions() {
        use super::*;
        assert_eq!(0, process_instructions(vec![]));
        assert_eq!(4, process_instructions(vec![(2, 2)]));
        assert_eq!(10, process_instructions(vec![(2, 2), (2, 2), (1, 2)]));
    }
    #[test]
    fn test_parse_corrupted_memory() {
        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"".to_string())
        // );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"sometotallyrandomnonsense".to_string())
        // );

        // assert_eq!(
        //     vec![(2, 4)],
        //     parse_corrupted_memory(&"mul(2,4)".to_string())
        // );

        // assert_eq!(
        //     vec![(2, 4), (2, 4)],
        //     parse_corrupted_memory(&"mul(2,4)mul(2,4)".to_string())
        // );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"mul(4*".to_string())
        // );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"mul(6,9!".to_string())
        // );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"?(12,34)".to_string())
        // );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"mul ( 2 , 4 )".to_string())
        // );

        // assert_eq!(
        //     vec![(2, 4), (5, 5), (11, 8), (8, 5)],
        //     parse_corrupted_memory(
        //         &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        //             .to_string()
        //     )
        // );

        assert_eq!(
            vec![(815, 266), (392, 42), (640, 124), (96, 4), (371, 890)],
            parse_corrupted_memory(
                &"mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}"
                    .to_string()
            )
        );

        // assert_eq!(
        //     Vec::<(i64, i64)>::new(),
        //     parse_corrupted_memory(&"mul(200,)".to_string())
        // );
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
