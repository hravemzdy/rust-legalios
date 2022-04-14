use std::cmp::max;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::factories::provider_factory::BoxHealthProps;
use crate::props::particy_results::{ParticyHealthResult, ParticyHealthTarget};
use crate::props::props::IProps;
use crate::service::contract_terms::WorkHealthTerms;
use crate::service::version_id::VersionId;
use crate::service::{operations_dec, operations_round};

pub trait IPropsHealth : IProps {
    fn min_monthly_basis(&self) -> i32;
    fn max_annuals_basis(&self) -> i32;
    fn lim_monthly_state(&self) -> i32;
    fn lim_monthly_dis50(&self) -> i32;
    fn factor_compound(&self) -> Decimal;
    fn factor_employee(&self) -> Decimal;
    fn margin_income_emp(&self) -> i32;
    fn margin_income_agr(&self) -> i32;

    fn value_equals(&self, other_health: &BoxHealthProps) ->  bool;
    fn has_particy(&self, term: &WorkHealthTerms, income_term: i32, income_spec: i32) ->  bool;
    fn rounded_compound_paym(&self, basis_result: i32) ->  i32;
    fn rounded_employee_paym(&self, basis_result: i32) ->  i32;
    fn rounded_augment_employee_paym(&self, basis_generals: i32, basis_augment: i32) ->  i32;
    fn rounded_augment_employer_paym(&self, basis_generals: i32, base_employee: i32, base_employer: i32) ->  i32;
    fn rounded_employer_paym(&self, basis_result: i32) ->  i32;
    fn annuals_basis_cut(&self, income_list: &Vec<ParticyHealthTarget>, annuity_basis: i32) -> (i32, i32, Vec<ParticyHealthResult>);
}

#[derive(Debug, Copy, Clone)]
pub struct PropsHealthBase {
    version: VersionId,
    min_monthly_basis: i32,
    max_annuals_basis: i32,
    lim_monthly_state: i32,
    lim_monthly_dis50: i32,
    factor_compound: Decimal,
    factor_employee: Decimal,
    margin_income_emp: i32,
    margin_income_agr: i32,
}

#[allow(dead_code)]
impl PropsHealthBase {
    pub(crate) fn new(_version: VersionId,
                      _min_monthly_basis: i32,
                      _max_annuals_basis: i32,
                      _lim_monthly_state: i32,
                      _lim_monthly_dis50: i32,
                      _factor_compound: Decimal,
                      _factor_employee: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsHealthBase {
        PropsHealthBase {
            version: _version,
            min_monthly_basis: _min_monthly_basis,
            max_annuals_basis: _max_annuals_basis,
            lim_monthly_state: _lim_monthly_state,
            lim_monthly_dis50: _lim_monthly_dis50,
            factor_compound: _factor_compound,
            factor_employee: _factor_employee,
            margin_income_emp: _margin_income_emp,
            margin_income_agr: _margin_income_agr,
        }
    }
    pub(crate) fn empty() -> PropsHealthBase {
        PropsHealthBase {
            version: VersionId::new(),
            min_monthly_basis: 0,
            max_annuals_basis: 0,
            lim_monthly_state: 0,
            lim_monthly_dis50: 0,
            factor_compound: Decimal::ZERO,
            factor_employee: Decimal::ZERO,
            margin_income_emp: 0,
            margin_income_agr: 0,
        }
    }
    fn dec_insurance_round_up(value_dec: Decimal) -> Decimal {
        return operations_round::dec_round_up(value_dec);
    }

    fn int_insurance_round_up(value_dec: Decimal) -> i32 {
        return operations_round::round_up(value_dec);
    }
    fn has_term_exemption_particy(_term: &WorkHealthTerms) -> bool { false }
    fn has_income_based_employment_particy(_term: &WorkHealthTerms) -> bool { false }
    fn has_income_based_agreements_particy(_term: &WorkHealthTerms) -> bool { false }
    fn has_income_cumulated_particy(_term: &WorkHealthTerms) -> bool { false }

    pub(crate) fn has_particy_with_adapters(&self, term: &WorkHealthTerms,
                                            income_term: i32, income_spec: i32,
                                            exemption_particy: fn (&WorkHealthTerms) -> bool,
                                            employment_particy: fn (&WorkHealthTerms) -> bool,
                                            agreements_particy: fn (&WorkHealthTerms) -> bool,
                                            cumulated_particy: fn (&WorkHealthTerms) -> bool) -> bool {
        let mut particy_spec: bool = true;
        if exemption_particy(&term) {
            particy_spec = false;
        } else if agreements_particy(&term) && self.margin_income_agr > 0 {
            particy_spec = false;
            if cumulated_particy(&term) {
                if income_term >= self.margin_income_agr {
                    particy_spec = true;
                }
            } else {
                if income_spec >= self.margin_income_agr {
                    particy_spec = true;
                }
            }
        } else if employment_particy(&term) && self.margin_income_emp > 0 {
            particy_spec = false;
            if cumulated_particy(&term) {
                if income_term >= self.margin_income_emp {
                    particy_spec = true;
                }
            } else {
                if income_spec >= self.margin_income_emp {
                    particy_spec = true;
                }
            }
        }
        return particy_spec;
    }
}

impl IProps for PropsHealthBase {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsHealth for PropsHealthBase {
    fn min_monthly_basis(&self) -> i32 {
        self.min_monthly_basis
    }

