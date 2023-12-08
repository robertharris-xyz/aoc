use std::fs;
use std::collections::HashMap;

fn sum_of_file(input: Vec<&str>, digit_hashmap: HashMap<&str, u8>) -> u32 {
    let mut result = 0;
    for line in input.into_iter() {
        let mut temp = vec![];
        for i in 0..line.len() {
            let digit = digit_hashmap
                .iter()
                .find(|(key, _)| line[i..].starts_with(*key));

            if let Some((_, value)) = digit {
                temp.push(value)
            }
        }

        let mut temp_iter = temp.into_iter();

        let first = *temp_iter.next().unwrap();
        let last = *temp_iter.next_back().unwrap_or(&first);

        let first_u32: u32 = first as u32;
        let last_u32: u32 = last as u32;

        result += first_u32 * 10 + last_u32;
    }
    result
}

fn part1(file_text: &str) -> Option<u32> {
    let file_input = parse_file(file_text);
    let digit_hashmap = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let result = sum_of_file(file_input, digit_hashmap);

    Some(result)
}

fn part2(file_text: &str) -> Option<u32> {
    let file_input = parse_file(file_text);
    let digit_hashmap = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);
    let result = sum_of_file(file_input, digit_hashmap);

    Some(result)
}

fn read_file(file_path: String) -> String {
    let f = fs::read_to_string(file_path);
    String::from(f.expect("could not open input file").trim_end())
}

fn parse_file(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn main() {
    let input = read_file("./data/input.txt".to_string());
    let sum_part1 = part1(&input);
    println!("part 1: {}", sum_part1.unwrap());
    let sum_part2 = part2(&input);
    println!("part 2: {}", sum_part2.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let file_text = read_file("./data/example1.txt".to_string());
        let sum = part1(&file_text);
        assert_eq!(sum.unwrap(), 142)
    }

    #[test]
    fn example2() {
        let file_text = read_file("./data/example2.txt".to_string());
        let sum = part2(&file_text);
        assert_eq!(sum.unwrap(), 281);
    }
}
