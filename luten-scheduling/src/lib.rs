extern crate chrono;

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
    day: WorkDay,
    slot_of_day: u16,
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
    name: String,
    slot_assignment: SlotAssignment,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Student {
    name: String,
    slot_assignment: SlotAssignment,
}



#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instance {
    students: Vec<Student>,
    tutors: Vec<Tutor>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Solution {
    testats: Vec<(Timeslot, Tutor, Student)>,
}

pub fn solve(instance: &Instance) -> Solution {


    unimplemented!()
}


pub fn is_valid_solution(instance: &Instance, solution: &Solution) -> bool {
    let no_student_missing = instance.students.iter()
        .all(|s| solutions.testats.iter().find(|testat| testat.2 == s).is_some());
}
