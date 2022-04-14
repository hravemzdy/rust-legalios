use rust_decimal::Decimal;

pub(crate) trait HistoryConstSocial {
    const VERSION_CODE: i16;

    const MAX_ANNUALS_BASIS: i32;
    const FACTOR_EMPLOYER: Decimal;
    const FACTOR_EMPLOYER_HIGHER: Decimal;
    const FACTOR_EMPLOYEE: Decimal;
    const FACTOR_EMPLOYEE_REDUCE: Decimal;
    const FACTOR_EMPLOYEE_GARANT: Decimal;
    const MARGIN_INCOME_EMP: i32;
    const MARGIN_INCOME_AGR: i32;

}