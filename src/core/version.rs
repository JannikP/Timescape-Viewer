use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u64,
    minor: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn bump_major(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
        }
    }

    pub fn bump_minor(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
        }
    }

    pub fn is_major_update_compared_to(&self, older: &Self) -> bool {
        self.major > older.major
    }

    pub fn is_minor_update_compared_to(&self, older: &Self) -> bool {
        self.major == older.major && self.minor > older.minor
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => self.minor.cmp(&other.minor),
            ord => ord,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}
