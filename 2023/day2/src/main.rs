use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use fancy_regex::Regex;

fn part1(file_path: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum = 0;

    let lines = read_lines(file_path).unwrap();

    for l in lines {
        let line = l.unwrap();

        // Regex to get game ID
        let game_id_regex = Regex::new(r"(?!Game )\d+(?=:)").unwrap();
        let game_id: u32 = game_id_regex
            .find(&line)
            .unwrap()
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let mut is_impossible: bool = false;

        for round in line.split(";") {
            for mut cube in round.split(",") {
                let game_and_colour_regex =
                    Regex::new(r"(?!Game \d+: )\d+ (blue|green|red)").unwrap();
                if game_and_colour_regex.is_match(cube).unwrap() {
                    cube = game_and_colour_regex.find(&cube).unwrap().unwrap().as_str();
                }
                let mut cube_split = cube.split(" ");
                let cube_num: u32 = cube_split.next().unwrap().parse::<u32>().unwrap();
                let cube_colour: &str = cube_split.next().unwrap();

                match cube_colour {
                    "red" => {
                        if cube_num > max_red {
                            is_impossible = true;
                            break;
                        }
                    }
                    "green" => {
                        if cube_num > max_green {
                            is_impossible = true;
                            break;
                        }
                    }
                    "blue" => {
                        if cube_num > max_blue {
                            is_impossible = true;
                            break;
                        }
                    }
                    _ => panic!("Unrecognised cube: {}", cube),
                };
            }
            if is_impossible {
                break;
            }
        }
        if !is_impossible {
            sum += game_id;
        }
    }
    sum
}

fn part2(file_path: &str) -> u32 {
    let mut sum = 0;

    let lines = read_lines(file_path).unwrap();

    for l in lines {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let line = l.unwrap();

        for round in line.split(";") {
            for mut cube in round.split(",") {
                let game_and_colour_regex =
                    Regex::new(r"(?!Game \d+: )\d+ (blue|green|red)").unwrap();
                if game_and_colour_regex.is_match(cube).unwrap() {
                    cube = game_and_colour_regex.find(&cube).unwrap().unwrap().as_str();
                }
                let mut cube_split = cube.split(" ");
                let cube_num: u32 = cube_split.next().unwrap().parse::<u32>().unwrap();
                let cube_colour: &str = cube_split.next().unwrap();

                match cube_colour {
                    "red" => {
                        if cube_num > min_red {
                            min_red = cube_num;
                        }
                    }
                    "green" => {
                        if cube_num > min_green {
                            min_green = cube_num;
                        }
                    }
                    "blue" => {
                        if cube_num > min_blue {
                            min_blue = cube_num;
                        }
                    }
                    _ => panic!("Unrecognised cube: {}", cube),
                };
            }
        }

        sum += min_red * min_green * min_blue;
    }
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_file_path = "./data/input.txt";
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
        assert_eq!(sum, 8);
    }

    #[test]
    fn example2() {
        let sum = part2("./data/example1.txt");
        assert_eq!(sum, 2286);
    }
}
