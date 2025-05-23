use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid: Vec<Vec<char>> = vec![vec![]];
    let mut total = 0;

    if let Ok(lines) = read_lines("tests/day4.txt") {
        for line in lines {
            grid.push(line.unwrap().chars().collect());
        }

        // iterate through the grid char by char
        // for each char, check all around it recurrsively for the next char - return true if you find a match
        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                total += count_word("XMAS".to_string(), &grid, i as i64, j as i64);
            }
        }
    }

    println!("{}", total)
}

fn count_word(word: String, grid: &Vec<Vec<char>>, i: i64, j: i64) -> i64 {
    let mut total = 0;

    // word.chars()
    // search up
    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i + offset as i64,
            j,
        )
    }) {
        total += 1;
    }
    // search down

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i - offset as i64,
            j,
        )
    }) {
        total += 1;
    }
    // search left

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i,
            j - offset as i64,
        )
    }) {
        total += 1;
    }
    // search right

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i,
            j + offset as i64,
        )
    }) {
        total += 1;
    }
    // up left

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i + offset as i64,
            j - offset as i64,
        )
    }) {
        total += 1;
    }
    // down left

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i - offset as i64,
            j + offset as i64,
        )
    }) {
        total += 1;
    }
    // up right

    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i + offset as i64,
            j + offset as i64,
        )
    }) {
        total += 1;
    }
    // down right
    if (0..word.len()).all(|offset| {
        is_char_at_index(
            word.chars().nth(offset).unwrap(),
            grid,
            i - offset as i64,
            j - offset as i64,
        )
    }) {
        total += 1;
    }

    total
}

fn is_char_at_index(letter: char, grid: &Vec<Vec<char>>, i: i64, j: i64) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    let ui = i as usize;
    let uj = j as usize;

    if ui < grid.len() {
        if uj < grid[ui].len() {
            return grid[ui][uj] == letter;
        }
    }

    return false;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::count_word;
    use crate::is_char_at_index;

    #[test]
    fn test_count_word() {
        assert_eq!(
            count_word(
                "S".to_string(),
                &vec![
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                ],
                1,
                1,
            ),
            1
        );

        assert_eq!(
            count_word(
                "S".to_string(),
                &vec![
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                ],
                2,
                2,
            ),
            1
        );

        assert_eq!(
            count_word(
                "S".to_string(),
                &vec![
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                    vec!['S', 'S', 'S'],
                ],
                0,
                0,
            ),
            1
        );

        assert_eq!(
            count_word(
                "XMAS".to_string(),
                &vec![
                    vec!['X', 'M', 'A'],
                    vec!['S', 'A', 'S'],
                    vec!['S', 'S', 'S'],
                ],
                0,
                0,
            ),
            6
        );
    }

    #[test]
    fn test_is_char_at_index() {
        assert!(is_char_at_index('S', &vec![vec!['S']], 0, 0));
        assert!(!is_char_at_index('S', &vec![vec!['S']], -1, 0));
        assert!(!is_char_at_index('S', &vec![vec!['S']], 1, 0));
        assert!(!is_char_at_index('S', &vec![vec!['S']], 0, -1));
        assert!(!is_char_at_index('S', &vec![vec!['S']], 0, 1));
        assert!(!is_char_at_index('R', &vec![vec!['S']], 0, 0));
    }
}
