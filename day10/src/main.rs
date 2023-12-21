use std::collections::HashMap;

fn parse() -> Vec<Vec<char>> {
    include_str!("../input")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn start_pos(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn char_at(map: &Vec<Vec<char>>, pos: (usize, usize)) -> char {
    *map.get(pos.0).unwrap().get(pos.1).unwrap()
}

fn next(current_char: char, from_direction: char) -> Option<char> {
    let valid = HashMap::from([
        ('N', HashMap::from([('|', 'S'), ('L', 'E'), ('J', 'W')])),
        ('S', HashMap::from([('|', 'N'), ('7', 'W'), ('F', 'E')])),
        ('E', HashMap::from([('-', 'W'), ('L', 'N'), ('F', 'S')])),
        ('W', HashMap::from([('-', 'E'), ('J', 'N'), ('7', 'S')])),
    ]);

    Some(valid.get(&from_direction)?.get(&current_char)?.clone())
}

fn invert(direction: char) -> char {
    match direction {
        'N' => 'S',
        'S' => 'N',
        'E' => 'W',
        'W' => 'E',
        _ => panic!("oops"),
    }
}

fn main() {
    let map = parse();
    let mut polygon: Vec<(usize, usize)> = Vec::new();

    let mut pos = start_pos(&map).unwrap();
    polygon.push(pos);

    pos = (pos.0, pos.1 + 1);
    let mut coming_from = 'W';

    let mut _distance = 0;
    loop {
        polygon.push(pos);
        let cur = char_at(&map, pos);

        match next(cur, coming_from) {
            Some(direction) => {
                pos = match direction {
                    'N' => (pos.0 - 1, pos.1),
                    'S' => (pos.0 + 1, pos.1),
                    'E' => (pos.0, pos.1 + 1),
                    'W' => (pos.0, pos.1 - 1),
                    _ => panic!("oops"),
                };
                coming_from = invert(direction);
            }
            None => break,
        }

        _distance += 1;
    }
    // println!("{}", (distance as f32 / 2.0).round() as u32);

    let mut score = 0;
    for line in map.iter().enumerate() {
        let mut hits = 0;
        let mut buf = 0;
        for c in line.1.iter().enumerate() {
            let pos = (line.0, c.0);
            if polygon.contains(&pos) {
                hits += 1;
            } else if hits % 2 == 1 && *c.1 == '.' {
                buf += 1;
            } else if hits > 0 && hits % 2 == 0 {
                score += buf;
                buf = 0;
            }
        }
    }
    println!("{}", score);
}
