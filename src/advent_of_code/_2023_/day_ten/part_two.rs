use std::collections::BTreeMap;

pub fn run() -> u32 {
    let north = [-1, 0];
    let south = [1, 0];
    let east = [0, 1];
    let west = [0, -1];

    let features = BTreeMap::from([
        ('|', Feature::Pipe(north, south)),
        ('-', Feature::Pipe(east, west)),
        ('L', Feature::Pipe(north, east)),
        ('J', Feature::Pipe(north, west)),
        ('7', Feature::Pipe(south, west)),
        ('F', Feature::Pipe(south, east)),
        ('S', Feature::Start),
        ('.', Feature::Ground),
    ]);

    let mut start = None;
    let map = include_str!("day_ten.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let feature = features.get(&c).expect(&format!(
                        "i was looking for a char and found this '{c}' instead"
                    ));

                    if let Feature::Start = feature {
                        start = Some([row as isize, col as isize])
                    }

                    feature
                })
                .collect::<Vec<&Feature>>()
        })
        .collect::<Vec<Vec<&Feature>>>();

    let start = start.unwrap();
    let mut layout = vec![vec![0; map.len()]; map.len()];
    layout[start[0] as usize][start[1] as usize] = 1;
    let mut stack = vec![
        (start, [north[0] + start[0], north[1] + start[1]]),
        (start, [south[0] + start[0], south[1] + start[1]]),
        (start, [east[0] + start[0], east[1] + start[1]]),
        (start, [west[0] + start[0], west[1] + start[1]]),
    ];

    while stack.len() > 0 {
        let (from, to) = stack.pop().unwrap();
        if to[0] < 0 || to[1] < 0 {
            continue;
        }

        if let Some(row) = map.get(to[0] as usize) {
            if let Some(Feature::Pipe(first, second)) = row.get(to[1] as usize) {
                let first = [first[0] + to[0], first[1] + to[1]];
                let second = [second[0] + to[0], second[1] + to[1]];
                if first.eq(&from) {
                    layout[to[0] as usize][to[1] as usize] = 1;
                    stack.push((to, second));
                }

                if second.eq(&from) {
                    layout[to[0] as usize][to[1] as usize] = 1;
                    stack.push((to, first));
                }
            }
        }
    }

    let mut enclosed = 0;
    let size = layout.len();
    for row in 0..size {
        for col in 0..size {
            if layout[row][col] == 0 && is_enclosed(row, col, &layout) {
                enclosed += 1;
            }
        }
    }

    println!("{layout:?}");

    *layout.iter().flatten().max().unwrap()
}

#[derive(Debug)]
enum Feature {
    Pipe([isize; 2], [isize; 2]),
    Ground,
    Start,
}

fn is_enclosed(row: usize, col: usize, layout: &Vec<Vec<u32>>) -> bool {
    false
}
