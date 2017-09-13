use rand::{Rng, thread_rng};
use rand::distributions::{IndependentSample, Sample};
use rand::distributions::normal::Normal;
use rand::distributions::range::Range;
use std::collections::HashMap;

use types::*;
use WorkDay::*;
use util::clamp;

pub struct RatingDistribution<T, U, V> where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
{
    pub available_blocks_per_day: u16,
    // distribution used to sample overall number of slots rated as `Good` or `Tolerable`
    pub rated_slots: T,
    // proportion of slots rated good vs tolerable
    pub good_slot_percentage: U,
    // percentage of slots rated together as a block of four slots
    pub block_percentage: V,
}

impl RatingDistribution<Normal, Range<f64>, Range<f64>> {
    pub fn default() -> Self {
        use rand::distributions::normal::Normal;
        use rand::distributions::range::Range;
        Self {
            available_blocks_per_day: 4,
            rated_slots: Normal::new(10.0, 2.5),
            good_slot_percentage: Range::new(0.6, 0.9),
            block_percentage: Range::new(0.5, 0.7),
        }
    }
}

impl<T, U, V> Sample<SlotAssignment> for RatingDistribution<T, U, V> where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
{
    fn sample<R: Rng>(&mut self, _: &mut R) -> SlotAssignment {
        unimplemented!()
    }
}

impl<T, U, V> IndependentSample<SlotAssignment> for RatingDistribution<T, U, V> where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
{
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> SlotAssignment {

        /// Generates a `ŚlotAssignment` where x = 4 * `good_blocks` + `good_slots`
        /// slots have been marked as `SlotRating::Good`
        /// and y = 4 * `tolerable_blocks` + `tolerable_slots`
        /// slots have been marked as `SlotRating::Tolerable`
        /// and where z = `good_blocks` + `tolerable_blocks`
        /// blocks of 4 coherent slots appear
        fn fill_slots<R: Rng>(
            rng: &mut R,
            good_blocks: u64,
            tolerable_blocks: u64,
            good_slots: u64,
            tolerable_slots: u64,
            available_blocks_per_day: u16,
        ) -> SlotAssignment {

            let mut map = HashMap::new();

            {
                let mut add = |no, block, rating| {
                    for _ in 0..no {
                        loop {
                            let slots: Vec<_> = if block {
                                (0..available_blocks_per_day).map(|n| n * 4).collect()
                            } else {
                                (0..(available_blocks_per_day * 4)).collect()
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

        // TODO: make sure that sampled values make sense (no negative slot amounts,
        // percentages between 0 and 1)
        let total_available_slots = self.available_blocks_per_day * 4 * 3;
        let slot_no = {
            let slot_no = self.rated_slots.ind_sample(rng).round();
            clamp(slot_no, 1.0, total_available_slots as f64) as u64
        };

        let good_slots = {
            let good_slots = (self.good_slot_percentage.ind_sample(rng) * (slot_no as f64)).round();
            clamp(good_slots, 1.0, total_available_slots as f64) as u64
        };
        let tolerable_slots = slot_no - good_slots;

        let block_percentage_sample = {
            let percentage = self.good_slot_percentage.ind_sample(rng);
            clamp(percentage, 0.0, 1.0)
        };
        let good_blocks = ((good_slots as f64) * block_percentage_sample / 4.0).round() as u64;
        let good_single_slots = good_slots - (good_blocks * 4);
        let tolerable_blocks = ((tolerable_slots as f64) * block_percentage_sample / 4.0).round() as u64;
        let tolerable_single_slots = tolerable_slots - (tolerable_blocks * 4);

        fill_slots(rng, good_blocks, tolerable_blocks, good_single_slots, tolerable_single_slots, self.available_blocks_per_day)
    }
}

pub fn random_instance<T, U, V, W, X, Y>(
    no_of_students: u64,
    no_of_tutors: u64,
    student_rating_distribution: &mut RatingDistribution<T, U, V>,
    tutor_rating_distribution: &mut RatingDistribution<W, X, Y>,
    team_percentage: f64,
) -> Instance where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
    W: IndependentSample<f64>,
    X: IndependentSample<f64>,
    Y: IndependentSample<f64>,
{
    assert!(team_percentage >= 0.0 && team_percentage <= 1.0);

    let mut rng = thread_rng();

    let tutors: Vec<_> = (0..no_of_tutors).map(|t| {
        Tutor {
            name: t.to_string(),
            slot_assignment: tutor_rating_distribution.ind_sample(&mut rng),
            scale_factor: if rng.gen_weighted_bool(10) { 2.0 } else { 1.0 }
        }
    }).collect();


    let mut students = Vec::new();

    // Student pairs - they have the same SlotAssignment
    let pairs = ((no_of_students as f64) * team_percentage).round() as u64;
    for p in 0..pairs {
        let slot_assignment = student_rating_distribution.ind_sample(&mut rng);

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
        Student {
            name: s.to_string(),
            slot_assignment: student_rating_distribution.ind_sample(&mut rng),
            partner: None,
        }
    }));

    Instance {
        students: students,
        tutors: tutors,
    }
}
