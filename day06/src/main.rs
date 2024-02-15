fn parse() -> (u64, u64) {
    let mut lines = include_str!("../input").lines();
    let time: u64 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let record: u64 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    (time, record)
}

fn ways_to_beat(time: u64, record: u64) -> u64 {
    let mut beats = 0;
    for hold in 0..time {
        let distance = (time - hold) * hold;
        if distance > record {
            beats += 1;
        }
    }
    beats
}

fn main() {
    let data = parse();
    dbg!(data);
    println!("{}", ways_to_beat(data.0, data.1));
}
