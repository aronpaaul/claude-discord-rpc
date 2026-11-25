use std::time::{SystemTime, UNIX_EPOCH};

pub struct IsoTime;

impl IsoTime {
    pub fn parseMillis(text: &str) -> Option<i64> {
        let number = |from: usize, to: usize| text.get(from..to)?.parse::<i64>().ok();
        let year = number(0, 4)?;
        let month = number(5, 7)?;
        let day = number(8, 10)?;
        let hour = number(11, 13)?;
        let minute = number(14, 16)?;
        let second = number(17, 19)?;
        let hasFraction = text.as_bytes().get(19) == Some(&b'.');
        let fraction = if hasFraction { number(20, 23).unwrap_or(0) } else { 0 };
        let days = Self::daysFromCivil(year, month, day);
        Some((days * 86400 + hour * 3600 + minute * 60 + second) * 1000 + fraction)
    }

    pub fn nowMillis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_millis() as i64)
            .unwrap_or(0)
    }

    fn daysFromCivil(year: i64, month: i64, day: i64) -> i64 {
        let shiftedYear = if month <= 2 { year - 1 } else { year };
        let era = shiftedYear.div_euclid(400);
        let yearOfEra = shiftedYear - era * 400;
        let shiftedMonth = if month > 2 { month - 3 } else { month + 9 };
        let dayOfYear = (153 * shiftedMonth + 2) / 5 + day - 1;
        let dayOfEra = yearOfEra * 365 + yearOfEra / 4 - yearOfEra / 100 + dayOfYear;
        era * 146097 + dayOfEra - 719468
    }
}
