mod day_four;
mod day_one;
mod day_three;
mod day_two;

#[cfg(test)]
mod tests {

    #[test]
    fn day_one_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_one::run().unwrap(),
            54249
        );
    }

    #[test]
    fn day_two_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_two::run().unwrap(),
            71535
        );
    }

    #[test]
    fn day_three_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_three::run().unwrap(),
            75519888
        );
    }

    #[test]
    fn day_four_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_four::run().unwrap(),
            5625994
        );
    }
}
