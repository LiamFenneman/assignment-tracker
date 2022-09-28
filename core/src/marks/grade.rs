use std::fmt;

use crate::mark::Percent;

/// A letter grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    A(Option<Modifier>),
    B(Option<Modifier>),
    C(Option<Modifier>),
    D,
    E,
    F,
}

/// Convert from a percent to a letter grade.
/// TODO: use types instead of the `u8` values.
/// TODO: allow for custom grade ranges.
const fn pct_to_grade(pct: u8) -> Grade {
    match pct {
        90..=100 => Grade::A(Some(Modifier::Plus)),
        85..=89 => Grade::A(None),
        80..=84 => Grade::A(Some(Modifier::Minus)),
        75..=79 => Grade::B(Some(Modifier::Plus)),
        70..=74 => Grade::B(None),
        65..=69 => Grade::B(Some(Modifier::Minus)),
        60..=64 => Grade::C(Some(Modifier::Plus)),
        55..=59 => Grade::C(None),
        50..=54 => Grade::C(Some(Modifier::Minus)),
        40..=49 => Grade::D,
        1..=39 => Grade::E,
        0 => Grade::F,
        _ => unreachable!(),
    }
}

/// Convert from a letter grade to a percentage.
/// TODO: use types instead of the `u8` values.
const fn grade_to_pct(grade: Grade) -> u8 {
    match grade {
        Grade::A(Some(Modifier::Plus)) => 90,
        Grade::A(None) => 85,
        Grade::A(Some(Modifier::Minus)) => 80,
        Grade::B(Some(Modifier::Plus)) => 75,
        Grade::B(None) => 70,
        Grade::B(Some(Modifier::Minus)) => 65,
        Grade::C(Some(Modifier::Plus)) => 60,
        Grade::C(None) => 55,
        Grade::C(Some(Modifier::Minus)) => 50,
        Grade::D => 40,
        Grade::E => 20,
        Grade::F => 0,
    }
}

impl From<Percent> for Grade {
    fn from(percent: Percent) -> Self {
        pct_to_grade(percent.value())
    }
}

impl From<Grade> for Percent {
    fn from(grade: Grade) -> Self {
        Percent::new(grade_to_pct(grade)).expect("invalid conversion")
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grade::A(None) => write!(f, "A"),
            Grade::B(None) => write!(f, "B"),
            Grade::C(None) => write!(f, "C"),
            Grade::D => write!(f, "D"),
            Grade::E => write!(f, "E"),
            Grade::F => write!(f, "F"),
            Grade::A(Some(Modifier::Plus)) => write!(f, "A+"),
            Grade::B(Some(Modifier::Plus)) => write!(f, "B+"),
            Grade::C(Some(Modifier::Plus)) => write!(f, "C+"),
            Grade::A(Some(Modifier::Minus)) => write!(f, "A-"),
            Grade::B(Some(Modifier::Minus)) => write!(f, "B-"),
            Grade::C(Some(Modifier::Minus)) => write!(f, "C-"),
        }
    }
}

/// A letter grade modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    Plus,
    Minus,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::Minus => write!(f, "-"),
        }
    }
}
