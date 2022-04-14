use rust_decimal::Decimal;
use crate::providers::history_const_health::HistoryConstHealth;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2018::history_const_health_2018::HistoryConstHealth2018;
use crate::providers::period_2019::history_const_salary_2019::HistoryConstSalary2019;
//
// Created by Ladislav Lisy on 13.06.2021.
//

// MIN_MONTHLY_BASIS     Minimální základ zdravotního pojištění na jednoho pracovníka
//
// MAX_ANNUALS_BASIS     Maximální roční vyměřovací základ na jednoho pracovníka (tzv.strop)
//
// LIM_MONTHLY_STATE     Vyměřovací základ ze kterého platí pojistné stát za státní pojištěnce (mateřská, studenti, důchodci)
//
// LIM_MONTHLY_DIS50     Vyměřovací základ ze kterého platí pojistné stát za státní pojištěnce (mateřská, studenti, důchodci)
//                      u zaměstnavatele zaměstnávajícího více než 50% osob OZP
// FACTOR_COMPOUND       složená sazba zdravotního pojištění (zaměstnace + zaměstnavatele)
//
// FACTOR_EMPLOYEE       podíl sazby zdravotního pojištění připadajícího na zaměstnace (1/FACTOR_EMPLOYEE)
//
// MARGIN_INCOME_EMP     hranice příjmu pro vznik účasti na pojištění pro zaměstnace v pracovním poměru
//
// MARGIN_INCOME_AGR     hranice příjmu pro vznik účasti na pojištění pro zaměstnace na dohodu

pub(crate) struct HistoryConstHealth2019 {
}

impl HistoryConstHealth for HistoryConstHealth2019 {
    const VERSION_CODE: i16 = 2019;

    const MIN_MONTHLY_BASIS: i32 = HistoryConstSalary2019::MIN_MONTHLY_WAGE;
    const MAX_ANNUALS_BASIS: i32 = HistoryConstHealth2018::MAX_ANNUALS_BASIS;
    const LIM_MONTHLY_STATE: i32 = HistoryConstHealth2018::LIM_MONTHLY_STATE;
    const LIM_MONTHLY_DIS50: i32 = 7540;
    const FACTOR_COMPOUND: Decimal = HistoryConstHealth2018::FACTOR_COMPOUND;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstHealth2018::FACTOR_EMPLOYEE;
    const MARGIN_INCOME_EMP: i32 = 3000;
    const MARGIN_INCOME_AGR: i32 = HistoryConstHealth2018::MARGIN_INCOME_AGR;
}

