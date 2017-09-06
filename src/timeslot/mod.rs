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
