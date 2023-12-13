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

fn main() {
    let (instructions, network) = parse();
    let mut instr_loop = instructions.chars().cycle();

    let mut paths: Vec<String> = network
        .keys()
        .cloned()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .collect();

    let mut steps: u64 = 0;
    loop {
        // print!("{:?}\r", &paths);

        let direction = instr_loop.next().unwrap();
        for path in &mut paths {
            *path = match direction {
                'L' => network.get(path).unwrap().0.to_owned(),
                _ => network.get(path).unwrap().1.to_owned(),
            };
        }
        steps += 1;

        if paths.iter().all(|v| v.ends_with('Z')) {
            break;
        }
    }
    println!("\n\n{}", steps);
}
