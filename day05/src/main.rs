fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut data = input.lines().filter(|&x| x != "");
    let seeds: Vec<usize> = data.next().unwrap().split(": ").collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let lines: Vec<&str> = data.collect();

    let seed_ranges: Vec<&[usize]> = seeds.chunks(2).collect();
    let mut lowest: Option<usize> = None;
    let mut visited: Vec<usize> = Vec::new();

    for range in seed_ranges {
        dbg!(range);
        for seed in range[0]..range[0] + range[1] {
            if visited.contains(&seed) {
                continue;
            }
            visited.push(seed);
            // dbg!(seed);

            let mut curr = seed.clone();
            let mut skip = false; // Skip to next map
            for line in lines.clone() {
                if line.contains("map") {
                    skip = false;
                    continue;
                }
                if skip {
                    continue;
                }

                let key: Vec<usize> = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let dest = key[0];
                let src = key[1];
                let range = key[2];

                for i in src..src + range {
                    if i == curr {
                        let res = dest + (i - src);
                        // dbg!(res);
                        curr = res;
                        skip = true;
                        break;
                    }
                }
            }

            lowest = Some(lowest.map_or(curr, |num| num.min(curr)));
        }
    }

    println!("{}", lowest.unwrap());
}
