mod day_seven;
mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod day_six;
mod day_eight;

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
            crate::advent_of_code::_2023_::day_four::run(),
            5625994
        );
    }

    #[test]
    fn day_five_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_five::run(),
            46294175
        );
    }

    #[test]
    fn day_six_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_six::run(),
            11427375
        );
    }

    #[test]
    fn day_seven_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_seven::run(),
            256448566
        );
    }

    #[test]
    fn day_eight_passes() {
        assert_eq!(
            crate::advent_of_code::_2023_::day_eight::run(),
            256448566
        );
    }
}
