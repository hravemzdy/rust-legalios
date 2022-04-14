use std::cmp::{max, min};
use crate::service::period::IPeriod;
use chrono::{Datelike, NaiveDate, Weekday};

#[allow(dead_code)]
const TERM_BEG_FINISHED: i16 = 32;

#[allow(dead_code)]
const TERM_END_FINISHED: i16 = 0;

#[allow(dead_code)]
const WEEKSUN_SUNDAY: i16 = 0;

#[allow(dead_code)]
const WEEKMON_SUNDAY: i16 = 7;

#[allow(dead_code)]
const TIME_MULTIPLY_SIXTY: i32 = 60;

#[allow(dead_code)]
const WEEKDAYS_COUNT: i16 = 7;

#[allow(dead_code)]
pub(crate) fn length_of_month(date: NaiveDate) -> i16 {
    let next_month_date = NaiveDate::from_ymd(
        match date.month() {
            12 => date.year() + 1,
            _ => date.year(),
        },
        match date.month() {
            12 => 1,
            _ => date.month() + 1,
        },
        1,
    );
    return next_month_date.signed_duration_since(date).num_days() as i16;
}

#[allow(dead_code)]
pub(crate) fn empty_month_schedule() -> Vec<i32> {
    return vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
}

#[allow(dead_code)]
pub(crate) fn total_weeks_hours(template: Vec<i32>) -> i32 {
    let result = template.iter().take(7).fold(0, |agr, x| agr + x);

    return result
}

#[allow(dead_code)]
pub(crate) fn total_month_hours(template: Vec<i32>) -> i32 {
    let result = template.iter().take(31).fold(0,|agr, x| agr + x);

    return result;
}

#[allow(dead_code)]
pub(crate) fn days_in_month(period: &dyn IPeriod) -> i16 {
    let date: NaiveDate = NaiveDate::from_ymd(period.year() as i32, period.month() as u32, 1);
    return length_of_month(date);
}

#[allow(dead_code)]
pub(crate) fn date_of_month(period: &dyn IPeriod, day_ordinal: i16) -> NaiveDate
{
    let period_day: i16 = min(max(1, day_ordinal), days_in_month(period));

    return NaiveDate::from_ymd(period.year() as i32, period.month() as u32, period_day as u32);
}

#[allow(dead_code)]
pub(crate) fn day_of_week_mon_to_sun(period_date_cwd: Weekday) -> i16 {
    // DayOfWeek Sunday = 0,
    // Monday = 1, Tuesday = 2, Wednesday = 3, Thursday = 4, Friday = 5, Saturday = 6,
    return match period_date_cwd {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    };
}

#[allow(dead_code)]
pub(crate) fn day_of_week_from_ordinal(day_ordinal: i16, period_begin_cwd: i16) -> i16 {
    // day_ordinal 1..31
    // period_begin_cwd 1..7
    // day_of_week 1..7

    let day_of_week = (((day_ordinal - 1) + (period_begin_cwd - 1)) % 7) + 1;

    return day_of_week;
}

#[allow(dead_code)]
pub(crate) fn week_day_of_month(period: &dyn IPeriod, day_ordinal: i16) -> i16 {
    let period_date = date_of_month(period, day_ordinal);

    let period_date_cwd = period_date.weekday();

    return day_of_week_mon_to_sun(period_date_cwd);
}

#[allow(dead_code)]
pub(crate) fn date_from_in_period(period: &dyn IPeriod, date_from: Option<NaiveDate>) -> i16 {
    let period_date_beg = NaiveDate::from_ymd(period.year() as i32, period.month() as u32, 1);

    let day_term_from = match date_from {
        Some(value) => {
            if value < period_date_beg {
                return 1;
            }
            return value.day() as i16;
        },
        None => 1,
    };
    return day_term_from;
}

#[allow(dead_code)]
pub(crate) fn date_stop_in_period(period: &dyn IPeriod, date_ends: Option<NaiveDate>) -> i16 {
    let days_period = days_in_month(period);

    let period_date_end = NaiveDate::from_ymd(period.year() as i32, period.month() as u32, days_period as u32);
    let day_term_end = match date_ends {
        Some(value) => {
            if value > period_date_end {
                return days_period;
            }
            return value.day() as i16;
        },
        None => days_period,
    };
    return day_term_end;
}

#[allow(dead_code)]
pub(crate) fn timesheet_week_schedule(_period: &dyn IPeriod, seconds_weekly: i32, workdays_weekly: i16) -> Vec<i32> {
    let seconds_daily = seconds_weekly / min(workdays_weekly, WEEKDAYS_COUNT) as i32;

    let sec_remainder = seconds_weekly - (seconds_daily * workdays_weekly as i32);

    let week_schedule = (1..=7).map(|x| week_day_seconds(x, workdays_weekly, seconds_daily, sec_remainder)).collect();

    return week_schedule;
}

