fn main() {
    let mut result = vec![];
    let mut current = vec![];
    generate_combinations(3, 3, &mut result, &mut current);
    println!("{:?}", result);
}

fn generate_combinations(
    options: u32,
    samples: u32,
    result: &mut Vec<Vec<u32>>,
    current: &mut Vec<u32>,
) {
    // println!("Current {:?}, samples {}", current, samples);
    if samples == 0 {
        result.push(current.clone());
        return;
    }
    for i in 1..=options {
        current.push(i);
        generate_combinations(i, samples - 1, result, current);
        current.pop();
    }
}
