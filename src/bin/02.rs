advent_of_code::solution!(2);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();
    let game_id_re = Regex::new(r"Game (\d+):").unwrap();

    let bluemax: u32 = 14;
    let redmax: u32 = 12;
    let greenmax: u32 = 13;

    let mut sum = 0;
    for line in input.lines() {
        let not_too_many_green = green_re.captures_iter(line).fold(true, |acc, c| {
            acc && (c[1].parse::<u32>().unwrap() <= greenmax)
        });
        let not_too_many_blue = blue_re.captures_iter(line).fold(true, |acc, c| {
            acc && (c[1].parse::<u32>().unwrap() <= bluemax)
        });
        let not_too_many_red = red_re.captures_iter(line).fold(true, |acc, c| {
            acc && (c[1].parse::<u32>().unwrap() <= redmax)
        });
        if not_too_many_blue && not_too_many_green && not_too_many_red {
            sum += game_id_re.captures(line).unwrap()[1]
                .parse::<u32>()
                .unwrap();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let min_red: u32 = red_re
            .captures_iter(line)
            .map(|c| c[1].parse().unwrap())
            .max()
            .unwrap();
        let min_green: u32 = green_re
            .captures_iter(line)
            .map(|c| c[1].parse().unwrap())
            .max()
            .unwrap();
        let min_blue: u32 = blue_re
            .captures_iter(line)
            .map(|c| c[1].parse().unwrap())
            .max()
            .unwrap();
        sum += min_red * min_green * min_blue;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
