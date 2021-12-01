use std::fs;

pub fn run() {
    let depths = read_input();

    let depth_increases = part_one(&depths);
    println!("Depth increases: {}", depth_increases);

    let window_increases = part_two(&depths);
    println!("Window increases: {}", window_increases);
}

fn read_input() -> Vec<i32> {
    let filename = "src/day_1_input.txt";

    let day_1 = fs::read_to_string(filename).expect("Something went wrong reading the file");

    day_1
        .lines()
        .map(|my_string| my_string.parse::<i32>().unwrap())
        .collect()
}

fn part_one(depths: &Vec<i32>) -> i32 {
    let mut depths_iter = depths.iter().peekable();

    let mut increases = 0;

    while let Some(current_depth) = depths_iter.next() {
        if let Some(next_depth) = depths_iter.peek() {
            if current_depth < next_depth {
                increases += 1;
            }
        }
    }

    increases
}

fn part_two(depths: &Vec<i32>) -> i32 {
    let mut depths_iter = depths.iter().copied();

    let mut window: Vec<i32> = Vec::new();

    let mut increases = 0;

    let mut previous_sum = 0;

    while let Some(depth) = depths_iter.next() {
        if window.len() < 3 {
            window.push(depth);
            previous_sum = window.iter().sum();
        } else {
            window.remove(0);
            window.push(depth);

            let current_sum = window.iter().sum();

            if previous_sum < current_sum {
                increases += 1
            }

            previous_sum = current_sum;
        }
    }

    increases
}

#[test]
fn part_one_test() {
    let depths = read_input();

    assert_eq!(part_one(&depths), 1527);
}

#[test]
fn part_two_test() {
    let depths = read_input();

    assert_eq!(part_two(&depths), 1575);
}
