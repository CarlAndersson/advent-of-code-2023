advent_of_code::solution!(3);
use regex::Regex;

fn is_symbol(c: char) -> bool{
    !(c.is_digit(10) || c == '.')
}

fn check_surroundings(prev_line: &str, this_line: &str, next_line: &str, start: usize, end: usize) -> bool {
    let check_start = if start == 0 {start} else {start - 1};
    let check_end = if end == 140 {end} else {end + 1};

    prev_line[check_start..check_end].chars().any(is_symbol)
    || this_line[check_start..check_end].chars().any(is_symbol)
    || next_line[check_start..check_end].chars().any(is_symbol)
}


fn sum_line(prev_line: &str, this_line: &str, next_line: &str) -> u32 {
    let re: Regex = Regex::new(r"\d+").unwrap();
    // let sum: u32 = 0;
    re.find_iter(this_line)
    .filter(|m| check_surroundings(prev_line, this_line, next_line, m.start(), m.end()))
    .map(|m| m.as_str().parse::<u32>().unwrap())
    .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut prev_line: &str = &".".repeat(140);
    let mut this_line: &str = &".".repeat(140);

    let mut sum: u32 = 0;
    for next_line in input.lines() {
        sum += sum_line(prev_line, this_line, next_line);
        // Prepare for next iteration
        prev_line = this_line;
        this_line = next_line;
    }
    sum += sum_line(prev_line, this_line, &".".repeat(140));
    // None
    Some(sum)
}

fn get_digit_at_end(input: &str) -> String {
    input
    .chars()
    .rev()
    .take_while(|c| c.is_ascii_digit())
    .collect::<String>()
    .chars()
    .rev()
    .collect::<String>()
}

fn get_digit_at_start(input: &str) -> String {
    input
    .chars()
    .take_while(|c| c.is_ascii_digit())
    .collect::<String>()
}

fn find_gears(prev_line: &str, this_line: &str, next_line: &str) -> u32 {
    let re = Regex::new(r"\*").unwrap();
    // println!("Started with lines:");
    // println!("{prev_line}");
    // println!("{this_line}");
    // println!("{next_line}");

    re.find_iter(this_line).map(|m| {
        // println!("{m:?}");
        let start = m.start().max(1);
        let end = m.end().min(139);


        let left_digit = get_digit_at_end(&this_line[..start]).parse::<u32>().ok();
        let right_digit = get_digit_at_start(&this_line[end..]).parse::<u32>().ok();

        // if left_digit.is_some() {println!("Parsed left {left_digit:?}")};
        // if right_digit.is_some() {println!("Parsed right {right_digit:?}")};

        let above_left = get_digit_at_end(&prev_line[..start]);
        let above = &prev_line[m.start() .. m.end()];
        let above_right = get_digit_at_start(&prev_line[end..]);

        let above_digit: Option<u32> = match (above_left.as_str(), above, above_right.as_str()) {
            ("", ".", "") => None,
            (right, ".", left) if right.len() * left.len() > 0=> {
                // println!("Returning {left:?} * {right:?}");
                return left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
            }
            (right, mid, left) => (format!("{}{}{}", right, mid.replace(".", ""), left)).parse().ok()
        };

        // if above_digit.is_some() {
        //     println!("Parsed above: left={above_left:?}, mid={above:?}, right={above_right:?}");
        // }

        let below_left = get_digit_at_end(&next_line[..start]);
        let below = &next_line[m.start() .. m.end()];
        let below_right = get_digit_at_start(&next_line[end..]);

        let below_digit: Option<u32> = match (below_left.as_str(), below, below_right.as_str()) {
            ("", ".", "") => None,
            (right, ".", left) if right.len() * left.len() > 0=> {
                // println!("Returning {left:?} * {right:?}");
                return left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
            }
            (right, mid, left) => (format!("{}{}{}", right, mid.replace(".", ""), left)).parse().ok()
        };

        // if below_digit.is_some() {
        //     println!("Parsed below: left={below_left:?}, mid={below:?}, right={below_right:?}");
        // }

        // println!("Left {left_digit:?}\nRight: {right_digit:?}\nAbove: {above_digit:?}\nBelow: {below_digit:?}");
        match (left_digit, right_digit, above_digit, below_digit) {
            (Some(left), Some(right), None, None) => {
                // println!("Returning {left:?} * {right:?}");
                left * right
            },
            (Some(left), None, Some(above), None) => {
                // println!("Returning {left:?} * {above:?}");
                left * above
            },
            (Some(left), None, None, Some(below)) => {
                // println!("Returning {left:?} * {below:?}");
                left * below
            },
            (None, Some(right), Some(above), None) => {
                // println!("Returning {right:?} * {above:?}");
                right * above
            },
            (None, Some(right), None, Some(below)) => {
                // println!("Returning {right:?} * {below:?}");
                right * below
            },
            (None, None, Some(above), Some(below)) => {
                // println!("Returning {above:?} * {below:?}");
                above * below
            },
            _ => 0
        }
    }).sum()

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut prev_line: &str = &".".repeat(140);
    let mut this_line: &str = &".".repeat(140);

    let mut sum: u32 = 0;
    for next_line in input.lines() {
        // Do stuff
        let line_sum = find_gears(prev_line, this_line, next_line);
        sum += line_sum;
        // if line_sum != 0 {println!("Line summed to {line_sum:?}, current sum is {sum:?}")}
        // Prepare for next iteration
        prev_line = this_line;
        this_line = next_line;
    }
    sum += find_gears(prev_line, this_line, &".".repeat(140));
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
