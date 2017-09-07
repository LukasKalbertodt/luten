//! Routes and functions for everything in the state "Preparation". **Has
//! routes.**

use diesel;
use diesel::prelude::*;


mod html;
pub mod routes;


use db::Db;
use db::schema::prep_student_preferences;
use errors::*;
use user::Student;


/// Preferences by a student, set by the student during the preparation state.
#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "prep_student_preferences"]
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
}
