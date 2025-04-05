// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

use std::{ops::Mul, thread::current};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Mul {
        left: Option<i64>,
        right: Option<i64>,
    },
    Do,
    Dont,
}

impl Operation {
    pub fn valid(&self) -> bool {
        match self {
            Self::Do => true,
            Self::Dont => true,
            Self::Mul {
                left: Some(_),
                right: Some(_),
            } => true,
            _ => false,
        }
    }
}

struct Parser<'a> {
    current: i64,
    tail: i64,
    current_operation_string: String,
    is_parsing_args: bool,
    number_buffer: String,
    current_operation: Option<Operation>,
    contents: &'a String,
    result: Vec<Operation>,
}

impl<'a> Parser<'a> {
    fn new(contents: &'a String) -> Self {
        Self {
            current: 0,
            tail: 0,
            current_operation_string: "".to_string(),
            is_parsing_args: false,
            number_buffer: "".to_string(),
            current_operation: None,
            contents: contents,
            result: vec![],
        }
    }

    fn advance_tail(&mut self) {
        self.tail += 1;
    }

    fn advance_current(&mut self) {
        self.current = self.current + 1;
        self.tail = self.current;
        self.current_operation_string = "".to_string();
        self.is_parsing_args = false;
        self.number_buffer = "".to_string();
        self.current_operation = None;
    }

    fn consume_operation(&mut self) {
        self.current = self.tail + 1;
        self.tail = self.current;
        self.current_operation_string = "".to_string();
        self.is_parsing_args = false;
        self.number_buffer = "".to_string();
        match self.current_operation {
            Some(operation) => {
                self.result.push(operation);
            }
            None => (),
        }
        self.current_operation = None;
    }

    fn current_char(&self) -> Option<char> {
        self.contents.chars().nth(self.tail as usize)
    }

    fn parse(&mut self) -> Vec<Operation> {
        loop {
            // println!("{}", self.current_operation_string);
            // println!("{:?}", self.current_operation);

            match self.current_char() {
                Some(c) => {
                    match self.current_operation {
                        None => {
                            self.current_operation_string.push(c);

                            match self.current_operation_string.as_str() {
                                // use a paren to distinguish from don't
                                "do(" => {
                                    self.current_operation = Some(Operation::Do);
                                    // self.advance_tail();
                                }
                                "don't" => {
                                    self.current_operation = Some(Operation::Dont);
                                    self.advance_tail();
                                }
                                "mul" => {
                                    self.current_operation = Some(Operation::Mul {
                                        left: None,
                                        right: None,
                                    });
                                    self.advance_tail();
                                }
                                other => {
                                    // There's definitely a more optimal way of detecting a valid command
                                    if other.len() > 4 {
                                        // not a valid command - reset
                                        self.advance_current();
                                    } else {
                                        self.advance_tail();
                                    }
                                }
                            }
                        }
                        Some(operation) => {
                            match c {
                                '(' => {
                                    if !self.is_parsing_args {
                                        self.is_parsing_args = true;
                                        self.advance_tail();
                                    } else {
                                        self.advance_current();
                                    }
                                }
                                ')' => {
                                    match operation {
                                        Operation::Mul {
                                            left: Some(lval),
                                            right: None,
                                        } => {
                                            if self.number_buffer.len() > 0 {
                                                self.current_operation = Some(Operation::Mul {
                                                    left: Some(lval),
                                                    right: Some(
                                                        self.number_buffer.parse().unwrap(),
                                                    ),
                                                });
                                            }
                                        }
                                        _ => (),
                                    }

                                    if self.current_operation.unwrap().valid()
                                        && self.is_parsing_args
                                    {
                                        self.consume_operation();
                                    } else {
                                        self.advance_current();
                                    }
                                }
                                ',' => {
                                    // for mul command - push the first number - if there's none, it's invalid
                                    // invalid otherwise
                                    match operation {
                                        Operation::Mul {
                                            left: None,
                                            right: None,
                                        } => {
                                            if self.number_buffer.len() > 0 {
                                                self.current_operation = Some(Operation::Mul {
                                                    left: Some(self.number_buffer.parse().unwrap()),
                                                    right: None,
                                                });
                                                self.number_buffer = "".to_string();
                                                self.advance_tail();
                                            } else {
                                                self.advance_current();
                                            }
                                        }
                                        _ => {
                                            // invalid
                                            self.advance_current();
                                        }
                                    }
                                }
                                '0'..='9' => {
                                    // add to buffer if less than 3 & valid command
                                    if self.number_buffer.len() < 3 {
                                        self.number_buffer.push(c);
                                        self.advance_tail();
                                    } else {
                                        // invalid
                                        self.advance_current();
                                    }
                                }
                                _ => {
                                    // invalid
                                    self.advance_current();
                                }
                            }
                        }
                    }
                }
                None => break,
            }
        }

        self.result.clone()
    }
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
    Parser::new(contents).parse()
}

