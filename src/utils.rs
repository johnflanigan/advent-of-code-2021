pub trait Solution<T> {
    fn read_input(&self) -> Vec<T>;
    fn part_one(&self, input: &Vec<T>) -> i32;
    fn part_two(&self, input: &Vec<T>) -> i32;
}

pub fn run<T, D: Solution<T>>(solution: D) {
    let input = solution.read_input();

    let part_one_solution = solution.part_one(&input);
    println!("Part one solution: {}", part_one_solution);

    let part_two_solution = solution.part_two(&input);
    println!("Part two solution: {}", part_two_solution);
}
