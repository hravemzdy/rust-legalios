use rust_decimal::Decimal;

pub(crate) trait HistoryConstHealth {
    const VERSION_CODE: i16;

    const MIN_MONTHLY_BASIS: i32;
    const MAX_ANNUALS_BASIS: i32;
    const LIM_MONTHLY_STATE: i32;
    const LIM_MONTHLY_DIS50: i32;
    const FACTOR_COMPOUND: Decimal;
    const FACTOR_EMPLOYEE: Decimal;
    const MARGIN_INCOME_EMP: i32;
    const MARGIN_INCOME_AGR: i32;

}