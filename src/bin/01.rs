advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re_first = Regex::new(r"\D*(\d).*").unwrap();
    let re_last = Regex::new(r".*(\d)\D*").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let first = re_first.captures(line).unwrap();
        let last = re_last.captures(line).unwrap();
        let tens: u32 = first[1].parse().unwrap();
        let units: u32 = last[1].parse().unwrap();
        // println!("{line} extracted {tens} {units}");
        sum += tens * 10 + units;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_first = Regex::new(r"\D*(\d).*").unwrap();
    let re_last = Regex::new(r".*(\d)\D*").unwrap();

    let re_numbers = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let pre_parsed = re_numbers.replace_all(input, |cap: &regex::Captures| match &cap[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("What digits are you talking about..."),
    });

    // let result = re.replace_all(string, |cap: &Captures| {
    //     match &cap[0] {
    //         "or" => "str",
    //         "e" => "er",
    //         _ => panic!("We should never get here"),
    //     }.to_string()
    // )
    let mut sum = 0;
    for line in pre_parsed.lines() {
        let first = re_first.captures(line).unwrap();
        let last = re_last.captures(line).unwrap();
        let tens: u32 = first[1].parse().unwrap();
        let units: u32 = last[1].parse().unwrap();
        // println!("{line} extracted {tens} {units}");
        sum += tens * 10 + units;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
