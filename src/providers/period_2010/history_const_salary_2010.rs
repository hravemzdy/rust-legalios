use crate::providers::history_const_salary::HistoryConstSalary;
// WORKING_SHIFT_WEEK      Počet pracovních dnů v týdnu
//
// WORKING_SHIFT_TIME      Počet pracovních hodin denně
//
// MIN_MONTHLY_WAGE        Minimální mzda měsíční
//
// MIN_HOURLY_WAGE         Minimální mzda hodinová (100*Kč)
pub(crate) struct HistoryConstSalary2010 {
}

impl HistoryConstSalary for HistoryConstSalary2010 {
    const VERSION_CODE: i16 = 2010;

    const WORKING_SHIFT_WEEK: i32 = 5;
    const WORKING_SHIFT_TIME: i32 = 8;
    const MIN_MONTHLY_WAGE: i32 = 8000;
    const MIN_HOURLY_WAGE: i32 = 4810;
}