fn process_instructions(instructions: &Vec<Operation>) -> i64 {
    let mut should_process = true;

    instructions
        .iter()
        .map(|operation| match operation {
            Operation::Mul {
                left: Some(left),
                right: Some(right),
            } => {
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
            _ => {
                panic!("invalid operation detected")
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
            process_instructions(&vec![Operation::Mul {
                left: Some(2),
                right: Some(2)
            }])
        );
        assert_eq!(
            10,
            process_instructions(&vec![
                Operation::Mul {
                    left: Some(2),
                    right: Some(2)
                },
                Operation::Mul {
                    left: Some(2),
                    right: Some(2)
                },
                Operation::Mul {
                    left: Some(1),
                    right: Some(2)
                }
            ])
        );

        assert_eq!(
            6,
            process_instructions(&vec![
                Operation::Mul {
                    left: Some(2),
                    right: Some(2)
                },
                Operation::Dont,
                Operation::Mul {
                    left: Some(2),
                    right: Some(2)
                },
                Operation::Do,
                Operation::Mul {
                    left: Some(1),
                    right: Some(2)
                }
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
            vec![Operation::Mul {
                left: Some(2),
                right: Some(4)
            }],
            parse_corrupted_memory(&"mul(2,4)".to_string())
        );

        assert_eq!(
            vec![Operation::Mul {
                left: Some(2),
                right: Some(4)
            }],
            parse_corrupted_memory(&"mulmul(2,4)".to_string())
        );

        assert_eq!(
            vec![
                Operation::Mul {
                    left: Some(2),
                    right: Some(4)
                },
                Operation::Mul {
                    left: Some(2),
                    right: Some(4)
                }
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
                Operation::Mul {
                    left: Some(2),
                    right: Some(4)
                },
                Operation::Mul {
                    left: Some(5),
                    right: Some(5)
                },
                Operation::Mul {
                    left: Some(11),
                    right: Some(8)
                },
                Operation::Mul {
                    left: Some(8),
                    right: Some(5)
                }
            ],
            parse_corrupted_memory(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            )
        );

        assert_eq!(
            vec![Operation::Mul { left:Some(815), right: Some(266)}, Operation::Mul { left:Some(392), right: Some(42)}, Operation::Mul { left:Some(640), right: Some(124)}, Operation::Mul { left:Some(96), right: Some(4)}, Operation::Mul { left:Some(371), right: Some(890)}],
            parse_corrupted_memory(
                &"mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}"
                    .to_string()
            )
        );

        assert_eq!(
            vec![
                Operation::Mul {
                    left: Some(2),
                    right: Some(4),
                },
                Operation::Dont,
                Operation::Mul {
                    left: Some(5),
                    right: Some(5)
                },
                Operation::Mul {
                    left: Some(11),
                    right: Some(8)
                },
                Operation::Do,
                Operation::Mul {
                    left: Some(8),
                    right: Some(5)
                },
            ],
            parse_corrupted_memory(
                &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
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

        assert_eq!(
            48,
            process_contents(
                &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            )
        );
    }
}
