use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use crate::providers::history_const_social::HistoryConstSocial;
use crate::providers::period_2018::history_const_social_2018::HistoryConstSocial2018;
//
// Created by Ladislav Lisy on 13.06.2021.
//

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

pub(crate) struct HistoryConstSocial2019var07 {
}

impl HistoryConstSocial for HistoryConstSocial2019var07 {
    const VERSION_CODE: i16 = HistoryConstSocial2019::VERSION_CODE;

    const MAX_ANNUALS_BASIS: i32 = HistoryConstSocial2019::MAX_ANNUALS_BASIS;
    const FACTOR_EMPLOYER: Decimal = dec!(24.8);
    const FACTOR_EMPLOYER_HIGHER: Decimal = HistoryConstSocial2019::FACTOR_EMPLOYER_HIGHER;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstSocial2019::FACTOR_EMPLOYEE;
    const FACTOR_EMPLOYEE_REDUCE: Decimal = HistoryConstSocial2019::FACTOR_EMPLOYEE_REDUCE;
    const FACTOR_EMPLOYEE_GARANT: Decimal = HistoryConstSocial2019::FACTOR_EMPLOYEE_GARANT;
    const MARGIN_INCOME_EMP: i32 = HistoryConstSocial2019::MARGIN_INCOME_EMP;
    const MARGIN_INCOME_AGR: i32 = HistoryConstSocial2018::MARGIN_INCOME_AGR;
}

pub(crate) struct HistoryConstSocial2019 {
}

impl HistoryConstSocial for HistoryConstSocial2019 {
    const VERSION_CODE: i16 = 2019;

    const MAX_ANNUALS_BASIS: i32 = 1569552;
    const FACTOR_EMPLOYER: Decimal = HistoryConstSocial2018::FACTOR_EMPLOYER;
    const FACTOR_EMPLOYER_HIGHER: Decimal = HistoryConstSocial2018::FACTOR_EMPLOYER_HIGHER;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstSocial2018::FACTOR_EMPLOYEE;
    const FACTOR_EMPLOYEE_REDUCE: Decimal = HistoryConstSocial2018::FACTOR_EMPLOYEE_REDUCE;
    const FACTOR_EMPLOYEE_GARANT: Decimal = HistoryConstSocial2018::FACTOR_EMPLOYEE_GARANT;
    const MARGIN_INCOME_EMP: i32 = 3000;
    const MARGIN_INCOME_AGR: i32 = HistoryConstSocial2018::MARGIN_INCOME_AGR;
}

