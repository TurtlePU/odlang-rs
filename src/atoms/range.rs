use std::{fmt::{Display, Formatter}, ops::{Add, Sub}};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub from: Position,
    pub until: Delta,
}

#[derive(Clone, Copy, Debug, Default)]
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
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.line, self.column)
    }
}
