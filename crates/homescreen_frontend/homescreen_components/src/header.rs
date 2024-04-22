use crate::prelude::*;
use chrono::{DateTime, Datelike, Local, Timelike, Weekday};
use dioxus::prelude::*;
use std::{fmt::Display, time::Duration};

const HOURS: [&str; 24] = [
    "Midnight", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    "Eleven", "Midday", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    "Ten", "Eleven",
];
const MINUTES: [&str; 12] = [
    "O'Clock",
    "Five past",
    "Ten past",
    "Quater past",
    "Twenty past",
    "Twenty-five past",
    "Half past",
    "Twenty-five to",
    "Twenty to",
    "Quater to",
    "Ten to",
    "Five to",
];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Clone, Default)]
pub struct Time {
    hour: u32,
    minute: u32,
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
impl From<DateTime<Local>> for Time {
    fn from(value: DateTime<Local>) -> Self {
        Self {
            hour: value.hour(),
            minute: value.minute(),
        }
    }
}
impl Time {
    pub fn new(hour: u32, minute: u32) -> Option<Self> {
        if (0..=23).contains(&hour) && (0..=59).contains(&minute) {
            Some(Self { hour, minute })
        } else {
            None
        }
    }
    pub fn new_unchecked(hour: u32, minute: u32) -> Self {
        assert!((0..=23).contains(&hour));
        assert!((0..=59).contains(&minute));
        Self { hour, minute }
    }

    ///
    /// ```rust
    /// # use homescreen_components::prelude::*;
    /// let time = Time::new_unchecked(0, 0);
    /// assert_eq!(time.format(), "Midnight".to_string());
    ///
    /// let time = Time::new_unchecked(12, 0);
    /// assert_eq!(time.format(), "Midday".to_string());
    ///
    /// let time = Time::new_unchecked(2, 0);
    /// assert_eq!(time.format(), "Two O'Clock".to_string());
    ///
    /// let time = Time::new_unchecked(2, 30);
    /// assert_eq!(time.format(), "Half past Two");
    ///
    /// let time = Time::new_unchecked(2, 26);
    /// assert_eq!(time.format(), "Twenty-five past Two");
    ///
    /// let time = Time::new_unchecked(2, 45);
    /// assert_eq!(time.format(), "Quater to Three");
    ///
    /// let time = Time::new_unchecked(23, 55);
    /// assert_eq!(time.format(), "Five to Midnight");
    /// ```
    pub fn format(&self) -> String {
        match (self.hour(), self.minute() / 5) {
            (0 | 12, 0) => HOURS[self.hour() as usize].into(),
            (hour, 0) => format!("{} {}", HOURS[hour as usize], MINUTES[0]),
            (hour, min) if min <= 6 => {
                format!("{} {}", MINUTES[min as usize], HOURS[hour as usize])
            }
            (hour, min) if hour != 23 => {
                format!("{} {}", MINUTES[min as usize], HOURS[hour as usize + 1])
            }
            (_, min) => format!("{} {}", MINUTES[min as usize], HOURS[0]),
        }
        .to_string()
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }
    pub fn minute(&self) -> u32 {
        self.minute
    }
}

pub struct Date {
    weekday: Weekday,
    day: u32,
    month: u32,
    year: i32,
}
impl From<DateTime<Local>> for Date {
    fn from(value: DateTime<Local>) -> Self {
        Self {
            weekday: value.weekday(),
            day: value.day(),
            month: value.month0(),
            year: value.year(),
        }
    }
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
impl Date {
    pub fn display(&self) -> String {
        let str_weekday = self.weekday.to_string();
        let (weekday, _) = str_weekday.split_at(3);
        let day = self.day;
        let month = MONTHS[self.month as usize];
        let year = self.year;

        format!("{weekday} {day} {month} {year}")
    }

    pub fn day(&self) -> u32 {
        self.day
    }
    pub fn month(&self) -> u32 {
        self.month
    }
    pub fn year(&self) -> i32 {
        self.year
    }
}

pub fn Header() -> Element {
    let mut time = use_signal(|| Time::from(Local::now()));
    let mut date = use_signal(|| Date::from(Local::now()));

    use_interval(Duration::from_millis(100), move || {
        time.set(Local::now().into());
        date.set(Local::now().into());
    });

    rsx!(
        header {
            section {
                id: "date-time",
                h2 {
                    class: "sr-only",
                    { "Date &amp; Time" }
                }
                p {
                    id: "time",
                    { format!("{time}") }
                }
                hr {}
                p {
                    id: "date",
                    { format!("{date}") }
                }
            }
        }
    )
}
