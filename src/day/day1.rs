use thiserror::Error;

use super::{Solver, SolverError, SolverResult};

use std::{result::Result, str::FromStr};

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid ration. Expected numeric value, got {0}")]
    InvalidRation(String),

    #[error("Not enough elfs {0}, expedition is over")]
    TooLitleElfs(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Ration(pub usize);

impl FromStr for Ration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse().map_err(|_| Error::InvalidRation(s.to_string()))?,
        ))
    }
}

#[derive(Debug)]
struct Elf {
    rations: Vec<Ration>,
}

impl Elf {
    fn total_calories(&self) -> usize {
        self.rations.iter().map(|r| r.0).sum()
    }
}

impl TryFrom<Vec<String>> for Elf {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let rations: Vec<Ration> = value
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rations })
    }
}

fn partition(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut partitions = Vec::new();
    let mut current = Vec::new();

    for item in lines.into_iter() {
        if item.is_empty() {
            partitions.push(current.clone());
            current.clear();
        } else {
            current.push(item);
        }
    }

    if !current.is_empty() {
        partitions.push(current);
    }

    partitions
}

fn read_elfs(lines: Vec<String>) -> Result<Vec<Elf>, Error> {
    partition(lines)
        .into_iter()
        .map(Elf::try_from)
        .collect::<Result<Vec<_>, _>>()
}

struct Day1;

impl Solver for Day1 {
    fn name(&self) -> &'static str {
        "Calorie Counting"
    }

    fn solve_part1(&self, lines: Vec<String>) -> SolverResult {
        let elfs = read_elfs(lines).map_err(|e| SolverError::Generic(e.into()))?;
        Ok(elfs
            .iter()
            .map(|e| e.total_calories())
            .max()
            .expect("should have at least one elf")
            .to_string())
    }

    fn solve_part2(&self, lines: Vec<String>) -> SolverResult {
        let mut elfs = read_elfs(lines).map_err(|e| SolverError::Generic(e.into()))?;
        elfs.sort_by(|e1, e2| e2.total_calories().cmp(&e1.total_calories()));

        let top_three = elfs
            .get(0..3)
            .ok_or(Error::TooLitleElfs(elfs.len()))
            .map_err(|e| SolverError::Generic(e.into()))?;

        Ok(top_three
            .iter()
            .map(|e| e.total_calories())
            .sum::<usize>()
            .to_string())
    }

    fn test_expected(&self, part: usize) -> &'static str {
        match part {
            1 => "24000",
            2 => "45000",
            _ => unreachable!(),
        }
    }
}

pub(super) fn new() -> Box<dyn Solver> {
    Box::new(Day1)
}
