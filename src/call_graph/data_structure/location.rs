use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }

    pub fn is_before(&self, other: &Location) -> bool {
        if self.line < other.line {
            return true;
        } else if self.line == other.line {
            return self.column < other.column;
        }
        false
    }

    pub fn is_location_same_or_after(&self, other: &Location) -> bool {
        return !self.is_before(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_before() {
        let location = Location::new(1, 1);
        let mut other = Location::new(2, 2);
        assert!(location.is_before(&other));
        other = Location::new(1, 1);
        assert!(!location.is_before(&other));
        other = Location::new(0, 0);
        assert!(!location.is_before(&other));
    }

    #[test]
    fn test_is_location_same_or_after() {
        let location = Location::new(1, 1);
        let mut other = Location::new(0, 0);
        assert!(location.is_location_same_or_after(&other));
        other = Location::new(1, 0);
        assert!(location.is_location_same_or_after(&other));
        other = Location::new(1, 1);
        assert!(location.is_location_same_or_after(&other));
        other = Location::new(2, 2);
        assert!(!location.is_location_same_or_after(&other));
    }
}
