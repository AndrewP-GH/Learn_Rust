use std::fs;

fn load_data() -> Vec<(String, i32)> {
    let contents = fs::read_to_string("./src/adventofcode/day_2/input.txt")
        .expect("Something went wrong reading the file");

    return contents
        .split("\n")
        .map(|s| {
            let parts = s.split(" ").collect::<Vec<_>>();
            (String::from(parts[0]), parts[1].parse().unwrap())
        })
        .collect();
}

#[allow(dead_code)]
pub fn task_1() {
    let commands = load_data();
    let mut coords = (0, 0);
    for command in commands {
        let s_slice: &str = &command.0[..];
        match s_slice {
            "forward" => coords = (coords.0 + command.1, coords.1),
            "down" => coords = (coords.0, coords.1 + command.1),
            "up" => coords = (coords.0, coords.1 - command.1),
            _ => {}
        }
    }

    println!("first: {}", coords.0 * coords.1);
}

#[allow(dead_code)]
pub fn task_2() {
    let commands = load_data();
    let mut coords = (0, 0, 0);
    for command in commands {
        let s_slice: &str = &command.0[..];
        match s_slice {
            "forward" => {
                coords = (
                    coords.0 + command.1,
                    coords.1,
                    coords.2 + coords.1 * command.1,
                )
            }
            "down" => coords = (coords.0, coords.1 + command.1, coords.2),
            "up" => coords = (coords.0, coords.1 - command.1, coords.2),
            _ => {}
        }
    }

    println!("second: {}", coords.0 * coords.2);
}
