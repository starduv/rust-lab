use std::{collections::HashSet, vec};

pub fn run() -> i64 {
    // let mut seeds = vec![[79i64, 79i64 + 14i64], [55i64, 55i64 + 13i64]];
    let mut seeds = vec![
        Seed {
            map_index: 0,
            start: 2276375722,
            end: 2276375722 + 160148132,
        },
        Seed {
            map_index: 0,
            start: 3424292843,
            end: 3424292843 + 82110297,
        },
        Seed {
            map_index: 0,
            start: 1692203766,
            end: 1692203766 + 342813967,
        },
        Seed {
            map_index: 0,
            start: 3289792522,
            end: 3289792522 + 103516087,
        },
        Seed {
            map_index: 0,
            start: 2590548294,
            end: 2590548294 + 590357761,
        },
        Seed {
            map_index: 0,
            start: 1365412380,
            end: 1365412380 + 80084180,
        },
        Seed {
            map_index: 0,
            start: 3574751516,
            end: 3574751516 + 584781136,
        },
        Seed {
            map_index: 0,
            start: 4207087048,
            end: 4207087048 + 36194356,
        },
        Seed {
            map_index: 0,
            start: 1515742281,
            end: 1515742281 + 174009980,
        },
        Seed {
            map_index: 0,
            start: 6434225,
            end: 6434225 + 291842774,
        },
    ];

    let mut maps: Vec<Vec<Vec<i64>>> = vec![Vec::new(); 7];
    let mut map_index = 0usize;
    for line in include_str!("day_five.txt").lines() {
        if line.is_empty() {
            map_index += 1;
        } else {
            let mut entry = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            entry[2] = entry[1] + entry[2];

            maps[map_index].push(entry);
        }
    }

    let mut lowest = std::i64::MAX;
    while seeds.len() > 0 {
        let next = seeds.pop().unwrap();
        if next.map_index >= maps.len() {
            lowest = std::cmp::min(lowest, next.start);
            continue;
        }

        let map = &maps[map_index];
        for entry in map {
            if next.start <= entry[2] && next.end >= entry[1] {
                if next.start < entry[1] {
                    seeds.push(Seed {
                        map_index: next.map_index + 1,
                        start: next.start,
                        end: entry[1] - 1,
                    });
                }

                if next.end > entry[2] {
                    seeds.push(Seed {
                        map_index: next.map_index + 1,
                        start: entry[2] + 1,
                        end: next.end,
                    });
                }

                let diff_start = std::cmp::max(next.start, entry[1]);
                let diff_end = std::cmp::min(next.end, entry[2]);
                let next_start = entry[0] + diff_start - entry[1];
                let next_end = next_start + diff_end - diff_start;
                seeds.push(Seed {
                    map_index: next.map_index + 1,
                    start: next_start,
                    end: next_end,
                });
                break;
            }
        }
    }

    lowest
}

struct Seed {
    map_index: usize,
    start: i64,
    end: i64,
}
