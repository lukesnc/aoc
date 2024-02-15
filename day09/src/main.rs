fn parse() -> Vec<Vec<i32>> {
    include_str!("../input")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let data = parse();

    let mut sum = 0;
    for line in &data {
        let mut diffs = line.clone();
        let mut map: Vec<Vec<i32>> = Vec::new();

        while !diffs.iter().all(|&x| x == 0) {
            let len = diffs.len();
            for i in 0..len - 1 {
                diffs.push(diffs.get(i + 1).unwrap() - diffs.get(i).unwrap());
            }

            diffs.drain(0..len);
            map.insert(0, diffs.clone());
        }
        map.push(line.clone());

        let mut prediction = 0;
        for v in &map {
            prediction = v.first().unwrap() - prediction;
        }
        sum += prediction;
    }
    println!("{}", sum);
}
