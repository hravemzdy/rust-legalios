use rust_decimal::Decimal;
use crate::providers::history_const_social::HistoryConstSocial;
use crate::providers::period_2020::history_const_social_2020::HistoryConstSocial2020;
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

pub(crate) struct HistoryConstSocial2021 {
}

impl HistoryConstSocial for HistoryConstSocial2021 {
    const VERSION_CODE: i16 = 2021;

    const MAX_ANNUALS_BASIS: i32 = 1701168;
    const FACTOR_EMPLOYER: Decimal = HistoryConstSocial2020::FACTOR_EMPLOYER;
    const FACTOR_EMPLOYER_HIGHER: Decimal = HistoryConstSocial2020::FACTOR_EMPLOYER_HIGHER;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstSocial2020::FACTOR_EMPLOYEE;
    const FACTOR_EMPLOYEE_REDUCE: Decimal = HistoryConstSocial2020::FACTOR_EMPLOYEE_REDUCE;
    const FACTOR_EMPLOYEE_GARANT: Decimal = HistoryConstSocial2020::FACTOR_EMPLOYEE_GARANT;
    const MARGIN_INCOME_EMP: i32 = 3500;
    const MARGIN_INCOME_AGR: i32 = HistoryConstSocial2020::MARGIN_INCOME_AGR;
}

