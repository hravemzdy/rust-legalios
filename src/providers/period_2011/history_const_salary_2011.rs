use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2010::history_const_salary_2010::HistoryConstSalary2010;

// WORKING_SHIFT_WEEK      Počet pracovních dnů v týdnu
//
// WORKING_SHIFT_TIME      Počet pracovních hodin denně
//
// MIN_MONTHLY_WAGE        Minimální mzda měsíční
//
// MIN_HOURLY_WAGE         Minimální mzda hodinová (100*Kč)
pub(crate) struct HistoryConstSalary2011 {
}

impl HistoryConstSalary for HistoryConstSalary2011 {
    const VERSION_CODE: i16 = 2011;

    const WORKING_SHIFT_WEEK: i32 = HistoryConstSalary2010::WORKING_SHIFT_WEEK;
    const WORKING_SHIFT_TIME: i32 = HistoryConstSalary2010::WORKING_SHIFT_TIME;
    const MIN_MONTHLY_WAGE: i32 = HistoryConstSalary2010::MIN_MONTHLY_WAGE;
    const MIN_HOURLY_WAGE: i32 = HistoryConstSalary2010::MIN_HOURLY_WAGE;
}