use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    game: u32,
    winning: HashSet<u32>,
    bets: HashSet<u32>
}

impl Card {
    fn parse_line(line: &str) -> Self {
        let (game, numbers) = line.split_once(":").unwrap();
        let (winners, bets) = numbers.split_once("|").unwrap();
        let game = game.strip_prefix("Card").unwrap().trim().parse::<u32>().unwrap();

        Self {
            game: game,
            winning: HashSet::from_iter(winners.split_whitespace().map(|d| d.parse::<u32>().unwrap())),
            bets: HashSet::from_iter(bets.split_whitespace().map(|d| d.parse::<u32>().unwrap()))
        }
    }

    fn matches(&self) -> u32 {
        self.winning.intersection(&self.bets).count() as u32
    }

    fn score(&self) -> u32 {

        match self.matches() {
            0 => 0,
            wins => 2u32.pow(wins - 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines()
        .map(Card::parse_line)
        .map(|card| card.score()).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input.lines().map(Card::parse_line).collect();
    let mut counts: Vec<u32> = vec![1; cards.len()];
    for (idx, card) in cards.iter().enumerate() {
        let score = card.matches() as usize;
        let start = (idx + 1).min(cards.len());
        let end = (idx + score + 1).min(cards.len());
        let inc_by = counts[idx];
        counts[start .. end].iter_mut().for_each(|e| *e += inc_by);
    };

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
