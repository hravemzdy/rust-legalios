use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use crate::providers::history_const_social::HistoryConstSocial;
// MAX_ANNUALS_BASIS            Maximální roční vyměřovací základ na jednoho pracovníka (tzv.strop)
//
// FACTOR_EMPLOYER              Sazba - standardní sociálního pojištění - zaměstnavatele
//
// FACTOR_EMPLOYER_HIGHER       Sazba - vyší sociálního pojištění - zaměstnavatele
//
// FACTOR_EMPLOYEE              Sazba sociálního pojištění - zaměstnance
//
// FACTOR_EMPLOYEE_REDUCE       Snížení sazby sociálního pojištění - zaměstnance - s důchodovým spořením
//
// FACTOR_EMPLOYEE_GARANT       Sazba důchodového spoření - zaměstnance - s důchodovým spořením
//
// MARGIN_INCOME_EMP            hranice příjmu pro vznik účasti na pojištění pro zaměstnace v pracovním poměru
//
// MARGIN_INCOME_AGR            hranice příjmu pro vznik účasti na pojištění pro zaměstnace na dohodu
pub(crate) struct HistoryConstSocial2010 {
}

impl HistoryConstSocial for HistoryConstSocial2010 {
    const VERSION_CODE: i16 = 2010;

    const MAX_ANNUALS_BASIS: i32 = 1707048;
    const FACTOR_EMPLOYER: Decimal = dec!(25);
    const FACTOR_EMPLOYER_HIGHER: Decimal = Decimal::ZERO;
    const FACTOR_EMPLOYEE: Decimal = dec!(6.5);
    const FACTOR_EMPLOYEE_REDUCE: Decimal = Decimal::ZERO;
    const FACTOR_EMPLOYEE_GARANT: Decimal = Decimal::ZERO;
    const MARGIN_INCOME_EMP: i32 = 2000;
    const MARGIN_INCOME_AGR: i32 = 0;
}