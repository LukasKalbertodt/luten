extern crate rand;

pub mod types;
pub mod instances;

use types::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Team {
    Single(Student),
    Full(Student, Student),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Solution {
    testats: Vec<(Timeslot, Tutor, Team)>,
}



pub fn solve(instance: &Instance) -> Solution {


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

    no_student_missing
}
