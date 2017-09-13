pub mod random;

use types::*;

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
