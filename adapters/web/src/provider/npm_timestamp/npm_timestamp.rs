//! `npm_timestamp.rs` — npm timestamp.
//! Parse timestamp npm.

/// Parse npm `time` values (`2026-09-18T12:00:00.000Z` or without millis)
/// to unix seconds. Returns None when unparseable (caller keeps version).
/// (Parse thời gian npm ra unix seconds.)
pub(crate) fn parse_npm_time(raw: &str) -> Option<u64> {
    let normalized = raw.trim_end_matches('Z');
    let (date, time) = normalized.split_once('T')?;
    let mut date_parts = date.split('-');
    let (year, month, day) = (
        date_parts.next()?.parse::<u64>().ok()?,
        date_parts.next()?.parse::<u64>().ok()?,
        date_parts.next()?.parse::<u64>().ok()?,
    );
    let time = time.split('.').next()?;
    let mut time_parts = time.split(':');
    let (hh, mm, ss) = (
        time_parts.next()?.parse::<u64>().ok()?,
        time_parts.next()?.parse::<u64>().ok()?,
        time_parts.next()?.parse::<u64>().ok()?,
    );
    if year < 1970 || day == 0 || day > days_in_month(year, month)? || hh > 23 || mm > 59 || ss > 60
    {
        return None;
    }
    // Days-from-civil (Howard Hinnant) — no chrono dependency here.
    // (Ngày từ dân sự — không cần chrono.)
    let adjusted_year = if month <= 2 {
        year.checked_sub(1)?
    } else {
        year
    };
    let era = adjusted_year / 400;
    let year_of_era = adjusted_year % 400;
    let month_prime = if month > 2 {
        month.checked_sub(3)?
    } else {
        month.checked_add(9)?
    };
    let day_of_year = 153_u64
        .checked_mul(month_prime)?
        .checked_add(2)?
        .checked_div(5)?
        .checked_add(day.checked_sub(1)?)?;
    let day_of_era = year_of_era
        .checked_mul(365)?
        .checked_add(year_of_era / 4)?
        .checked_sub(year_of_era / 100)?
        .checked_add(day_of_year)?;
    let days_since_epoch = era
        .checked_mul(146_097)?
        .checked_add(day_of_era)?
        .checked_sub(719_468)?;
    days_since_epoch
        .checked_mul(86_400)?
        .checked_add(hh.checked_mul(3_600)?)?
        .checked_add(mm.checked_mul(60)?)?
        .checked_add(ss)
}

/// Apply Gregorian leap-year rules / Áp dụng quy tắc năm nhuận Gregorian.
fn is_leap_year(year: u64) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

/// Return the valid day count for a month / Trả số ngày hợp lệ theo tháng.
fn days_in_month(year: u64, month: u64) -> Option<u64> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}
