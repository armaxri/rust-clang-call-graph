use serde::Deserialize;
use serde::Serialize;

use super::location::Location;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

impl Range {
    pub fn new(start: Location, end: Location) -> Self {
        Range { start, end }
    }

    pub fn is_location_within_range(&self, location: &Location) -> bool {
        if location.is_location_same_or_after(&self.start)
            && self.end.is_location_same_or_after(location)
        {
            return true;
        }
        false
    }

    pub fn is_within_range_of(&self, other: &Range) -> bool {
        if other.start.is_location_same_or_after(&self.start)
            && self.end.is_location_same_or_after(&other.end)
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_location_within_range() {
        let range = Range::new(Location::new(1, 1), Location::new(2, 2));
        let mut other = Location::new(1, 1);
        assert!(range.is_location_within_range(&other));
        other = Location::new(2, 2);
        assert!(range.is_location_within_range(&other));
        other = Location::new(1, 2);
        assert!(range.is_location_within_range(&other));
        other = Location::new(0, 0);
        assert!(!range.is_location_within_range(&other));
        other = Location::new(3, 3);
        assert!(!range.is_location_within_range(&other));
        other = Location::new(2, 3);
        assert!(!range.is_location_within_range(&other));
    }

    #[test]
    fn test_is_within_range_of() {
        let range = Range::new(Location::new(1, 1), Location::new(2, 2));
        let mut other = Range::new(Location::new(1, 1), Location::new(2, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Location::new(1, 1), Location::new(1, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Location::new(2, 1), Location::new(2, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Location::new(1, 1), Location::new(2, 1));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Location::new(0, 0), Location::new(2, 2));
        assert!(!range.is_within_range_of(&other));
        other = Range::new(Location::new(1, 1), Location::new(3, 3));
        assert!(!range.is_within_range_of(&other));
    }
}
