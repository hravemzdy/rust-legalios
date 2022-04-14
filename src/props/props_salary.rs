use std::cmp::{max, min};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::factories::provider_factory::BoxSalaryProps;
use crate::props::props::IProps;
use crate::service::operations_dec;
use crate::service::operations_round;
use crate::service::version_id::VersionId;

pub trait IPropsSalary : IProps {
    fn working_shift_week(&self) -> i32;
    fn working_shift_time(&self) -> i32;
    fn min_monthly_wage(&self) -> i32;
    fn min_hourly_wage(&self) -> i32;

    fn value_equals(&self, other_salary: &BoxSalaryProps) -> bool;
    fn coeff_with_part_and_full_hours(&self, full_work_hours: Decimal, part_work_hours: Decimal) ->  Decimal;
    fn payment_with_monthly_and_full_week_and_full_and_work_hours(&self, amount_monthly: Decimal,
                                                                  full_week_hours: i32, part_week_hours: i32,
                                                                  full_work_hours: i32, part_work_hours: i32) ->  Decimal;
    fn payment_round_up_with_monthly_and_full_week_and_full_and_work_hours(&self, amount_monthly: Decimal,
                                                                           full_week_hours: i32, part_week_hours: i32,
                                                                           full_work_hours: i32, part_work_hours: i32) ->  Decimal;
    fn payment_with_monthly_and_coeff_and_full_and_work_hours(&self, amount_monthly: Decimal,
                                                              monthly_coeff: Decimal, full_work_hours: i32, part_work_hours: i32) ->  Decimal;
    fn payment_round_up_with_monthly_and_coeff_and_full_and_work_hours(&self, amount_monthly: Decimal,
                                                                       monthly_coeff: Decimal, full_work_hours: i32, part_work_hours: i32) ->  Decimal;
    fn payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal,
                                                     monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn payment_round_up_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal,
                                                              monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn relative_amount_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn relative_tariff_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn relative_payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn reverzed_amount_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn reverzed_tariff_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn reverzed_payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) ->  Decimal;
    fn payment_with_tariff_and_hours(&self, tariff_hourly: Decimal, workings_hours: Decimal) ->  Decimal;
    fn payment_round_up_with_tariff_and_hours(&self, tariff_hourly: Decimal, workings_hours: Decimal) ->  Decimal;
    fn tariff_with_payment_and_hours(&self, amount_hourly: Decimal, workings_hours: Decimal) ->  Decimal;
    fn payment_with_amount_fixed(&self, amount_fixed: Decimal) ->  Decimal;
    fn payment_round_up_with_amount_fixed(&self, amount_fixed: Decimal) ->  Decimal;
    fn hours_to_half_hours_up(&self, hours_value: Decimal) ->  Decimal;
    fn hours_to_quart_hours_up(&self, hours_value: Decimal) ->  Decimal;
    fn hours_to_half_hours_down(&self, hours_value: Decimal) ->  Decimal;
    fn hours_to_quart_hours_down(&self, hours_value: Decimal) ->  Decimal;
    fn hours_to_half_hours_norm(&self, hours_value: Decimal) ->  Decimal;
    fn hours_to_quart_hours_norm(&self, hours_value: Decimal) ->  Decimal;
    fn money_to_round_down(&self, money_value: Decimal) ->  Decimal;
    fn money_to_round_up(&self, money_value: Decimal) ->  Decimal;
    fn money_to_round_norm(&self, money_value: Decimal) ->  Decimal;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsSalary{
    version: VersionId,
    working_shift_week: i32,
    working_shift_time: i32,
    min_monthly_wage: i32,
    min_hourly_wage: i32,
}

