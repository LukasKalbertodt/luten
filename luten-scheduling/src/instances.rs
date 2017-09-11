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

    let i1 = Instance {
        students: vec![student1, student2, student3],
        tutors: vec![tutor1, tutor2],
    };

    vec![i1]
}
