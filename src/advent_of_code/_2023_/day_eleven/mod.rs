pub fn run() -> usize {
    let mut len = 140;
    let mut galaxies = vec![];
    let mut empty_rows = vec![true; len];
    let mut empty_cols = vec![true; len];
    let mut map = include_str!("day_eleven.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => {
                        empty_rows[row] = empty_rows[row] && false;
                        empty_cols[col] = empty_cols[col] && false;
                        galaxies.push([row, col]);
                        (galaxies.len() - 1).to_string()
                    }
                    _ => {
                        empty_rows[row] = empty_rows[row] && true;
                        empty_cols[col] = empty_cols[col] && true;
                        c.into()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut sum = 0;
    let multiple = 1000000;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let expanded_rows = empty_rows[galaxies[i][0]..galaxies[j][0]]
                .iter()
                .filter(|is_empty| **is_empty)
                .count();

            let start = std::cmp::min(galaxies[i][1], galaxies[j][1]);
            let end = std::cmp::max(galaxies[i][1], galaxies[j][1]);
            let expanded_cols = empty_cols[start..end]
                .iter()
                .filter(|is_empty| **is_empty)
                .count();

            let row_diff = galaxies[j][0].abs_diff(galaxies[i][0])
                + (multiple * expanded_rows - expanded_rows);

            let col_diff = galaxies[j][1].abs_diff(galaxies[i][1])
                + (multiple * expanded_cols - expanded_cols);

            sum += row_diff + col_diff;
        }
    }

    sum
}
