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

    let mut steps = 0;
    let mut cur = network.keys().nth(0).unwrap();
    loop {
        cur = match instr_loop.next().unwrap() {
            'L' => &network.get(cur).unwrap().0,
            _ => &network.get(cur).unwrap().1,
        };
        steps += 1;

        if cur == "ZZZ" {
            break;
        }
    }
    println!("{}", steps);
}
