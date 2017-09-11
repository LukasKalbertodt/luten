use rand::{Rng, thread_rng};
use std::collections::HashMap;

use types::*;
use WorkDay::*;

pub fn small_instances() -> Vec<Instance> {
    let mut slots = Vec::new();
    for i in 0..5 {
        slots.push(Timeslot {
            day: Monday,
            slot_of_day: i,
        });
    }

    let tutor1 = Tutor {
        name: "tobias".into(),
        slot_assignment: SlotAssignment::new(&[slots[0]], &[slots[1]]),
        english_testats: true,
    };
    let tutor2 = Tutor {
        name: "karo".into(),
        slot_assignment: SlotAssignment::new(&[slots[2]], &[slots[3]]),
        english_testats: false,
    };

    let student1 = Student {
        name: "susi".into(),
        slot_assignment: SlotAssignment::new(&[slots[1]], &[]),
        prefers_english: false,
        partner: Some("willi".into()),
    };

    let student2 = Student {
        name: "willi".into(),
        slot_assignment: SlotAssignment::new(&[slots[2], slots[3]], &[slots[1]]),
        prefers_english: false,
        partner: Some("susi".into()),
    };

    let student3 = Student {
        name: "lisa".into(),
        slot_assignment: SlotAssignment::new(&[slots[0], slots[1]], &[]),
        prefers_english: true,
        partner: None,
    };

    // expected best solution:
    // Monday slot 0: Tobias - Lisa
    // Monday slot 1: Tobias - Willi, Susi

    let i1 = Instance {
        students: vec![student1, student2, student3],
        tutors: vec![tutor1, tutor2],
    };



    vec![i1]
}

pub fn random_instance(no_of_students: u16, no_of_tutors: u16) -> Instance {

    fn fill_slots<R: Rng>(rng: &mut R, good_blocks: u8, tolerable_blocks: u8, good_slots: u8, tolerable_slots: u8)
        -> SlotAssignment {

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

    let mut tutors = Vec::new();

    for t in 0..no_of_tutors {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);
            tutors.push(Tutor {
            name: t.to_string(),
            slot_assignment: fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots),
            english_testats: rng.gen_weighted_bool(3),
        });
    }

    let mut students = Vec::new();
    let pairs = rng.gen_range(no_of_students / 10, no_of_students / 2);

    for p in 0..pairs {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);
        let slot_assignment = fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots);
        let english = rng.gen_weighted_bool(10);

        students.push(Student {
            name: (p * 2).to_string(),
            slot_assignment: slot_assignment.clone(),
                prefers_english: english,
                partner: Some(((p * 2) + 1).to_string()),
        });
        students.push(Student {
            name: ((p * 2) + 1).to_string(),
            slot_assignment: slot_assignment,
            prefers_english: english,
            partner: Some((p * 2).to_string()),
        });
    }

    for s in (pairs * 2)..no_of_students {
        let good_blocks = rng.gen_range(2, 4);
        let tolerable_blocks = rng.gen_range(0, 2);
        let good_slots = rng.gen_range(0, 5);
        let tolerable_slots = rng.gen_range(0, 5);
        students.push(Student {
            name: s.to_string(),
            slot_assignment: fill_slots(&mut rng, good_blocks, tolerable_blocks, good_slots, tolerable_slots),
            prefers_english: rng.gen_weighted_bool(10),
            partner: None,
        });
    }

    Instance {
        students: students,
        tutors: tutors,
    }
}