#[allow(dead_code)]
impl PropsSalary {
    pub(crate) fn new(_version: VersionId,
                      _working_shift_week: i32,
                      _working_shift_time: i32,
                      _min_monthly_wage: i32,
                      _min_hourly_wage: i32) -> PropsSalary {
        PropsSalary {
            version: _version,
            working_shift_week: _working_shift_week,
            working_shift_time: _working_shift_time,
            min_monthly_wage: _min_monthly_wage,
            min_hourly_wage: _min_hourly_wage,
        }
    }
    pub(crate) fn empty() -> PropsSalary {
        PropsSalary {
            version: VersionId::new(),
            working_shift_week: 0,
            working_shift_time: 0,
            min_monthly_wage: 0,
            min_hourly_wage: 0,
        }
    }
    fn total_hours_with_full_and_part_hours(full_work_hours: i32, part_work_hours: i32) -> i32 {
        let totals_hours = max(0, part_work_hours);

        let result_hours = min(full_work_hours, totals_hours);

        return result_hours;
    }
    fn dec_payment_with_monthly_and_coeff_and_full_and_work_hours(amount_monthly: Decimal, monthly_coeff: Decimal, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let coeff_amount = Self::factorize_value(amount_monthly, monthly_coeff);
    
        let payment = Self::dec_payment_with_monthly_and_full_and_work_hours(coeff_amount, full_work_hours, part_work_hours);
    
        return payment;
    }
    fn dec_payment_with_monthly_and_full_and_work_hours(amount_monthly: Decimal, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let paym_work_hours = Self::total_hours_with_full_and_part_hours(full_work_hours, part_work_hours);

        let payment = operations_dec::multiply_and_divide(amount_monthly, Decimal::from_i32(paym_work_hours).unwrap(), Decimal::from_i32(full_work_hours).unwrap());
        
        return payment;
    }
    fn dec_payment_with_tariff_and_hours(tariff_hourly: Decimal, workings_hours: Decimal) -> Decimal {
        let total_hours: Decimal  = Decimal::ZERO.max(workings_hours);

        let payment: Decimal = operations_dec::multiply(total_hours, tariff_hourly);
        
        return payment;
    }
    fn dec_tariff_with_payment_and_hours(amount_hourly: Decimal, workings_hours: Decimal) -> Decimal {
        let total_hours: Decimal  = Decimal::ZERO.max(workings_hours);

        let tariff: Decimal = operations_dec::divide(amount_hourly, total_hours);
        
        return tariff;
    }
    fn dec_payment_with_amount_fixed(amount_fixed: Decimal) -> Decimal {
        let payment = amount_fixed;
        
        return payment;
    }
    fn factorize_value(base_value: Decimal, factor: Decimal) -> Decimal {
        let result = operations_dec::multiply(base_value, factor);

        return result;
    }
    fn reverzed_factorize_value(base_value: Decimal, factor: Decimal) -> Decimal {
        let result = operations_dec::multiply(base_value, operations_dec::divide(Decimal::from_i32(1).unwrap(), factor));

        return result;
    }
}

impl IProps for PropsSalary {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSalary for PropsSalary {
    fn working_shift_week(&self) -> i32 {
        self.working_shift_week
    }

    fn working_shift_time(&self) -> i32 {
        self.working_shift_time
    }

    fn min_monthly_wage(&self) -> i32 {
        self.min_monthly_wage
    }

    fn min_hourly_wage(&self) -> i32 {
        self.min_hourly_wage
    }

    fn value_equals(&self, other_salary: &BoxSalaryProps) -> bool {
        return self.working_shift_week == other_salary.working_shift_week() &&
            self.working_shift_time == other_salary.working_shift_time() &&
            self.min_monthly_wage == other_salary.min_monthly_wage() &&
            self.min_hourly_wage == other_salary.min_hourly_wage();
    }

    fn coeff_with_part_and_full_hours(&self, full_work_hours: Decimal, part_work_hours: Decimal) -> Decimal {
        let coeff_working = Decimal::from_i32(1).unwrap().min(operations_dec::divide(part_work_hours, full_work_hours));

        return coeff_working;
    }

    fn relative_amount_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let amount_coeffs = Self::factorize_value(amount_monthly, monthly_coeff);

        let relative_amount = Self::factorize_value(amount_coeffs, working_coeff);

        return relative_amount;
    }

    fn reverzed_amount_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let amount_coeffs = Self::reverzed_factorize_value(amount_monthly, monthly_coeff);

        let reverzed_amount = Self::reverzed_factorize_value(amount_coeffs, working_coeff);

