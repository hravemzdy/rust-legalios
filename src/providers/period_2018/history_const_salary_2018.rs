use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2017::history_const_salary_2017::HistoryConstSalary2017;
//
// Created by Ladislav Lisy on 13.06.2021.
//

// WORKING_SHIFT_WEEK      Počet pracovních dnů v týdnu
//
// WORKING_SHIFT_TIME      Počet pracovních hodin denně
//
// MIN_MONTHLY_WAGE        Minimální mzda měsíční
//
// MIN_HOURLY_WAGE         Minimální mzda hodinová (100*Kč)

pub(crate) struct HistoryConstSalary2018 {
}

impl HistoryConstSalary for HistoryConstSalary2018 {
    const VERSION_CODE: i16 = 2018;

    const WORKING_SHIFT_WEEK: i32 = HistoryConstSalary2017::WORKING_SHIFT_WEEK;
    const WORKING_SHIFT_TIME: i32 = HistoryConstSalary2017::WORKING_SHIFT_TIME;
    const MIN_MONTHLY_WAGE: i32   = 12200;
    const MIN_HOURLY_WAGE: i32    = 7320;
}

