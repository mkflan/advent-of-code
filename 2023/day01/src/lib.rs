pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut i| {
            let first = unsafe { i.next().unwrap_unchecked() };
            let last = i.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "fo4r")
        .replace("five", "fi5e")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "ni9e");

    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut i| {
            let first = unsafe { i.next().unwrap_unchecked() };
            let last = i.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let p1_sample_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, solve_part1(p1_sample_input));

        let p2_sample_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, solve_part2(p2_sample_input));
    }
}