        return reverzed_amount;
    }

    fn relative_tariff_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let payment_value = self.relative_amount_with_monthly_and_coeff_and_work_coeff(amount_monthly, monthly_coeff, working_coeff);

        return operations_round::dec_round_norm01(payment_value);
    }

    fn reverzed_tariff_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let payment_value = self.reverzed_amount_with_monthly_and_coeff_and_work_coeff(amount_monthly, monthly_coeff, working_coeff);

        return operations_round::dec_round_norm01(payment_value);
    }

    fn relative_payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let payment_value = self.relative_amount_with_monthly_and_coeff_and_work_coeff(amount_monthly, monthly_coeff, working_coeff);

        return operations_round::dec_round_norm(payment_value);
    }

    fn reverzed_payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let payment_value = self.reverzed_amount_with_monthly_and_coeff_and_work_coeff(amount_monthly, monthly_coeff, working_coeff);

        return operations_round::dec_round_norm(payment_value);
    }

    fn payment_with_amount_fixed(&self, amount_fixed: Decimal) -> Decimal {
        let payment_value = Self::dec_payment_with_amount_fixed(amount_fixed);

        return operations_round::dec_round_norm(payment_value);
    }

    fn payment_round_up_with_amount_fixed(&self, amount_fixed: Decimal) -> Decimal {
        let payment_value = Self::dec_payment_with_amount_fixed(amount_fixed);

        return operations_round::dec_round_up(payment_value);
    }
    fn payment_with_tariff_and_hours(&self, tariff_hourly: Decimal, workings_hours: Decimal) -> Decimal {
        let payment_value = Self::dec_payment_with_tariff_and_hours(tariff_hourly, workings_hours);
    
        return operations_round::dec_round_norm(payment_value);
    }
    
    fn payment_round_up_with_tariff_and_hours(&self, tariff_hourly: Decimal, workings_hours: Decimal) -> Decimal {
        let payment_value = Self::dec_payment_with_tariff_and_hours(tariff_hourly, workings_hours);

        return operations_round::dec_round_up(payment_value);
    }

    fn tariff_with_payment_and_hours(&self, amount_hourly: Decimal, workings_hours: Decimal) -> Decimal {
        let tariff_value = Self::dec_tariff_with_payment_and_hours(amount_hourly, workings_hours);

        return self.money_to_round_norm(tariff_value);
    }

    fn payment_with_monthly_and_coeff_and_full_and_work_hours(&self, amount_monthly: Decimal, monthly_coeff: Decimal, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let amount_coeffs = Self::factorize_value(amount_monthly, monthly_coeff);

        let payment_value = Self::dec_payment_with_monthly_and_full_and_work_hours(amount_coeffs, full_work_hours, part_work_hours);

        return operations_round::dec_round_norm(payment_value);
    }

    fn payment_round_up_with_monthly_and_coeff_and_full_and_work_hours(&self, amount_monthly: Decimal, monthly_coeff: Decimal, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let amount_coeffs = Self::factorize_value(amount_monthly, monthly_coeff);

        let payment_value = Self::dec_payment_with_monthly_and_full_and_work_hours(amount_coeffs, full_work_hours, part_work_hours);

        return operations_round::dec_round_up(payment_value);
    }

    fn payment_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let amount_factor = Self::factorize_value(amount_monthly, monthly_coeff);

        let payment_value = Self::factorize_value(amount_factor, working_coeff);

        return operations_round::dec_round_norm(payment_value);
    }

    fn payment_round_up_with_monthly_and_coeff_and_work_coeff(&self, amount_monthly: Decimal, monthly_coeff: Decimal, working_coeff: Decimal) -> Decimal {
        let amount_factor = Self::factorize_value(amount_monthly, monthly_coeff);

        let payment_value = Self::factorize_value(amount_factor, working_coeff);

        return operations_round::dec_round_up(payment_value);
    }

    fn payment_with_monthly_and_full_week_and_full_and_work_hours(&self, amount_monthly: Decimal, full_week_hours: i32, part_week_hours: i32, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let coeff_salary = self.coeff_with_part_and_full_hours(Decimal::from_i32(part_week_hours).unwrap(), Decimal::from_i32(full_week_hours).unwrap());

        let salary_value = self.payment_with_monthly_and_coeff_and_full_and_work_hours(amount_monthly, coeff_salary, full_work_hours, part_work_hours);

        return salary_value;
    }

    fn payment_round_up_with_monthly_and_full_week_and_full_and_work_hours(&self, amount_monthly: Decimal, full_week_hours: i32, part_week_hours: i32, full_work_hours: i32, part_work_hours: i32) -> Decimal {
        let coeff_salary = self.coeff_with_part_and_full_hours(Decimal::from_i32(part_week_hours).unwrap(), Decimal::from_i32(full_week_hours).unwrap());

        let salary_value = self.payment_round_up_with_monthly_and_coeff_and_full_and_work_hours(amount_monthly, coeff_salary, full_work_hours, part_work_hours);

        return salary_value;
    }

    fn hours_to_half_hours_up(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_up50(hours_value);
    }

    fn hours_to_quart_hours_up(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_up25(hours_value);
    }

    fn hours_to_half_hours_down(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_down50(hours_value);
    }

    fn hours_to_quart_hours_down(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_down25(hours_value);
    }

    fn hours_to_half_hours_norm(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_norm50(hours_value);
    }

    fn hours_to_quart_hours_norm(&self, hours_value: Decimal) -> Decimal {
        return operations_round::dec_round_norm25(hours_value);
    }

    fn money_to_round_down(&self, money_value: Decimal) -> Decimal {
        return operations_round::dec_round_down01(money_value);
    }

    fn money_to_round_up(&self, money_value: Decimal) -> Decimal {
        return operations_round::dec_round_up01(money_value);
    }

    fn money_to_round_norm(&self, money_value: Decimal) -> Decimal {
        return operations_round::dec_round_norm01(money_value);
    }
}
