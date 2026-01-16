use chrono::{DateTime, Utc};

use crate::common::error::ValidationError;

#[derive(Debug, Clone)]
pub struct TimeSlot {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl TimeSlot {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, ValidationError> {
        if end <= start {
            return Err(ValidationError::EndBeforeStart);
        }
        Ok(Self { start, end })
    }

    pub fn new_future(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, ValidationError> {
        if start < Utc::now() {
            return Err(ValidationError::ReservationInPast);
        }
        Self::new(start, end)
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn overlaps(&self, other: &TimeSlot) -> bool {
        self.start < other.end && self.end > other.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_valid_time_slot() {
        let start = Utc::now() + Duration::hours(1);
        let end = start + Duration::hours(1);
        let slot = TimeSlot::new(start, end);
        assert!(slot.is_ok());
    }

    #[test]
    fn test_invalid_time_slot_end_before_start() {
        let start = Utc::now() + Duration::hours(2);
        let end = Utc::now() + Duration::hours(1);
        let slot = TimeSlot::new(start, end);
        assert!(matches!(slot, Err(ValidationError::EndBeforeStart)));
    }

    #[test]
    fn test_overlapping_slots() {
        let now = Utc::now();
        let slot1 = TimeSlot::new(now, now + Duration::hours(2)).unwrap();
        let slot2 = TimeSlot::new(now + Duration::hours(1), now + Duration::hours(3)).unwrap();
        assert!(slot1.overlaps(&slot2));
    }

    #[test]
    fn test_non_overlapping_slots() {
        let now = Utc::now();
        let slot1 = TimeSlot::new(now, now + Duration::hours(1)).unwrap();
        let slot2 = TimeSlot::new(now + Duration::hours(2), now + Duration::hours(3)).unwrap();
        assert!(!slot1.overlaps(&slot2));
    }
}
