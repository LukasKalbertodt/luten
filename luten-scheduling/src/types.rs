use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum WorkDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SlotRating {
    Good,
    Tolerable,
    NotFitting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Timeslot {
    pub day: WorkDay,
    pub slot_of_day: u16,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlotAssignment {
    ratings: HashMap<Timeslot, SlotRating>,
}

impl SlotAssignment {
    pub fn new(good_slots: &[Timeslot], tolerable_slots: &[Timeslot]) -> Self {
        assert!(!good_slots.is_empty());

        let mut ratings = HashMap::new();
        ratings.extend(good_slots.iter().map(|&ts| (ts, SlotRating::Good)));
        ratings.extend(tolerable_slots.iter().map(|&ts| (ts, SlotRating::Tolerable)));

        // If this is false, there has been keys in both, `good_slots` and
        // `tolerable_slots`. This is not allowed.
        assert_eq!(good_slots.len() + tolerable_slots.len(), ratings.len());

        Self { ratings }
    }

    pub fn rating_for(&self, slot: Timeslot) -> SlotRating {
        self.ratings.get(&slot).cloned().unwrap_or(SlotRating::NotFitting)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tutor {
    pub name: String,
    pub slot_assignment: SlotAssignment,
    pub english_testats: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Student {
    pub name: String,
    pub slot_assignment: SlotAssignment,
    pub prefers_english: bool,
    pub partner: Option<String>,
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instance {
    pub students: Vec<Student>,
    pub tutors: Vec<Tutor>,
}
