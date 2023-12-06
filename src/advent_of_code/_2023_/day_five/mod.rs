use std::{collections::HashSet, vec};

pub fn run() -> i64 {
    // let mut visited = vec![HashSet::new(); 7];
    // let mut seeds = vec![[79i64, 79i64 + 14i64], [55i64, 55i64 + 13i64]];
    let mut seeds = vec![
        [2276375722, 2276375722 + 160148132 - 1],
        [3424292843, 3424292843 + 82110297 - 1],
        [1692203766, 1692203766 + 342813967 - 1],
        [3289792522, 3289792522 + 103516087 - 1],
        [2590548294, 2590548294 + 590357761 - 1],
        [1365412380, 1365412380 + 80084180 - 1],
        [3574751516, 3574751516 + 584781136 - 1],
        [4207087048, 4207087048 + 36194356 - 1],
        [1515742281, 1515742281 + 174009980 - 1],
        [6434225, 6434225 + 291842774 - 1],
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
        let mut next = seeds.pop().unwrap();

        for (map_index, map) in maps.iter().enumerate() {
            for entry in map {
                if next[0] <= entry[2] && next[1] >= entry[1] {
                    let temp = [next[0], entry[1] - 1];
                    if map_index == 6 && next[0] < entry[1] {
                        lowest = std::cmp::min(lowest, temp[0]);
                    } else if next[0] < entry[1] {
                        seeds.push(temp);
                    }

                    let temp = [entry[2] + 1, next[1]];
                    if map_index == 6 && next[1] > entry[2] {
                        lowest = std::cmp::min(lowest, temp[0]);
                    } else if next[1] > entry[2] {
                        seeds.push(temp);
                    }

                    let diff_start = std::cmp::max(next[0], entry[1]);
                    let diff_end = std::cmp::min(next[1], entry[2]);
                    let next_start = entry[0] + diff_start - entry[1];
                    let next_end = next_start + diff_end - diff_start;
                    next = [next_start, next_end];

                    break;
                }
            }

            if map_index == maps.len() - 1 {
                lowest = std::cmp::min(lowest, next[0]);
            }
        }
    }

    lowest
}
