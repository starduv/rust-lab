use std::collections::BTreeMap;

pub fn run() -> usize {
    let map = include_str!("day_ten.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let row_size = map.len();
    let col_size = map[0].len();
    let mut enclosed = vec![vec![0; col_size]; row_size];
    for row in 0..row_size {
        let mut state = State::Closed;
        for col in 0..col_size {
            match map[row][col] {
                '-' => {}
                '.' => match state {
                    State::Open => enclosed[row][col] += 1,
                    _ => {}
                },
                _ => match state {
                    State::Open => state = State::Closed,
                    State::Closed => state = State::Open,
                },
            }
        }
    }

    for col in 0..col_size {
        let mut state = State::Closed;
        for row in 0..row_size {
            match map[row][col] {
                '|' => {}
                '.' => match state {
                    State::Open => enclosed[row][col] += 1,
                    _ => {}
                },
                _ => match state {
                    State::Open => state = State::Closed,
                    State::Closed => state = State::Open,
                },
            }
        }
    }

    println!("{enclosed:?}");

    enclosed.iter().flatten().filter(|v| 2u32.eq(v)).count()
}

#[derive(Debug)]
enum Feature {
    Pipe([isize; 2], [isize; 2]),
    Ground,
    Start,
}

enum State {
    Open,
    Closed,
}
