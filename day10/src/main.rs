use std::{char, collections::HashMap};

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

fn walk(map: &Vec<Vec<char>>, mut pos: (usize, usize), mut coming_from: char) -> u32 {
    let mut distance = 0;
    loop {
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
            None => return (distance as f32 / 2.0).round() as u32,
        }

        distance += 1;
    }
}

fn main() {
    let map = parse();

    let mut pos = start_pos(&map).unwrap();
    pos = (pos.0, pos.1 + 1);
    let coming_from = 'W';

    println!("{}", walk(&map, pos, coming_from));
}
