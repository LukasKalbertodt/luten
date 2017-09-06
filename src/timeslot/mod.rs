use chrono;

use dict::{self, Locale};

/// Day of the week.
///
/// We create our own type instead of using `chrono::Weekday` to implement
/// diesel-traits for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
