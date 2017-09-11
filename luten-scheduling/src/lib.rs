extern crate rand;

pub mod types;
pub mod instances;

use types::*;




#[derive(Clone, PartialEq, Debug)]
pub struct Solution {
    testats: Vec<(Timeslot, Tutor, Student)>,
}

pub fn solve(instance: &Instance) -> Solution {


    unimplemented!()
}


pub fn is_valid_solution(instance: &Instance, solution: &Solution) -> bool {
    //let no_student_missing = instance.students.iter()
    //    .all(|s| solution.testats.iter().find(|testat| testat.2 == s).is_some());
    false
}
