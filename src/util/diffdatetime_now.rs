use chrono::{DateTime, FixedOffset, Utc};
use num_integer::div_rem;
use serde::{Deserialize, Serialize};
use std::fmt;

const SECS_IN_YEAR: i64 = 31536000;
const SECS_IN_MONTH: i64 = 2628288;
const SECS_IN_DAY: i64 = 86400;
const SECS_IN_HOUR: i64 = 3600;
const SECS_IN_MIN: i64 = 60;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffDateTimeNow {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl fmt::Display for DiffDateTimeNow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut info = String::new();

        if self.years != 0 {
            info.push_str(&format!(" {} years", self.years));
        }
        if self.months != 0 {
            info.push_str(&format!(" {} months", self.months));
        }
        if self.days != 0 {
            info.push_str(&format!(" {} days", self.days));
        }
        if self.hours != 0 {
            info.push_str(&format!(" {} hours", self.hours));
        }
        if self.minutes != 0 {
            info.push_str(&format!(" {} min", self.minutes));
        }
        if self.seconds != 0 {
            info.push_str(&format!(" {} sec", self.seconds));
        }
        write!(f, "{}", info.trim())
    }
}

impl DiffDateTimeNow {
    pub fn new(rfc3339_string: String) -> Self {
        let created_at = DateTime::parse_from_rfc3339(&rfc3339_string).unwrap();
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        let tmp = (now - created_at).num_seconds();
        let secs = tmp;

        let (years, remainder) = div_rem(secs, SECS_IN_YEAR);
        let (months, remainder) = div_rem(remainder, SECS_IN_MONTH);
        let (days, remainder) = div_rem(remainder, SECS_IN_DAY);
        let (hours, remainder) = div_rem(remainder, SECS_IN_HOUR);
        let (minutes, remainder) = div_rem(remainder, SECS_IN_MIN);
        let seconds = remainder;

        Self {
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
        }
    }
}