#[allow(dead_code)]
pub(crate) fn week_day_seconds(day_ordinal: i16, days_of_work: i16, seconds_daily: i32, sec_remainder: i32) -> i32 {
    return match day_ordinal {
        d if d < days_of_work => seconds_daily,
        d if d == days_of_work => seconds_daily + sec_remainder,
        _ => 0,
    };
}

#[allow(dead_code)]
pub(crate) fn timesheet_full_schedule(period: &dyn IPeriod, week_schedule: &Vec<i32>) -> Vec<i32> {
    let period_days_count = days_in_month(period);

    let period_begin_cwd = week_day_of_month(period, 1);

    let month_schedule = (1..=period_days_count).map(|x| seconds_from_week_schedule(&week_schedule, x, period_begin_cwd)).collect();

    return month_schedule;
}

#[allow(dead_code)]
pub(crate) fn timesheet_work_schedule(month_schedule: Vec<i32>, day_term_from: i16, day_term_stop: i16) -> Vec<i32> {
    let time_sheet = month_schedule.iter().enumerate().map(|(i, x)| hours_from_calendar(day_term_from, day_term_stop, i as i16, *x)).collect();

    return time_sheet;
}

#[allow(dead_code)]
pub(crate) fn timesheet_work_contract(month_contract: Vec<i32>, month_position: Vec<i32>, day_term_from: i16, day_term_stop: i16) -> Vec<i32> {
    let idx_from = (day_term_from - 1) as usize;
    let idx_stop = (day_term_stop - 1) as usize;
    let ziped_month: Vec<_> = month_contract.iter().zip(month_position.iter()).collect();
    let result = ziped_month.iter().enumerate().map(|(idx, z)| {
        let mut res: i32 = 0;
        if idx >= idx_from && idx <= idx_stop {
            res = z.0 + z.1;
        };
        return res;
   }).collect();
   return result;
}

#[allow(dead_code)]
pub(crate) fn seconds_from_period_week_schedule(period: &dyn IPeriod, week_schedule: &Vec<i32>, day_ordinal: i16) -> i32 {
    let period_begin_cwd = week_day_of_month(period, 1);

    return seconds_from_week_schedule(&week_schedule, day_ordinal, period_begin_cwd);
}

#[allow(dead_code)]
pub(crate) fn seconds_from_week_schedule(week_schedule: &Vec<i32>, day_ordinal: i16, period_begin_cwd: i16) -> i32 {
    let day_of_week = day_of_week_from_ordinal(day_ordinal, period_begin_cwd);

    let index_week = day_of_week - 1;

    if index_week < 0 {
        return 0;
    }
    let index_vector = index_week as usize;
    if index_vector >= week_schedule.len() {
        return 0;
    }
    return week_schedule[index_vector];
}

#[allow(dead_code)]
pub(crate) fn seconds_from_schedule_seq(time_table: Vec<i32>, day_ordinal: i16, day_from_ord: i16, day_ends_ord: i16) -> i32 {
    if day_ordinal < day_from_ord || day_ordinal > day_ends_ord {
        return 0;
    }

    let index_table = day_ordinal - day_from_ord;

    if index_table < 0 {
        return 0;
    }
    let index_vector = index_table as usize;
    if index_vector >= time_table.len() {
        return 0;
    }

    return time_table[index_vector];
}

#[allow(dead_code)]
pub(crate) fn schedule_base_subtract(template: Vec<i32>, subtract: Vec<i32>, day_from: i32, day_stop: i32) -> Vec<i32> {
    let idx_from = (day_from - 1) as usize;
    let idx_stop = (day_stop - 1) as usize;
    let ziped_work_absc: Vec<_> = template.iter().zip(subtract.iter()).collect();
    let result = ziped_work_absc.iter().enumerate().map(|(idx, z)| {
        let mut res: i32 = 0;
        if idx >= idx_from && idx <= idx_stop {
            res = max(0, z.0 - z.1);
        };
        return res;
    }).collect();
    return result;
}

#[allow(dead_code)]
pub(crate) fn plus_hours_from_calendar(day_term_from: i16, day_term_stop: i16, day_index: i16, part_seconds: i32, work_seconds: i32) -> i32 {
    let day_ordinal = day_index + 1;

    let plus_seconds = match day_ordinal {
        x if  x < day_term_from => 0,
        x if  x > day_term_stop => 0,
        _ => work_seconds,
    };
    return plus_seconds + part_seconds;
}

#[allow(dead_code)]
pub(crate) fn hours_from_calendar(day_term_from: i16, day_term_stop: i16, day_index: i16, work_seconds: i32) -> i32 {
    let day_ordinal = day_index + 1;

    let working_day = match day_ordinal {
        x if  x < day_term_from => 0,
        x if  x > day_term_stop => 0,
        _ => work_seconds,
    };
    return working_day;
}
