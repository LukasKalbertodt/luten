use rand::{Rng, thread_rng};
use std::collections::HashMap;

use types::*;
use WorkDay::*;

#[macro_export]
macro_rules! instance {
    (
        tutors: {
            $(
                $tname:expr, $scale:expr => [
                    $( $tutor_slots:tt )*
                ];
            )*
        }
        students: {
            $(
                $sname:expr, $partner:expr => [
                    $( $student_slots:tt )*
                ];
            )*
        }
    ) => {{
        let mut tutors = Vec::new();

        $(
            {
                let (good, tolerable): (Vec<Timeslot>, Vec<Timeslot>) = instance! {
                    @parse_timeslots $( $tutor_slots )*
                };

                tutors.push(Tutor {
                    name: $tname.into(),
                    slot_assignment: SlotAssignment::new(&good, &tolerable),
                    scale_factor: $scale,
                });
            }
        )*

        let mut students = Vec::new();

        $(
            {
                let (good, tolerable): (Vec<Timeslot>, Vec<Timeslot>) = instance! {
                    @parse_timeslots $( $student_slots )*
                };

                students.push(Student {
                    name: $sname.into(),
                    slot_assignment: SlotAssignment::new(&good, &tolerable),
                    partner: $partner.map(|s: &'static str| s.to_string()),
                });
            }
        )*

        Instance {
            students: students,
            tutors: tutors,
        }

    }};
    (
        @parse_timeslots $( $rating:ident: $day:ident $slot:expr, )*
    ) => {{
        let mut good = Vec::new();
        let mut tolerable = Vec::new();
        $(
            {
                let slot = Timeslot {
                    day: WorkDay::$day,
                    slot_of_day: $slot,
                };

                match SlotRating::$rating {
                    SlotRating::Good => good.push(slot),
                    SlotRating::Tolerable => tolerable.push(slot),
                    _ => {}
                }
            }
        )*

        (good, tolerable)
    }}
}


pub fn small_instance0() -> Instance {
    instance! {
        tutors: {
            "Tobias", 1.0 => [
                Good: Monday 0,
                Tolerable: Monday 1,
            ];
            "Karo", 1.0 => [
                Good: Monday 2,
                Tolerable: Monday 3,
            ];
        }
        students: {
            "Susi", Some("Willi") => [
                Good: Monday 1,
            ];
            "Willi", Some("Susi") => [
                Good: Monday 2,
                Good: Monday 3,
                Tolerable: Monday 1,
            ];
            "Lisa", None => [
                Good: Monday 0,
                Good: Monday 1,
            ];
        }
    }
    // expected solution:
    // Monday 0: Tobias - Lisa
    // Monday 1: Tobias - Willi, Susi
}

pub fn small_instance1() -> Instance {
    instance! {
        tutors: {
            "T1", 1.0 => [
                Good: Tuesday 0,
                Tolerable: Monday 0,
            ];
            "T2", 1.0 => [
                Good: Wednesday 0,
                Tolerable: Tuesday 0,
            ];
        }
        students: {
            "S1", Some("S2") => [
                Good: Wednesday 0,
                Tolerable: Tuesday 0,
            ];
            "S2", Some("S1") => [
                Good: Thursday 0,
                Tolerable: Wednesday 0,
            ];
            "S3", None => [
                Good: Tuesday 0,
                Good: Wednesday 0,
            ];
            "S4", None => [
                Good: Monday 0,
                Good: Tuesday 0,
                Good: Wednesday 0,
            ];
        }
    }
    // expected solution:
    // Tuesday:     T1 - S3, S4
    // Wednesday:   T2 - S1, S2
}

pub fn random_instance(no_of_students: u16, no_of_tutors: u16) -> Instance {
    // TODO: add paramater to adjust whether a large, normal or small amount of
    // slots are selected by students and tutors

    /// Generates a `ŚlotAssignment` where x = 4 * `good_blocks` + `good_slots`
    /// slots have been marked as `SlotRating::Good`
    /// and y = 4 * `tolerable_blocks` + `tolerable_slots`
    /// slots have been marked as `SlotRating::Tolerable`
    /// and where z = `good_blocks` + `tolerable_blocks`
    /// blocks of 4 coherent slots appear
    fn fill_slots<R: Rng>(
        rng: &mut R,
        good_blocks: u8,
        tolerable_blocks: u8,
        good_slots: u8,
        tolerable_slots: u8
    ) -> SlotAssignment {

        let mut map = HashMap::new();

        {
            let mut add = |no, block, rating| {
                for _ in 0..no {
                    loop {
                        let slots: Vec<_> = if block {
                            (0..5).map(|n| n * 4).collect()
                        } else {
                            (0..20).collect()
                        };
                        let slot = Timeslot {
                            day: *(rng.choose(&[Monday, Tuesday, Wednesday]).unwrap()),
                            slot_of_day: *(rng.choose(&slots).unwrap()),
                        };

                        if !map.contains_key(&slot) {
                            map.insert(slot, rating);
                            if block {
                                for i in 1..4 {
                                    let new_slot = Timeslot {
                                        day: slot.day,
                                        slot_of_day: slot.slot_of_day + i,
                                    };
                                    map.insert(new_slot, rating);
                                }
                            }
                            break;
                        }
                    }

                }
            };


            add(good_blocks, true, SlotRating::Good);
            add(tolerable_blocks, true, SlotRating::Tolerable);
            add(good_slots, false, SlotRating::Good);
            add(tolerable_slots, false, SlotRating::Tolerable);
        }

        SlotAssignment {
            ratings: map,
        }
    }

    let mut rng = thread_rng();

    let tutors: Vec<_> = (0..no_of_tutors).map(|t| {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);

        Tutor {
            name: t.to_string(),
            slot_assignment: fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots),
            scale_factor: if rng.gen_weighted_bool(10) { 2.0 } else { 1.0 }
        }
    }).collect();



    let mut students = Vec::new();

    // Student pairs - they have the same SlotAssignment
    let pairs = rng.gen_range(no_of_students / 10, no_of_students / 2);
    for p in 0..pairs {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);
        let slot_assignment = fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots);

        students.push(Student {
            name: (p * 2).to_string(),
            slot_assignment: slot_assignment.clone(),
            partner: Some(((p * 2) + 1).to_string()),
        });
        students.push(Student {
            name: ((p * 2) + 1).to_string(),
            slot_assignment: slot_assignment,
            partner: Some((p * 2).to_string()),
        });
    }

    // 'Single' students
    students.extend(((pairs * 2)..no_of_students).map(|s| {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);

        Student {
            name: s.to_string(),
            slot_assignment: fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots),
            partner: None,
        }
    }));

    Instance {
        students: students,
        tutors: tutors,
    }
}
