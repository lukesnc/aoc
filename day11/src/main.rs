fn parse() -> Vec<String> {
    include_str!("../input")
        .lines()
        .map(|line| String::from(line))
        .collect()
}

fn expand(lines: &mut Vec<String>) {
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line.chars().all(|x| x == '.') {
            lines.insert(i, ".".repeat(line.len()));
            i += 1;
        }
        i += 1;
    }

    i = 0;
    while i < lines[0].len() {
        if lines.iter().all(|line| line.chars().nth(i).unwrap() == '.') {
            lines.iter_mut().for_each(|line| line.insert(i, '.'));
            i += 1;
        }
        i += 1;
    }
}

fn distance(point1: &(usize, usize), point2: &(usize, usize)) -> usize {
    let dx = point2.1.max(point1.1) - point2.1.min(point1.1);
    let dy = point2.0.max(point1.0) - point2.0.min(point1.0);
    dx + dy
}

fn main() {
    let mut universe = parse();
    expand(&mut universe);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, line) in universe.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        let p = &galaxies[i];
        for j in i + 1..galaxies.len() {
            let other = &galaxies[j];
            sum += distance(p, other);
        }
    }
    println!("{}", sum);
}
