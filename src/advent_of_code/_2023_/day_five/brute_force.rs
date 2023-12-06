use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    thread, time::Duration,
};

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn run() -> u64 {
    // let seeds = vec![[79, 79 + 14], [55, 55 + 13]];
    let seeds = vec![
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

    let maps: Arc<RwLock<Vec<Vec<Vec<u64>>>>> = Arc::new(RwLock::new(vec![Vec::new(); 7]));

    {
        let mut map_index = 0usize;
        let mut maps = maps.write().unwrap();
        for line in include_str!("day_five.txt").lines() {
            if line.is_empty() {
                maps[map_index].sort_by(|a, b| a[1].cmp(&b[1]));
                map_index += 1;
            } else {
                let mut entry = line
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                entry[2] = entry[1] + entry[2] - 1;

                maps[map_index].push(entry);
            }
        }

        maps[map_index].sort_by(|a, b| a[1].cmp(&b[1]));
    }

    for seed_range in seeds {
        let maps = maps.clone();
        thread::spawn(move || {
            GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
            let mut lowest = std::u64::MAX;
            for seed in seed_range[0]..seed_range[1] {
                let mut next = seed;
                let maps = maps.read().unwrap();
                for map in maps.iter() {
                    for entry in map {
                        if entry[1] <= next && next <= entry[2] {
                            let diff = next - entry[1];
                            next = entry[0] + diff;
                            break;
                        }
                    }
                }
                lowest = std::cmp::min(lowest, next);
            }
            println!("THE LOWEST VALUE IN THIS THREAD IS: {lowest}");
            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    }

    // Wait for other threads to finish.
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1)); 
    }

    0
}
