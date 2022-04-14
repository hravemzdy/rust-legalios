pub(crate) trait HistoryConstSalary {
    const VERSION_CODE: i16;

    const WORKING_SHIFT_WEEK: i32;
    const WORKING_SHIFT_TIME: i32;
    const MIN_MONTHLY_WAGE: i32;
    const MIN_HOURLY_WAGE: i32;
}