use std::str::FromStr;

use thiserror::Error;

use super::{Solver, SolverError};

#[derive(Debug, Error)]
enum Error {
    #[error("Attempted to play invalid deck item {0}")]
    InvalidPlay(String),

    #[error("Invalid round {0}")]
    InvalidRound(String),
}

enum Deck {
    Rock,
    Paper,
    Scissor,
}

impl Deck {
    fn score(&self) -> usize {
        match self {
            Deck::Rock => 1,
            Deck::Paper => 2,
            Deck::Scissor => 3,
        }
    }
}

enum Outcome {
    Win,
    Lost,
    Draw,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x" => Ok(Self::Lost),
            "y" => Ok(Self::Draw),
            "z" => Ok(Self::Win),
            _ => Err(Error::InvalidPlay(s.to_string())),
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lost => 0,
            Outcome::Draw => 3,
        }
    }
}

impl FromStr for Deck {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "x" => Ok(Self::Rock),
            "b" | "y" => Ok(Self::Paper),
            "c" | "z" => Ok(Self::Scissor),
            _ => Err(Error::InvalidPlay(s.to_string())),
        }
    }
}

trait Round {
    fn play(&self) -> usize;
}

struct Round1(Deck, Deck);

struct Round2(Deck, Outcome);

impl Round1 {
    fn eval(&self) -> Outcome {
        match (&self.1, &self.0) {
            (Deck::Rock, Deck::Scissor) => Outcome::Win,
            (Deck::Rock, Deck::Paper) => Outcome::Lost,

            (Deck::Paper, Deck::Rock) => Outcome::Win,
            (Deck::Paper, Deck::Scissor) => Outcome::Lost,

            (Deck::Scissor, Deck::Paper) => Outcome::Win,
            (Deck::Scissor, Deck::Rock) => Outcome::Lost,

            _ => Outcome::Draw,
        }
    }
}

impl Round for Round1 {
    fn play(&self) -> usize {
        let outcome = self.eval();
        outcome.score() + self.1.score()
    }
}

impl FromStr for Round1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let player_1 = parts.next().ok_or(Error::InvalidRound(s.to_string()))?;
        let player_2 = parts.next().ok_or(Error::InvalidRound(s.to_string()))?;

        Ok(Round1(player_1.parse()?, player_2.parse()?))
    }
}

impl Round for Round2 {
    fn play(&self) -> usize {
        let should_play = match (&self.0, &self.1) {
            (Deck::Rock, Outcome::Win) => Deck::Paper,
            (Deck::Rock, Outcome::Lost) => Deck::Scissor,
            (Deck::Rock, Outcome::Draw) => Deck::Rock,

            (Deck::Paper, Outcome::Win) => Deck::Scissor,
            (Deck::Paper, Outcome::Lost) => Deck::Rock,
            (Deck::Paper, Outcome::Draw) => Deck::Paper,

            (Deck::Scissor, Outcome::Win) => Deck::Rock,
            (Deck::Scissor, Outcome::Lost) => Deck::Paper,
            (Deck::Scissor, Outcome::Draw) => Deck::Scissor,
        };

        self.1.score() + should_play.score()
    }
}

impl FromStr for Round2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let player_1 = parts.next().ok_or(Error::InvalidRound(s.to_string()))?;
        let player_2 = parts.next().ok_or(Error::InvalidRound(s.to_string()))?;

        Ok(Round2(player_1.parse()?, player_2.parse()?))
    }
}

struct StrategyGuide<R: Round>(Vec<R>);

impl<R: Round> StrategyGuide<R> {
    fn evaluate(&self) -> usize {
        self.0.iter().map(|r| r.play()).sum()
    }
}

impl<R: Round + FromStr> TryFrom<Vec<String>> for StrategyGuide<R>
where
    R: FromStr<Err = Error>,
{
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let rounds: Vec<R> = value
            .into_iter()
            .map(|v| v.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(rounds))
    }
}

struct Day2;

fn solve<R: Round>(lines: Vec<String>) -> super::SolverResult
where
    StrategyGuide<R>: TryFrom<Vec<String>>,
    <StrategyGuide<R> as TryFrom<Vec<String>>>::Error: std::error::Error + 'static,
{
    let strategy_guide =
        StrategyGuide::<R>::try_from(lines).map_err(|e| SolverError::Generic(e.into()))?;
    Ok(strategy_guide.evaluate().to_string())
}

impl Solver for Day2 {
    fn name(&self) -> &'static str {
        "Rock Paper Scissors"
    }

    fn solve_part1(&self, lines: Vec<String>) -> super::SolverResult {
        solve::<Round1>(lines)
    }

    fn solve_part2(&self, lines: Vec<String>) -> super::SolverResult {
        solve::<Round2>(lines)
    }

    fn test_expected(&self, part: usize) -> &'static str {
        match part {
            1 => "15",
            2 => "12",
            _ => unreachable!(),
        }
    }
}

pub(super) fn new() -> Box<dyn Solver> {
    Box::new(Day2)
}
