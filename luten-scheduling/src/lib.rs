extern crate rand;

pub mod instances;
pub mod types;
pub mod util;

use std::collections::HashSet;

use types::*;

pub fn solve(_instance: &Instance) -> Solution {
    unimplemented!()
}


pub fn is_valid_solution(instance: &Instance, solution: &Solution) -> Result<(), Vec<String>> {
    let mut errs = Vec::new();

    let no_student_missing = instance.students.iter()
        .all(|s| solution.testats.iter()
            .find(|testat| testat.team.contains(s))
            .is_some());
    if !no_student_missing {
        errs.push("Some students from the instance are missing in the solution.".into());
    }


    let no_student_double = {
        let mut map = HashSet::new();
        solution.testats.iter()
            .all(|t| t.team.all_students(|s| map.insert(s.name.clone())))
    };
    if !no_student_double {
        errs.push("Some students occur more than once in the solution.".into());
    }


    let tutors_without_time_turners = {
        let mut map = HashSet::new();
        solution.testats.iter()
            .all(|t| map.insert((t.slot, &t.tutor.name)))
    };
    if !tutors_without_time_turners {
        errs.push("Some tutors have more than one Testat at the same time. Unfortunately the \
            university does not provide time turners. =(".into());
    }


    let preferred_partners = solution.testats.iter()
        .all(|testat| {
            testat.team.all_students(|s| {
                if let Some(ref preferred) = s.partner {
                    match testat.team {
                        Team::Single(_) => false,
                        Team::Full(ref s1, ref s2) => *preferred == *s1.name || *preferred == *s2.name,
                    }
                } else {
                    true
                }
            })
        });
    if !preferred_partners {
        errs.push("Some teams were ripped apart by the algorithm.".into())
    }


    let fitting_timeslots = solution.testats.iter()
        .all(|testat| {
            testat.team.all_students(|s| s.slot_assignment.rating_for(testat.slot).is_ok()) &&
            testat.tutor.slot_assignment.rating_for(testat.slot).is_ok()
        });
    if !fitting_timeslots {
        errs.push("Some people were allocated Timeslots that are not fitting.".into())
    }


    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs)
    }
}
