use std::{fmt::Display, ops::{Add, AddAssign, Sub}};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub from: Position,
    pub until: Delta,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Delta {
    pub lines: usize,
    pub columns: usize,
}

impl From<&str> for Delta {
    fn from(s: &str) -> Self {
        let mut lines = s.rsplit('\n');
        Self {
            columns: lines.next().unwrap().len(),
            lines: lines.count(),
        }
    }
}

impl Add<Delta> for Position {
    type Output = Position;

    fn add(mut self, Delta { lines, columns }: Delta) -> Self::Output {
        if lines > 0 {
            self.line += lines;
            self.column = columns;
        } else {
            self.column += columns;
        }
        self
    }
}

impl Sub for Position {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.line > rhs.line {
            Delta {
                lines: self.line - rhs.line,
                columns: self.column,
            }
        } else if self.line == rhs.line {
            Delta {
                lines: 0,
                columns: self.column - rhs.column,
            }
        } else {
            unreachable!()
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.from + self.until)
    }
}

impl Add for Range {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            from: self.from,
            until: rhs.to() - self.from,
        }
    }
}

impl AddAssign for Range {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<Position> for Range {
    fn from(from: Position) -> Self {
        Self {
            from,
            until: Delta::default()
        }
    }
}

impl Delta {
    pub fn nonzero(self) -> bool {
        self > Delta::default()
    }
}

impl Range {
    pub fn to(self) -> Position {
        self.from + self.until
    }
}