    fn max_annuals_basis(&self) -> i32 {
        self.max_annuals_basis
    }

    fn lim_monthly_state(&self) -> i32 {
        self.lim_monthly_state
    }

    fn lim_monthly_dis50(&self) -> i32 {
        self.lim_monthly_dis50
    }

    fn factor_compound(&self) -> Decimal {
        self.factor_compound
    }

    fn factor_employee(&self) -> Decimal {
        self.factor_employee
    }

    fn margin_income_emp(&self) -> i32 {
        self.margin_income_emp
    }

    fn margin_income_agr(&self) -> i32 {
        self.margin_income_agr
    }

    fn value_equals(&self, other_health: &BoxHealthProps) -> bool {
         return self.min_monthly_basis == other_health.min_monthly_basis() &&
            self.max_annuals_basis == other_health.max_annuals_basis() &&
            self.lim_monthly_state == other_health.lim_monthly_state() &&
            self.lim_monthly_dis50 == other_health.lim_monthly_dis50() &&
            self.factor_compound == other_health.factor_compound() &&
            self.factor_employee == other_health.factor_employee() &&
            self.margin_income_emp == other_health.margin_income_emp() &&
            self.margin_income_agr == other_health.margin_income_agr();
    }

    fn has_particy(&self, term: &WorkHealthTerms, income_term: i32, income_spec: i32) -> bool {
        return self.has_particy_with_adapters(term, income_term, income_spec,
                                              Self::has_term_exemption_particy,
                                              Self::has_income_based_employment_particy,
                                              Self::has_income_based_agreements_particy,
                                              Self::has_income_cumulated_particy);
    }

    fn rounded_compound_paym(&self, basis_result: i32) -> i32 {
        let factor_compound = operations_dec::divide(self.factor_compound, Decimal::from_i32(100).unwrap());

        return Self::int_insurance_round_up(operations_dec::multiply(Decimal::from_i32(basis_result).unwrap(), factor_compound));
    }

    fn rounded_employee_paym(&self, basis_result: i32) -> i32 {
        let factor_compound = operations_dec::divide(self.factor_compound, Decimal::from_i32(100).unwrap());
        return Self::int_insurance_round_up(operations_dec::multiply_and_divide(Decimal::from_i32(basis_result).unwrap(), factor_compound, self.factor_employee));
    }

    fn rounded_augment_employee_paym(&self, basis_generals: i32, basis_augment: i32) -> i32 {
        let factor_compound = operations_dec::divide(self.factor_compound, Decimal::from_i32(100).unwrap());

        return Self::int_insurance_round_up(
            operations_dec::multiply(Decimal::from_i32(basis_augment).unwrap(), factor_compound)
                + operations_dec::multiply_and_divide(Decimal::from_i32(basis_generals).unwrap(), factor_compound, self.factor_employee));
    }

    fn rounded_augment_employer_paym(&self, basis_generals: i32, base_employee: i32, base_employer: i32) -> i32 {
        let factor_compound = operations_dec::divide(self.factor_compound, Decimal::from_i32(100).unwrap());

        let compound_basis = base_employer + base_employee + basis_generals;

        let compound_payment = Self::int_insurance_round_up(operations_dec::multiply(Decimal::from_i32(compound_basis).unwrap(), factor_compound));
        let employee_payment = Self::int_insurance_round_up(
            operations_dec::multiply(Decimal::from_i32(base_employee).unwrap(), factor_compound)
                + operations_dec::multiply_and_divide(Decimal::from_i32(basis_generals).unwrap(), factor_compound, self.factor_employee));

        return max(0, compound_payment - employee_payment);
    }

    fn rounded_employer_paym(&self, basis_result: i32) -> i32 {
        let compound_payment = self.rounded_compound_paym(basis_result);
        let employee_payment = self.rounded_employee_paym(basis_result);

        return max(0, compound_payment - employee_payment);
    }

     fn annuals_basis_cut(&self, income_list: &Vec<ParticyHealthTarget>, annuity_basis: i32) -> (i32, i32, Vec<ParticyHealthResult>) {
         let annualy_maxim: i32 = self.max_annuals_basis;
         let annuals_basis = max(0, annualy_maxim - annuity_basis);
         let result_init= (annualy_maxim, annuals_basis, vec![]);

         let result_list = income_list.into_iter().fold(result_init, |agr, x| {
             let raw_annuals_basis: i32 = x.get_targets_base();
             let mut cut_annuals_basis: i32 = 0;
             let mut rem_annuals_basis: i32 = agr.1;

             if x.get_particy_code() != 0 {
                 cut_annuals_basis = raw_annuals_basis;
                 if agr.0 > 0 {
                     let ovr_annuals_basis = max(0, raw_annuals_basis - agr.1);
                     cut_annuals_basis = raw_annuals_basis - ovr_annuals_basis;
                 }
                 rem_annuals_basis = max(0, agr.1 - cut_annuals_basis);
             }
             let r = ParticyHealthResult::new(
                 x.get_contract_code(),
                 x.get_subject_type(),
                 x.get_interest_code(),
                 x.get_subject_term(),
                 x.get_particy_code(),
                 x.get_targets_base(),
                 max(0, cut_annuals_basis));
             return (agr.0, rem_annuals_basis, [agr.2, vec![r]].concat());
         });
         return result_list;
     }
}
