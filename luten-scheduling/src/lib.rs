extern crate rand;

pub mod types;
pub mod instances;

use std::collections::HashSet;

use types::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Team {
    Single(Student),
    Full(Student, Student),
}

impl Team {
    fn all_students<F>(&self, mut f: F) -> bool where
        F: FnMut(&Student) -> bool,
    {
        match *self {
            Team::Single(ref s) => f(s),
            Team::Full(ref s1, ref s2) => f(s1) && f(s2),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Solution {
    testats: Vec<(Timeslot, Tutor, Team)>,
}



pub fn solve(_instance: &Instance) -> Solution {
    unimplemented!()
}


pub fn is_valid_solution(instance: &Instance, solution: &Solution) -> bool {
    let no_student_missing = instance.students.iter()
        .all(|s| solution.testats.iter().find(|testat| {
            match testat.2 {
                Team::Single(ref s1) => s1 == s,
                Team::Full(ref s1, ref s2) => s1 == s || s2 == s,
            }
        }).is_some());



    let no_student_double = {
        let mut map = HashSet::new();
        solution.testats.iter()
            .all(|t| t.2.all_students(|s| map.insert(s.name.clone())))
    };

    let tutors_without_time_turners = {
        let mut map = HashSet::new();
        solution.testats.iter().all(|t| map.insert((t.0, &t.1.name)))
    };

    let _preferred_partners_old = instance.students.iter().all(|ref s| {
        if let Some(ref p) = s.partner {
            solution.testats.iter().find(|testat| {
                if let Team::Full(ref s1, ref s2) = testat.2 {
                    (*s1 == **s && s2.name == *p) || (s1.name == *p && *s2 == **s)
                } else {
                    false
                }
            }).is_some()
        } else {
            true
        }
    });

    let preferred_partners = solution.testats.iter().all(|testat| {
        testat.2.all_students(|s| {
            if let Some(ref preferred) = s.partner {
                match testat.2 {
                    Team::Single(_) => false,
                    Team::Full(ref s1, ref s2) => *preferred == *s1.name || *preferred == *s2.name,
                }
            } else {
                true
            }
        })
    });

    let fitting_timeslots = solution.testats.iter().all(|testat| {
        testat.2.all_students(|s| s.slot_assignment.rating_for(testat.0).is_ok()) &&
            testat.1.slot_assignment.rating_for(testat.0).is_ok()
    });


    no_student_missing && no_student_double && tutors_without_time_turners && preferred_partners && fitting_timeslots
}
