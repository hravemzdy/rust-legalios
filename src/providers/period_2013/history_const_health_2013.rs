use rust_decimal::Decimal;
use crate::providers::history_const_health::HistoryConstHealth;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2012::history_const_health_2012::HistoryConstHealth2012;
use crate::providers::period_2013::history_const_salary_2013::{HistoryConstSalary2013, HistoryConstSalary2013var08};
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

pub(crate) struct HistoryConstHealth2013var08 {
}

impl HistoryConstHealth for HistoryConstHealth2013var08 {
    const VERSION_CODE: i16 = HistoryConstHealth2013::VERSION_CODE;

    const MIN_MONTHLY_BASIS: i32 = HistoryConstSalary2013var08::MIN_MONTHLY_WAGE;
    const MAX_ANNUALS_BASIS: i32 = HistoryConstHealth2013::MAX_ANNUALS_BASIS;
    const LIM_MONTHLY_STATE: i32 = HistoryConstHealth2013::LIM_MONTHLY_STATE;
    const LIM_MONTHLY_DIS50: i32 = HistoryConstHealth2013::LIM_MONTHLY_DIS50;
    const FACTOR_COMPOUND: Decimal = HistoryConstHealth2013::FACTOR_COMPOUND;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstHealth2013::FACTOR_EMPLOYEE;
    const MARGIN_INCOME_EMP: i32 = HistoryConstHealth2013::MARGIN_INCOME_EMP;
    const MARGIN_INCOME_AGR: i32 = HistoryConstHealth2013::MARGIN_INCOME_AGR;
}

pub(crate) struct HistoryConstHealth2013var11 {
}

impl HistoryConstHealth for HistoryConstHealth2013var11 {
    const VERSION_CODE: i16 = HistoryConstHealth2013::VERSION_CODE;

    const MIN_MONTHLY_BASIS: i32 = HistoryConstSalary2013var08::MIN_MONTHLY_WAGE;
    const MAX_ANNUALS_BASIS: i32 = HistoryConstHealth2013::MAX_ANNUALS_BASIS;
    const LIM_MONTHLY_STATE: i32 = HistoryConstHealth2013::LIM_MONTHLY_STATE;
    const LIM_MONTHLY_DIS50: i32 = 5829;
    const FACTOR_COMPOUND: Decimal = HistoryConstHealth2013::FACTOR_COMPOUND;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstHealth2013::FACTOR_EMPLOYEE;
    const MARGIN_INCOME_EMP: i32 = HistoryConstHealth2013::MARGIN_INCOME_EMP;
    const MARGIN_INCOME_AGR: i32 = HistoryConstHealth2013::MARGIN_INCOME_AGR;
}

pub(crate) struct HistoryConstHealth2013 {
}

impl HistoryConstHealth for HistoryConstHealth2013 {
    const VERSION_CODE: i16 = 2013;

    const MIN_MONTHLY_BASIS: i32 = HistoryConstSalary2013::MIN_MONTHLY_WAGE;
    const MAX_ANNUALS_BASIS: i32 = 0;
    const LIM_MONTHLY_STATE: i32 = HistoryConstHealth2012::LIM_MONTHLY_STATE;
    const LIM_MONTHLY_DIS50: i32 = HistoryConstHealth2012::LIM_MONTHLY_DIS50;
    const FACTOR_COMPOUND: Decimal = HistoryConstHealth2012::FACTOR_COMPOUND;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstHealth2012::FACTOR_EMPLOYEE;
    const MARGIN_INCOME_EMP: i32 = HistoryConstHealth2012::MARGIN_INCOME_EMP;
    const MARGIN_INCOME_AGR: i32 = HistoryConstHealth2012::MARGIN_INCOME_AGR;
}

