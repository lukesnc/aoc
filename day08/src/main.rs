use std::collections::BTreeMap;

fn parse() -> (String, BTreeMap<String, (String, String)>) {
    let mut lines = include_str!("../input").lines().filter(|&x| x != "");
    let instructions = lines.next().unwrap().to_owned();
    let network: BTreeMap<String, (String, String)> = lines
        .map(|line| {
            (
                line.split(" = ").nth(0).unwrap().to_owned(),
                (
                    line.split(" = ")
                        .nth(1)
                        .unwrap()
                        .split(", ")
                        .nth(0)
                        .unwrap()
                        .replace("(", ""),
                    line.split(" = ")
                        .nth(1)
                        .unwrap()
                        .split(", ")
                        .nth(1)
                        .unwrap()
                        .replace(")", ""),
                ),
            )
        })
        .collect();
    (instructions, network)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_vec(numbers: &Vec<u64>) -> u64 {
    if numbers.is_empty() {
        0
    } else {
        let mut result = numbers[0];
        for &number in numbers.iter().skip(1) {
            result = lcm(result, number);
        }
        result
    }
}

fn main() {
    let (instructions, network) = parse();

    let mut paths: Vec<(String, u64)> = network
        .keys()
        .cloned()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .map(|x| (x, 0))
        .collect();

    for (path, steps) in &mut paths {
        let mut instr_loop = instructions.chars().cycle();
        loop {
            let direction = instr_loop.next().unwrap();
            *path = match direction {
                'L' => network.get(path).unwrap().0.to_owned(),
                _ => network.get(path).unwrap().1.to_owned(),
            };
            *steps += 1;

            if path.ends_with('Z') {
                break;
            }
        }
    }

    let lengths: Vec<u64> = paths.iter().map(|x| x.1).collect();
    let res = lcm_of_vec(&lengths);
    println!("{}", res);
}
