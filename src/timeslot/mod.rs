use std::fmt;
use std::cmp::Ordering;
use std::str::FromStr;

use chrono::{self, Duration, NaiveTime};
use diesel;
use diesel::prelude::*;

use config;
use dict::{self, Locale};
use db::Db;
use db::schema::timeslots;
use errors::*;

/// Day of the week.
///
/// We create our own type instead of using `chrono::Weekday` to implement
/// diesel-traits for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    /// Returns the name of the day.
    pub fn full_name(&self, locale: Locale) -> String {
        use self::DayOfWeek::*;

        let dict = dict::new(locale).timeslot;
        match *self {
            Monday => dict.monday_full(),
            Tuesday => dict.tuesday_full(),
            Wednesday => dict.wednesday_full(),
            Thursday => dict.thursday_full(),
            Friday => dict.friday_full(),
            Saturday => dict.saturday_full(),
            Sunday => dict.sunday_full(),
        }
    }

    /// Returns an abbreviation of the day name (usually 2 or 3 letters).
    pub fn short_name(&self, locale: Locale) -> String {
        use self::DayOfWeek::*;

        let dict = dict::new(locale).timeslot;
        match *self {
            Monday => dict.monday_short(),
            Tuesday => dict.tuesday_short(),
            Wednesday => dict.wednesday_short(),
            Thursday => dict.thursday_short(),
            Friday => dict.friday_short(),
            Saturday => dict.saturday_short(),
            Sunday => dict.sunday_short(),
        }
    }

    pub fn all_variant_strs() -> &'static [&'static str] {
        &[
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ]
    }

    pub fn from_variant_str(s: &str) -> Option<Self> {
        match s {
            "Monday" => Some(DayOfWeek::Monday),
            "Tuesday" => Some(DayOfWeek::Tuesday),
            "Wednesday" => Some(DayOfWeek::Wednesday),
            "Thursday" => Some(DayOfWeek::Thursday),
            "Friday" => Some(DayOfWeek::Friday),
            "Saturday" => Some(DayOfWeek::Saturday),
            "Sunday" => Some(DayOfWeek::Sunday),
            _ => None,
        }
    }
}

impl fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Insertable)]
#[table_name = "timeslots"]
pub struct NewTimeSlot {
    day: DayOfWeek,
    time: NaiveTime,
}

impl NewTimeSlot {
    pub fn new(day: DayOfWeek, time: Time) -> Self {
        Self {
            day,
            time: time.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, Queryable)]
pub struct TimeSlot {
    id: i16,
    day: DayOfWeek,
    time: NaiveTime,
}

impl PartialOrd for TimeSlot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.day.cmp(&other.day)
                .then(self.time.cmp(&other.time))
        )
    }
}

impl TimeSlot {
    /// Loads and returns the timeslot with the given id from the database.
    pub fn load_from_id(id: i16, db: &Db) -> Result<Option<Self>> {
        timeslots::table
            .find(id)
            .first::<Self>(&*db.conn()?)
            .optional()?
            .make_ok()
    }

    /// Returns all timeslots form the database.
    pub fn load_all(db: &Db) -> Result<Vec<Self>> {
        timeslots::table
            .load(&*db.conn()?)
            .chain_err(|| "unable to load timeslots from DB")
    }

    /// Creates a new timeslot with the given data and stores it in the
    /// database.
    pub fn create(day: DayOfWeek, time: Time, db: &Db) -> Result<Self> {
        let new_timeslot = NewTimeSlot { day, time: time.0 };

        diesel::insert(&new_timeslot)
            .into(timeslots::table)
            .get_result::<Self>(&*db.conn()?)?
            .make_ok()
    }

    /// Inserts all given timeslots into the database.
    pub fn create_all(timeslots: &[NewTimeSlot], db: &Db) -> Result<()> {
        diesel::insert(timeslots)
            .into(timeslots::table)
            .execute(&*db.conn()?)
            .map_err(|e| -> Error { e.into() })
            .and_then(|inserted_rows| {
                if inserted_rows != timeslots.len() {
                    Err("inserted_rows doesn't match the input len!".into())
                } else {
                    Ok(())
                }
            })
    }

    /// Counts the number of timeslots in the database.
    pub fn count(db: &Db) -> Result<u64> {
        timeslots::table
            .count()
            .get_result::<i64>(&*db.conn()?)
            .chain_err(|| "failed to count number of timeslots in DB")
            .map(|count| count as u64)
    }

    /// Deletes the timeslot with the given id from the database. Returns
    /// `true` if it has been deleted, `false` otherwise (which probably means
    /// the the id wasn't found).
    pub fn delete(id: i16, db: &Db) -> Result<bool> {
        diesel::delete(timeslots::table.find(id))
            .execute(&*db.conn()?)
            .map(|changes_rows| changes_rows == 1)
            .chain_err(|| "failed to delete timeslot from DB")
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn time(&self) -> Time {
        Time(self.time)
    }

    pub fn day(&self) -> DayOfWeek {
        self.day
    }
}


impl From<chrono::Weekday> for DayOfWeek {
    fn from(c: chrono::Weekday) -> Self {
        use chrono::Weekday::*;
        use self::DayOfWeek::*;

        match c {
            Mon => Monday,
            Tue => Tuesday,
            Wed => Wednesday,
            Thu => Thursday,
            Fri => Friday,
            Sat => Saturday,
            Sun => Sunday,
        }
    }
}

impl Into<chrono::Weekday> for DayOfWeek {
    fn into(self) -> chrono::Weekday {
        use chrono::Weekday::*;
        use self::DayOfWeek::*;

        match self {
            Monday => Mon,
            Tuesday => Tue,
            Wednesday => Wed,
            Thursday => Thu,
            Friday => Fri,
            Saturday => Sat,
            Sunday => Sun,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Time(NaiveTime);

impl Time {
    pub fn next(&self) -> Self {
        Time(self.0 + Duration::minutes(config::TIMESLOT_LEN.into()))
    }

    pub fn prev(&self) -> Self {
        Time(self.0 - Duration::minutes(config::TIMESLOT_LEN.into()))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.format("%H:%M").fmt(f)
    }
}

impl FromStr for Time {
    type Err = String;

    /// Parses strings of the form "HH:MM" where "MM" is a multiple of
    /// `config::TIMESLOT_LEN`.
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        use chrono::prelude::*;

        let time = NaiveTime::parse_from_str(s, "%H:%M")
            .map_err(|e| e.to_string())?;

        if time.minute() % (config::TIMESLOT_LEN as u32) != 0 {
            return Err(format!(
                "The minutes of a timeslot have to be a multiple of {}!",
                config::TIMESLOT_LEN,
            ));
        }

        Ok(Time(time))
    }
}

pub fn parse_time_interval(s: &str) -> StdResult<Vec<Time>, String> {
    let parts: Vec<_> = s.splitn(2, '-').collect();
    if parts.len() == 1 {
        Ok(vec![parts[0].trim().parse()?])
    } else {
        let start: Time = parts[0].trim().parse()?;
        let end: Time = parts[1].trim().parse()?;

        if start >= end {
            return Err("start has to be smaller than the end!".into());
        }

        let mut slot = start;
        let mut out = Vec::new();
        while slot < end {
            out.push(slot);
            slot.0 = slot.0 + Duration::minutes(config::TIMESLOT_LEN.into());
        }

        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    Good,
    Tolerable,
    Bad,
}
