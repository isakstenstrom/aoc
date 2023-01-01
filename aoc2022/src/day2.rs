use crate::util::read_input_from_file;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn from_char(c: &char) -> Move {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Invalid input: Move not in list"),
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_result(&self, other: &Move) -> GameResult {
        if self == other {
            return GameResult::Draw;
        } else if (self == &Move::Paper && other == &Move::Rock)
            || (self == &Move::Scissors && other == &Move::Paper)
            || (self == &Move::Rock && other == &Move::Scissors)
        {
            return GameResult::Win;
        }
        GameResult::Lose
    }

    fn get_opposing_move(&self, game_result: &GameResult) -> &Move {
        match (game_result, self) {
            (GameResult::Win, Move::Rock) => &Move::Paper,
            (GameResult::Win, Move::Paper) => &Move::Scissors,
            (GameResult::Win, Move::Scissors) => &Move::Rock,
            (GameResult::Lose, Move::Rock) => &Move::Scissors,
            (GameResult::Lose, Move::Paper) => &Move::Rock,
            (GameResult::Lose, Move::Scissors) => &Move::Paper,
            _ => self,
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Lose,
}
impl GameResult {
    fn from_char(c: &char) -> GameResult {
        match c {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("Invalid input: Move not in list"),
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

// Answer should be "15523"
pub fn task1() -> u32 {
    read_input_from_file("day2.txt")
        .iter()
        .map(|line| {
            let p1 = Move::from_char(&line.chars().next().unwrap());
            let p2 = Move::from_char(&line.chars().nth(2).unwrap());

            p2.to_score() + p2.get_result(&p1).to_score()
            // score += p2.to_score();
            // score += p2.get_result(&p1).to_score();
        })
        .sum()
}

// Answer should be "15702"
pub fn task2() -> u32 {
    read_input_from_file("day2.txt")
        .iter()
        .map(|line| {
            let p1 = Move::from_char(&line.chars().next().unwrap());
            let game_result = GameResult::from_char(&line.chars().nth(2).unwrap());

            p1.get_opposing_move(&game_result).to_score() + game_result.to_score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), 15523);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 15702);
    }
}
