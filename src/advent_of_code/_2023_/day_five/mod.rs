use std::vec;

pub fn run() -> i64 {
    // let mut seeds = vec![
    // Seed::new(0, 79i64, 79i64 + 14i64),
    // Seed::new(0, 55i64, 55i64 + 13i64),
    // ];
    let mut seeds = vec![
        Seed::new(0, 2276375722, 2276375722 + 160148132 - 1),
        Seed::new(0, 3424292843, 3424292843 + 82110297 - 1),
        Seed::new(0, 1692203766, 1692203766 + 342813967 - 1),
        Seed::new(0, 3289792522, 3289792522 + 103516087 - 1),
        Seed::new(0, 2590548294, 2590548294 + 590357761 - 1),
        Seed::new(0, 1365412380, 1365412380 + 80084180 - 1),
        Seed::new(0, 3574751516, 3574751516 + 584781136 - 1),
        Seed::new(0, 4207087048, 4207087048 + 36194356 - 1),
        Seed::new(0, 1515742281, 1515742281 + 174009980 - 1),
        Seed::new(0, 6434225, 6434225 + 291842774 - 1),
    ];

    let mut maps: Vec<Vec<Vec<i64>>> = vec![Vec::new(); 7];

    {
        let mut map_index = 0usize;
        for line in include_str!("day_five.txt").lines() {
            if line.is_empty() {
                map_index += 1;
            } else {
                let mut entry = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                entry[2] = entry[1] + entry[2] - 1;

                maps[map_index].push(entry);
            }
        }
    }

    let mut lowest = std::i64::MAX;
    while !seeds.is_empty() {
        let mut next = seeds.pop().unwrap();
        if next.map_index >= maps.len() {
            lowest = std::cmp::min(lowest, next.start);
            continue;
        }

        for entry in &maps[next.map_index] {
            if next.start <= entry[2] && next.end >= entry[1] {
                if next.start < entry[1] {
                    seeds.push(Seed::new(next.map_index + 1, next.start, entry[1] - 1));
                }

                if next.end > entry[2] {
                    seeds.push(Seed::new(next.map_index + 1, entry[2] + 1, next.end));
                }

                let diff_start = std::cmp::max(next.start, entry[1]);
                let diff_end = std::cmp::min(next.end, entry[2]);
                let next_start = entry[0] + diff_start - entry[1];
                let next_end = next_start + diff_end - diff_start;
                seeds.push(Seed::new(next.map_index + 1, next_start, next_end));

                next.range_found = true;

                break;
            }
        }

        if !next.range_found {
            next.map_index += 1;
            seeds.push(next);
        }
    }

    lowest
}

#[derive(Debug)]
struct Seed {
    range_found: bool,
    map_index: usize,
    start: i64,
    end: i64,
}

impl Seed {
    fn new(map_index: usize, start: i64, end: i64) -> Self {
        Seed {
            map_index,
            start,
            end,
            range_found: false,
        }
    }
}
