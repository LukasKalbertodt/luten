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
    /// Number used to compute the total amount of available slots that the tutors / students
    /// are able to rate
    pub available_blocks_per_day: u16,
    /// Distribution used to sample the overall number of slots rated as `Good` or `Tolerable`
    pub rated_slots_distribution: T,
    /// Distribution used to sample the proportion of slots rated `Good` às opposed to `Tolerable`
    pub good_slot_percentage_distribution: U,
    /// Distribution used to sample the percentage of slots rated together as a block of four slots
    pub block_percentage_distribution: V,
}

impl RatingDistribution<Normal, Range<f64>, Range<f64>> {
    pub fn of_average_student() -> Self {
        use rand::distributions::normal::Normal;
        use rand::distributions::range::Range;
        Self {
            available_blocks_per_day: 4,
            rated_slots_distribution: Normal::new(10.0, 2.5),
            good_slot_percentage_distribution: Range::new(0.6, 0.9),
            block_percentage_distribution: Range::new(0.5, 0.7),
        }
    }
}

impl<T, U, V> Sample<SlotAssignment> for RatingDistribution<T, U, V> where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
{
    fn sample<R: Rng>(&mut self, _: &mut R) -> SlotAssignment {
        // The `Sample` trait for some reason is a prerequisite for the `IndependentSample` trait,
        // but I have no clue why or how to implement it. Therefore this function will stay
        // unimplemented!() and should not be used.
        unimplemented!()
    }
}

impl<T, U, V> IndependentSample<SlotAssignment> for RatingDistribution<T, U, V> where
    T: IndependentSample<f64>,
    U: IndependentSample<f64>,
    V: IndependentSample<f64>,
{
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> SlotAssignment {
        // it is assumed that there are testats on 3 days of the week, and one block consists of
        // four slots.
        let available_slots = self.available_blocks_per_day * 4 * 3;
        let rated_slots = {
            let rated_slots = self.rated_slots_distribution.ind_sample(rng).round();
            clamp(rated_slots, 1.0, available_slots as f64) as u64
        };

        let good_slots = {
            let good_slots = (self.good_slot_percentage_distribution.ind_sample(rng) * (rated_slots as f64)).round();
            clamp(good_slots, 1.0, available_slots as f64) as u64
        };
        let tolerable_slots = rated_slots - good_slots;

        let block_percentage = {
            let percentage = self.block_percentage_distribution.ind_sample(rng);
            clamp(percentage, 0.0, 1.0)
        };
        let good_blocks = ((good_slots as f64) * block_percentage / 4.0).round() as u64;
        let good_single_slots = good_slots - (good_blocks * 4);
        let tolerable_blocks = ((tolerable_slots as f64) * block_percentage / 4.0).round() as u64;
        let tolerable_single_slots = tolerable_slots - (tolerable_blocks * 4);

        let mut ratings = HashMap::new();
        {
            // For single slot ratings (`block` = false), insert `amount` randomly chosen slots with
            // the rating `rating` into the HashMap of Ratings, choosing from
            // 3 * 4 * `available_blocks_per_day` slots. For block ratings (`block` = true), rate
            // 4 * `amount` slots in coherent groups of four slots each. This closure should be called
            // first for block ratings, then for single slot ratings.
            let mut rate_slots = |amount: u64, block: bool, rating: SlotRating| {
                for _ in 0..amount {
                    loop {
                        let slots: Vec<_> = if block {
                            (0..self.available_blocks_per_day).map(|n| n * 4).collect()
                        } else {
                            (0..(self.available_blocks_per_day * 4)).collect()
                        };
                        let slot = Timeslot {
                            day: *(rng.choose(&[Monday, Tuesday, Wednesday]).unwrap()),
                            slot_of_day: *(rng.choose(&slots).unwrap()),
                        };

                        if !ratings.contains_key(&slot) {
                            ratings.insert(slot, rating);
                            // for block ratings, also rate the following 3 slots to complete the block
                            if block {
                                for i in 1..4 {
                                    let new_slot = Timeslot {
                                        day: slot.day,
                                        slot_of_day: slot.slot_of_day + i,
                                    };
                                    ratings.insert(new_slot, rating);
                                }
                            }
                            break;
                        }
                    }
                }
            };

            rate_slots(good_blocks, true, SlotRating::Good);
            rate_slots(tolerable_blocks, true, SlotRating::Tolerable);
            rate_slots(good_single_slots, false, SlotRating::Good);
            rate_slots(tolerable_single_slots, false, SlotRating::Tolerable);
        }
        SlotAssignment {
            ratings: ratings,
        }
    }
}

/// Returns a randomly generated instance with `no_of_students` students and `no_of_tutors` tutors.
/// The `student_rating_distribution` contains probability distributions that are used to sample
/// a `SlotRating` for each generated student. The same for `tutor_rating_distribution` for the
/// tutors. With 0 <=`team_percentage` <= 1 one can adust how many students have preferred partners
/// that they want to be in a team with vs. how many students are 'single'.
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
