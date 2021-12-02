pub trait Solution {
    fn read_input(&self) -> Vec<i32>;
    fn part_one(&self, input: &Vec<i32>) -> i32;
    fn part_two(&self, input: &Vec<i32>) -> i32;
}

pub fn run<T: Solution>(solution: T) {
    let input = solution.read_input();

    let part_one_solution = solution.part_one(&input);
    println!("Part one solution: {}", part_one_solution);

    let part_two_solution = solution.part_two(&input);
    println!("Window increases: {}", part_two_solution);
}
