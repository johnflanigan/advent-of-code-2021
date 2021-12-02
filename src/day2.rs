use crate::utils::Solution;
use std::fs;

pub struct Day2;

pub struct Command {
    direction: Direction,
    distance: i32,
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl Solution<Command> for Day2 {
    fn read_input(&self) -> std::vec::Vec<Command> {
        let filename = "src/day2_input.txt";

        let day2 = fs::read_to_string(filename).expect("Something went wrong reading the file");

        day2.lines()
            .map(|string| {
                let v: Vec<&str> = string.split(' ').collect();

                let direction = match v[0] {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!("Unknown direction"),
                };

                let distance = v[1].parse::<i32>().unwrap();

                Command {
                    direction,
                    distance,
                }
            })
            .collect()
    }

    fn part_one(&self, commands: &std::vec::Vec<Command>) -> i32 {
        let mut horizontal = 0;
        let mut vertical = 0;

        for command in commands {
            match command.direction {
                Direction::Forward => horizontal += command.distance,
                Direction::Up => vertical -= command.distance,
                Direction::Down => vertical += command.distance,
            }
        }

        horizontal * vertical
    }

    fn part_two(&self, commands: &std::vec::Vec<Command>) -> i32 {
        let mut aim = 0;
        let mut horizontal = 0;
        let mut depth = 0;

        for command in commands {
            match command.direction {
                Direction::Forward => {
                    horizontal += command.distance;
                    depth += aim * command.distance;
                }
                Direction::Up => aim -= command.distance,
                Direction::Down => aim += command.distance,
            }
        }

        horizontal * depth
    }
}

#[test]
fn part_one_test() {
    let day2 = Day2;

    let input = day2.read_input();

    assert_eq!(day2.part_one(&input), 2073315);
}

#[test]
fn part_two_test() {
    let day2 = Day2;

    let input = day2.read_input();

    assert_eq!(day2.part_two(&input), 1840311528);
}
