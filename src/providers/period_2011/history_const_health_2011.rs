use rust_decimal::Decimal;
use crate::providers::history_const_health::HistoryConstHealth;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2010::history_const_health_2010::HistoryConstHealth2010;
use crate::providers::period_2011::history_const_salary_2011::HistoryConstSalary2011;
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
pub(crate) struct HistoryConstHealth2011 {
}

impl HistoryConstHealth for HistoryConstHealth2011 {
    const VERSION_CODE: i16 = 2011;

    const MIN_MONTHLY_BASIS: i32 = HistoryConstSalary2011::MIN_MONTHLY_WAGE;
    const MAX_ANNUALS_BASIS: i32 = 1781280;
    const LIM_MONTHLY_STATE: i32 = HistoryConstHealth2010::LIM_MONTHLY_STATE;
    const LIM_MONTHLY_DIS50: i32 = HistoryConstHealth2010::LIM_MONTHLY_DIS50;
    const FACTOR_COMPOUND: Decimal = HistoryConstHealth2010::FACTOR_COMPOUND;
    const FACTOR_EMPLOYEE: Decimal = HistoryConstHealth2010::FACTOR_EMPLOYEE;
    const MARGIN_INCOME_EMP: i32 = HistoryConstHealth2010::MARGIN_INCOME_EMP;
    const MARGIN_INCOME_AGR: i32 = HistoryConstHealth2010::MARGIN_INCOME_AGR;
}
