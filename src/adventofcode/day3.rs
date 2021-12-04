use std::fs;

fn load_data() -> (Vec<String>, i32) {
    let contents = fs::read_to_string("./src/adventofcode/day_3/input.txt")
        .expect("Something went wrong reading the file");

    let mut count = 0;
    let vec = contents
        .split("\n")
        .map(|s| {
            count += 1;
            String::from(s)
        })
        .collect();
    return (vec, count);
}

#[allow(dead_code)]
pub fn task_1() {
    let (lines, count) = load_data();
    const LEN: usize = 12;
    let mut statistic: [i32; LEN] = [0; LEN];
    for line in lines {
        for i in 0..LEN {
            if line.as_bytes()[i] as char == '1' {
                statistic[i] += 1;
            }
        }
    }
    let mut gamma: i32 = 0;
    for i in 0..LEN {
        if statistic[i] > count / 2 {
            gamma |= 1 << (LEN - i - 1)
        }
    }
    let epsilon = gamma ^ (1 << LEN) - 1;

    println!(
        "First: gamma {}, epsilon {}, result {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

#[allow(dead_code)]
pub fn task_2() {
    let (lines, count) = load_data();
    const LEN: usize = 12;
    let mut statistic: [i32; LEN] = [0; LEN];
    for line in &lines {
        for i in 0..LEN {
            if line.as_bytes()[i] as char == '1' {
                statistic[i] += 1;
            }
        }
    }
    let mut gamma: i32 = 0;
    for i in 0..LEN {
        if statistic[i] >= count / 2 {
            gamma |= 1 << (LEN - i - 1)
        }
    }
    let mut epsilon: i32 = 0;
    for i in 0..LEN {
        if statistic[i] < count / 2 {
            epsilon |= 1 << (LEN - i - 1)
        }
    }

    let mut gen = lines.into_boxed_slice();
    for i in 0..LEN {
        let numb = if (gamma >> (LEN - i - 1)) == 1 {
            '1'
        } else {
            '0'
        };
        gen = gen
            .iter()
            .map(|s| String::from(s))
            .filter(|s| s.as_bytes()[i] as char == numb)
            .collect();
        if gen.len() == 1 {
            break;
        }
    }
    let gen_res = gen.first().unwrap();
    println!("Generator: {}", gen_res);

    let (lines, _) = load_data();
    let mut scr = lines.into_boxed_slice();
    for i in 0..LEN {
        let numb = if (epsilon >> (LEN - i - 1)) == 1 {
            '0'
        } else {
            '1'
        };
        scr = scr
            .iter()
            .map(|s| String::from(s))
            .filter(|s| s.as_bytes()[i] as char == numb)
            .collect();
        if scr.len() == 1 {
            break;
        }
    }

    let scr_res = scr.first().unwrap();
    println!("Scrubber: {}", scr_res);
}
