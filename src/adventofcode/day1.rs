use std::fs;

fn load_data() -> Vec<i32> {
    let contents = fs::read_to_string("./src/adventofcode/day_1/input.txt")
        .expect("Something went wrong reading the file");

    return contents.split("\n").map(|s| s.parse().unwrap()).collect();
}

#[allow(dead_code)]
pub fn task_1() {
    let numbers = load_data();
    let mut counter = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            counter += 1;
        }
    }
    println!("first: {}", counter);
}

#[allow(dead_code)]
pub fn task_2() {
    let numbers = load_data();
    let mut counter = 0;
    for i in 3..numbers.len() {
        if numbers[i - 3] + numbers[i - 2] + numbers[i - 1]
            < numbers[i] + numbers[i - 1] + numbers[i - 2]
        {
            counter += 1;
        }
    }
    println!("second: {}", counter);
}
