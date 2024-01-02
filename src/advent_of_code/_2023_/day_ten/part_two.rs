use std::{cell::RefCell, collections::BTreeMap};

pub fn run() -> u32 {
    let north = [-1, 0];
    let south = [1, 0];
    let east = [0, 1];
    let west = [0, -1];

    let features = BTreeMap::from([
        (
            '|',
            Feature::Pipe {
                shape: '|',
                first: north,
                second: south,
                boundaries: Default::default(),
            },
        ),
        (
            '-',
            Feature::Pipe {
                shape: '-',
                first: east,
                second: west,
                boundaries: Default::default(),
            },
        ),
        (
            'L',
            Feature::Pipe {
                shape: 'L',
                first: north,
                second: east,
                boundaries: Default::default(),
            },
        ),
        (
            'J',
            Feature::Pipe {
                shape: 'J',
                first: north,
                second: west,
                boundaries: Default::default(),
            },
        ),
        (
            '7',
            Feature::Pipe {
                shape: '7',
                first: south,
                second: west,
                boundaries: Default::default(),
            },
        ),
        (
            'F',
            Feature::Pipe {
                shape: 'F',
                first: south,
                second: east,
                boundaries: Default::default(),
            },
        ),
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

        {
            let from = map[from[0] as usize][from[1] as usize];
            let to = map[to[0] as usize][to[1] as usize];
            to.update_boundaries(&from);
        }

        if let Some(row) = map.get(to[0] as usize) {
            if let Some(Feature::Pipe {
                first,
                second,
                shape: _,
                boundaries: _,
            }) = row.get(to[1] as usize)
            {
                let first = [first[0] + to[0], first[1] + to[1]];
                let second = [second[0] + to[0], second[1] + to[1]];
                if first.eq(&from) {
                    // update boundary
                    stack.push((to, second));
                } else if second.eq(&from) {
                    // update boundary
                    stack.push((to, first));
                }
            }
        }
    }

    0
}

#[derive(Debug)]
enum Feature {
    Pipe {
        shape: char,
        first: [isize; 2],
        second: [isize; 2],
        boundaries: RefCell<Boundaries>,
    },
    Ground,
    Start,
}

#[derive(Debug, Default)]
struct Boundaries {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl Feature {
    fn update(&self, from: &char, to: &char) -> () {
        // match shape {
        //     '|' => {}
        //     '-' => {}
        //     'L' => {}
        //     'J' => {}
        //     '7' => {}
        //     'F' => {}
        //     _ => {}
        // }
    }

    pub fn update_boundaries(&self, from: &Feature) -> () {
        if let Feature::Pipe {
            shape: from,
            first: _,
            second: _,
            boundaries: _,
        } = from
        {
            if let Feature::Pipe {
                shape: to,
                first: _,
                second: _,
                boundaries: _,
            } = self
            {
                self.update(from, to);
            }
        }
    }
}
