//! Routes and functions for everything in the state "Preparation". **Has
//! routes.**

use diesel;
use diesel::prelude::*;


mod html;
pub mod routes;


use db::Db;
use db::schema::{prep_student_preferences, timeslots, timeslot_ratings};
use errors::*;
use timeslot::{Rating, TimeSlot};
use user::{Student, User};


/// Preferences by a student, set by the student during the preparation state.
#[derive(Debug, Clone, Identifiable, Insertable, Queryable)]
#[table_name = "prep_student_preferences"]
#[primary_key(user_id)]
pub struct StudentPreferences {
    user_id: i64,
    pub partner: Option<String>,
    pub prefers_english: bool,
}

impl StudentPreferences {
    /// Loads the preferences of the given student.
    ///
    /// This assumes that a row for the given student already exists. As such
    /// an error is returned when it doesn't.
    pub fn load_for(user: &Student, db: &Db) -> Result<Self> {
        prep_student_preferences::table
            .find(user.id())
            .first::<Self>(&*db.conn()?)
            .chain_err(|| "student with non-existing preferences found")
    }

    /// Creates a new preferences-object from the given data and inserts it
    /// into the database. The inserted object is returned.
    pub fn create(
        user: &Student,
        partner: Option<String>,
        prefers_english: bool,
        db: &Db,
    ) -> Result<Self> {
        let new_entry = Self {
            user_id: user.id(),
            partner,
            prefers_english,
        };

        diesel::insert(&new_entry)
            .into(prep_student_preferences::table)
            .get_result::<Self>(&*db.conn()?)
            .chain_err(|| "failed to insert new StudentPreferences")
    }

    /// Creates a default preference-object and inserts it into the database.
    ///
    /// Note that the default values are hardcoded here for now. Later, in the
    /// bright future, we will make preferences much more customizable.
    pub fn create_default(user: &Student, db: &Db) -> Result<Self> {
        Self::create(user, None, false, db)
    }

    /// Updates the database with this value.
    pub fn update(&self, db: &Db) -> Result<()> {
        diesel::update(prep_student_preferences::table.find(self.user_id))
            .set((
                prep_student_preferences::columns::partner.eq(&self.partner),
                prep_student_preferences::columns::prefers_english.eq(&self.prefers_english),
            ))
            .execute(&*db.conn()?)
            .map_err(|e| -> Error { e.into() })
            .and_then(|affected_rows| {
                if affected_rows != 1 {
                    Err("number of affected rows != 1".into())
                } else {
                    Ok(())
                }
            })
            .chain_err(|| "failed to update student prep preferences")
    }
}

#[derive(Debug, Clone, Copy, Identifiable, Insertable, Queryable)]
#[table_name = "timeslot_ratings"]
#[primary_key(user_id, timeslot_id)]
pub struct TimeSlotRating {
    user_id: i64,
    timeslot_id: i16,
    rating: Rating,
}

impl TimeSlotRating {
    /// Loads a specific rating of a given user and a given timeslot.
    pub fn load(user: &User, timeslot_id: i16, db: &Db) -> Result<Option<(TimeSlot, Rating)>> {
        timeslot_ratings::table.find((user.id(), timeslot_id))
            .inner_join(timeslots::table)
            .get_result::<(Self, TimeSlot)>(&*db.conn()?)
            .optional()
            .chain_err(|| "failed to load timeslot rating from DB")
            .map(|opt| opt.map(|(r, slot)| (slot, r.rating)))
    }

    /// Loads all ratings of the given user.
    pub fn load_all_of_user(user: &User, db: &Db) -> Result<Vec<(TimeSlot, Rating)>> {
        let ratings: Vec<_> = timeslot_ratings::table
            .filter(timeslot_ratings::columns::user_id.eq(user.id()))
            .inner_join(timeslots::table)
            .get_results::<(Self, TimeSlot)>(&*db.conn()?)?
            .into_iter()
            .map(|(r, slot)| (slot, r.rating))
            .collect();

        //  We make sure the user has a rating for each existing timeslot: if
        //  there are ratings missing, we create default entries. Actually,
        //  this shouldn't be necessary: one user creation, default entries are
        //  created. The following code is only useful if timeslots are added
        //  after users are created.
        if ratings.len() as u64 != TimeSlot::count(&db)? {
            Self::create_defaults_for_user(&user, &db)?;
            TimeSlotRating::load_all_of_user(&user, &db)
        } else {
            Ok(ratings)
        }
    }

    /// Inserts a "Bad" rating for all existing timeslots for which the given
    /// user has no rating yet. Returns `true` if at least one rating was
    /// inserted, `false` otherwise.
    pub fn create_defaults_for_user(user: &User, db: &Db) -> Result<bool> {
        // First, find all timeslots for which the given user has no rating by
        // finding all timeslots and the slots for which the user has a
        // rating...
        let all_timeslots = TimeSlot::load_all(db)?;
        let my_ratings = timeslot_ratings::table
            .filter(timeslot_ratings::columns::user_id.eq(user.id()))
            .select(timeslot_ratings::columns::timeslot_id)
            .get_results::<i16>(&*db.conn()?)?;

        // ... and then filter out those timeslots for which the user already
        // has a rating (basically a naive set complement).
        let new_ratings: Vec<_> = all_timeslots.into_iter()
            .filter(|slot| !my_ratings.contains(&slot.id()))
            .map(|slot| {
                TimeSlotRating {
                    user_id: user.id(),
                    timeslot_id: slot.id(),
                    rating: Rating::Bad,
                }
            })
            .collect();

        // Insert the list of new ratings into the database.
        diesel::insert(&new_ratings)
            .into(timeslot_ratings::table)
            .execute(&*db.conn()?)
            .map(|insert_count| insert_count > 0)
            .chain_err(|| "failed to insert timeslot ratings into DB")
    }

    /// Updates all given timeslots with the given ratings.
    pub fn update_all(user: &User, ratings: &[(i16, Rating)], db: &Db) -> Result<()> {
        // Yeah, we execute one query per time slot here... Maybe we should
        // change this.
        let conn = &*db.conn()?;
        for &(slot_id, rating) in ratings {
            diesel::update(timeslot_ratings::table.find((user.id(), slot_id)))
                .set(timeslot_ratings::columns::rating.eq(rating))
                .execute(conn)?;
        }

        Ok(())
    }
}
