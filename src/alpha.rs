use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct AlphaGen(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Alpha(usize);

impl From<usize> for Alpha {
    fn from(num: usize) -> Self {
        Self(num)
    }
}

impl Display for Alpha {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "_{}", self.0)
    }
}

impl AlphaGen {
    pub fn next(&mut self) -> Alpha {
        self.0 += 1;
        Alpha(self.0)
    }
}
