mod day_nineteen;

#[cfg(test)]
mod tests {
    #[test]
    fn day_nineteen_passes() {
        assert_eq!(
            crate::advent_of_code::_2021_::day_nineteen::run(),
            256448566
        );
    }
}
