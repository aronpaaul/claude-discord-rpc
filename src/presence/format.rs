use crate::config::{Currency, Settings, TokenStyle};

pub struct ValueFormatter;

impl ValueFormatter {
    pub fn tokens(count: u64, style: TokenStyle) -> String {
        match style {
            TokenStyle::Full => count.to_string(),
            TokenStyle::Spaced => Self::grouped(count),
            TokenStyle::Compact => Self::compact(count),
        }
    }

    pub fn duration(millis: i64) -> String {
        let totalSeconds = (millis / 1000).max(0);
        let hours = totalSeconds / 3600;
        let minutes = totalSeconds % 3600 / 60;
        let seconds = totalSeconds % 60;
        if hours > 0 {
            format!("{hours}ч {minutes:02}м")
        } else if minutes > 0 {
            format!("{minutes}м {seconds:02}с")
        } else {
            format!("{seconds}с")
        }
    }

    pub fn cost(usd: f64, settings: &Settings) -> String {
        match settings.currency {
            Currency::Usd => format!("${:.2}", usd),
            Currency::Rub => format!("{:.0} ₽", usd * settings.usdToRubRate as f64),
            Currency::Eur => format!("€{:.2}", usd * settings.usdToEurRate as f64),
        }
    }

    fn compact(count: u64) -> String {
        let value = count as f64;
        if value >= 1_000_000_000.0 {
            format!("{:.2}B", value / 1_000_000_000.0)
        } else if value >= 1_000_000.0 {
            format!("{:.2}M", value / 1_000_000.0)
        } else if value >= 1_000.0 {
            format!("{:.1}k", value / 1_000.0)
        } else {
            count.to_string()
        }
    }

    fn grouped(count: u64) -> String {
        let digits = count.to_string();
        let mut result = String::new();
        for (index, digit) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index) % 3 == 0 {
                result.push(' ');
            }
            result.push(digit);
        }
        result
    }
}
