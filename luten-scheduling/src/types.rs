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

impl SlotRating {
    pub fn is_ok(&self) -> bool {
        match *self {
            SlotRating::NotFitting => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Timeslot {
    pub day: WorkDay,
    pub slot_of_day: u16,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlotAssignment {
    pub ratings: HashMap<Timeslot, SlotRating>,
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

#[derive(Clone, PartialEq, Debug)]
pub struct Tutor {
    pub name: String,
    pub slot_assignment: SlotAssignment,
    pub scale_factor: f32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Student {
    pub name: String,
    pub slot_assignment: SlotAssignment,
    pub partner: Option<String>,
}


#[derive(Clone, PartialEq, Debug)]
pub struct Instance {
    pub students: Vec<Student>,
    pub tutors: Vec<Tutor>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Team {
    Single(Student),
    Full(Student, Student),
}

impl Team {
    pub fn all_students<F>(&self, mut f: F) -> bool where
        F: FnMut(&Student) -> bool,
    {
        match *self {
            Team::Single(ref s) => f(s),
            Team::Full(ref s1, ref s2) => f(s1) && f(s2),
        }
    }

    pub fn contains(&self, s: &Student) -> bool {
        match *self {
            Team::Single(ref s1) => s1 == s,
            Team::Full(ref s1, ref s2) => s1 == s || s2 == s,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Testat {
    pub slot: Timeslot,
    pub tutor: Tutor,
    pub team: Team,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Solution {
    pub testats: Vec<Testat>,
}
