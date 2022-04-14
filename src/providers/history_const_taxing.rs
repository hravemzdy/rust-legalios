use rust_decimal::Decimal;

pub(crate) trait HistoryConstTaxing {
    const VERSION_CODE: i16;

    const ALLOWANCE_PAYER: i32;
    const ALLOWANCE_DISAB_1ST: i32;
    const ALLOWANCE_DISAB_2ND: i32;
    const ALLOWANCE_DISAB_3RD: i32;
    const ALLOWANCE_STUDY: i32;
    const ALLOWANCE_CHILD_1ST: i32;
    const ALLOWANCE_CHILD_2ND: i32;
    const ALLOWANCE_CHILD_3RD: i32;
    const SETTLEMENT_CHILD_2ND: i32;
    const SETTLEMENT_CHILD_3RD: i32;
    const FACTOR_ADVANCES: Decimal;
    const FACTOR_WITHHOLD: Decimal;
    const FACTOR_SOLITARY: Decimal;
    const FACTOR_TAXRATE2: Decimal;
    const MIN_AMOUNT_OF_TAXBONUS: i32;
    const MAX_AMOUNT_OF_TAXBONUS: i32;
    const MARGIN_INCOME_OF_TAXBONUS: i32;
    const MARGIN_INCOME_OF_ROUNDING: i32;
    const MARGIN_INCOME_OF_WITHHOLD: i32;
    const MARGIN_INCOME_OF_SOLITARY: i32;
    const MARGIN_INCOME_OF_TAXRATE2: i32;
    const MARGIN_INCOME_OF_WHT_EMP: i32;
    const MARGIN_INCOME_OF_WHT_AGR: i32;
}