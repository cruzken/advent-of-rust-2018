mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    #[test]
    fn solve_day01() {
        use day01::{star_one, star_two};

        let input = load_file("day01.txt");

        assert_eq!(star_one(&input), 3432671);
        assert_eq!(star_two(&input), 5146132);
    }
    #[test]
    fn solve_day02() {
        use day02::{star_one, star_two};

        let input = load_file("day02.txt");

        assert_eq!(star_one(&input), 4714701);
        assert_eq!(star_two(&input), 5121);
    }
    #[test]
    fn solve_day03() {
        use day03::{star_one, star_two};

        let input = load_file("day03.txt");

        assert_eq!(star_one(&input), 352);
        assert_eq!(star_two(&input), 43848);
    }
    #[test]
    fn solve_day04() {
        use day04::{star_one, star_two};

        let input = load_file("day04.txt");

        assert_eq!(star_one(&input), Some(1694));
        assert_eq!(star_two(&input), Some(1148));
    }
    #[test]
    fn solve_day05() {
        use day05::{star_one, star_two};

        let input = load_file("day05.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day06() {
        use day06::{star_one, star_two};

        let input = load_file("day06.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day07() {
        use day07::{star_one, star_two};

        let input = load_file("day07.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day08() {
        use day08::{star_one, star_two};

        let input = load_file("day08.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day09() {
        use day09::{star_one, star_two};

        let input = load_file("day09.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day10() {
        use day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day11() {
        use day11::{star_one, star_two};

        let input = load_file("day11.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day12() {
        use day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day13() {
        use day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day14() {
        use day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day15() {
        use day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day16() {
        use day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day17() {
        use day17::{star_one, star_two};

        let input = load_file("day17.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day18() {
        use day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day19() {
        use day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day20() {
        use day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day21() {
        use day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day22() {
        use day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day23() {
        use day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day24() {
        use day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}
