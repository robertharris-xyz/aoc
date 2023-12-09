use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(file_path: &str) -> u32 {
    todo!();
}

fn part2(file_path: &str) -> u32 {
    todo!();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_file_path = "./data/input.txt"

    let sum_part1 = part1(input_file_path);
    let sum_part2 = part2(input_file_path);
    
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let sum = part1("./data/example1.txt");
        assert_eq!(sum, 0);
    }

    #[test]
    fn example2() {
        let sum = part2("./data/example2.txt");
        assert_eq!(sum, 0);
    }
}
