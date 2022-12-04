use std::{collections::HashSet, str::FromStr};

use thiserror::Error;

use super::{Solver, SolverError};

#[derive(Debug, Error)]
enum Error {
    #[error("invalid item {0}")]
    InvalidItem(char),
}

fn distance(c1: char, c2: char) -> usize {
    (c2 as isize - c1 as isize).abs() as usize
}

const TOTAL_COMPARTMENTS: usize = 2;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Item(char);

impl Item {
    fn priority(&self) -> Option<usize> {
        match self.0 {
            'a'..='z' => Some(1 + distance(self.0, 'a')),
            'A'..='Z' => Some(27 + distance(self.0, 'A')),
            _ => None,
        }
    }
}

impl TryFrom<char> for Item {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' | 'A'..='Z' => Ok(Self(value)),
            _ => Err(Error::InvalidItem(value)),
        }
    }
}

#[derive(Debug)]
struct Compartment(HashSet<Item>);

impl Compartment {
    fn common(&self, other: &Compartment) -> Vec<Item> {
        self.0.intersection(&other.0).copied().collect()
    }
}

impl FromStr for Compartment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        ))
    }
}

#[derive(Debug)]
struct Rucksack([Compartment; TOTAL_COMPARTMENTS]);

impl Rucksack {
    fn duplicated(&self) -> Vec<Item> {
        self.0[0].common(&self.0[1])
    }

    fn items(&self) -> HashSet<Item> {
        let mut all_items = self.0[0].0.clone();
        all_items.extend(&self.0[1].0);
        all_items
    }
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let half = len / 2;
        let first_half = &s[..half];
        let second_half = &s[half..];

        Ok(Self([first_half.parse()?, second_half.parse()?]))
    }
}

struct Day3;

impl Solver for Day3 {
    fn name(&self) -> &'static str {
        "Rucksack Reorganization"
    }

    fn solve_part1(&self, lines: Vec<String>) -> super::SolverResult {
        let rucksacks: Vec<Rucksack> = lines
            .into_iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, Error>>()
            .map_err(|e| SolverError::Generic(e.into()))?;

        let common_items = rucksacks
            .iter()
            .filter_map(|r| r.duplicated().first().copied());

        Ok(common_items
            .filter_map(|i| i.priority())
            .sum::<usize>()
            .to_string())
    }

    fn solve_part2(&self, lines: Vec<String>) -> super::SolverResult {
        let rucksacks: Vec<Rucksack> = lines
            .into_iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, Error>>()
            .map_err(|e| SolverError::Generic(e.into()))?;

        let groups = rucksacks.as_slice().chunks(3);
        Ok(groups
            .into_iter()
            .filter_map(|g| {
                let first = g[0].items();
                let second = g[1].items();
                let third = g[2].items();

                let common = &first & &second;
                let badge = &common & &third;

                badge.iter().next().and_then(Item::priority)
            })
            .sum::<usize>()
            .to_string())
    }

    fn test_expected(&self, part: usize) -> &'static str {
        match part {
            1 => "157",
            2 => "70",
            _ => unreachable!(),
        }
    }
}

pub(super) fn new() -> Box<dyn Solver> {
    Box::new(Day3)
}
