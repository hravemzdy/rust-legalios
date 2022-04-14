use std::cmp::max;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::factories::provider_factory::BoxSocialProps;
use crate::props::particy_results::{ParticySocialResult, ParticySocialTarget};
use crate::props::props::IProps;
use crate::service::contract_terms::WorkSocialTerms;
use crate::service::{operations_dec, operations_round};
use crate::service::version_id::VersionId;

pub trait IPropsSocial : IProps {
    fn max_annuals_basis(&self) -> i32;
    fn factor_employer(&self) -> Decimal;
    fn factor_employer_higher(&self) -> Decimal;
    fn factor_employee(&self) -> Decimal;
    fn factor_employee_garant(&self) -> Decimal;
    fn factor_employee_reduce(&self) -> Decimal;
    fn margin_income_emp(&self) -> i32;
    fn margin_income_agr(&self) -> i32;

    fn value_equals(&self, other_social: &BoxSocialProps) -> bool;
    fn has_particy(&self, term: &WorkSocialTerms, income_term: i32, income_spec: i32) -> bool;
    fn rounded_employee_paym(&self, basis_result: i32) -> i32;
    fn rounded_employer_paym(&self, basis_result: i32) -> i32;
    fn result_overcaps(&self, base_suma: i32, over_caps: i32) -> (i32, i32);
    fn annuals_basis_cut(&self, income_list: &Vec<ParticySocialTarget>, annuity_basis: i32) -> (i32, i32, Vec<ParticySocialResult>);
}

#[derive(Debug, Copy, Clone)]
pub struct PropsSocialBase {
    version: VersionId,
    max_annuals_basis: i32,
    factor_employer: Decimal,
    factor_employer_higher: Decimal,
    factor_employee: Decimal,
    factor_employee_garant: Decimal,
    factor_employee_reduce: Decimal,
    margin_income_emp: i32,
    margin_income_agr: i32,
}

#[allow(dead_code)]
impl PropsSocialBase {
    pub(crate) fn new(_version: VersionId,
                      _max_annuals_basis: i32,
                      _factor_employer: Decimal,
                      _factor_employer_higher: Decimal,
                      _factor_employee: Decimal,
                      _factor_employee_garant: Decimal,
                      _factor_employee_reduce: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsSocialBase {
        PropsSocialBase {
            version: _version,
            max_annuals_basis: _max_annuals_basis,
            factor_employer: _factor_employer,
            factor_employer_higher: _factor_employer_higher,
            factor_employee: _factor_employee,
            factor_employee_garant: _factor_employee_garant,
            factor_employee_reduce: _factor_employee_reduce,
            margin_income_emp: _margin_income_emp,
            margin_income_agr: _margin_income_agr,
        }
    }
    pub(crate) fn empty() -> PropsSocialBase {
        PropsSocialBase {
            version: VersionId::new(),
            max_annuals_basis: 0,
            factor_employer: Decimal::ZERO,
            factor_employer_higher: Decimal::ZERO,
            factor_employee: Decimal::ZERO,
            factor_employee_garant: Decimal::ZERO,
            factor_employee_reduce: Decimal::ZERO,
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

    fn has_term_exemption_particy(_term: &WorkSocialTerms) -> bool { return false; }
    fn has_income_based_employment_particy(_term: &WorkSocialTerms) -> bool { return false; }
    fn has_income_based_agreements_particy(_term: &WorkSocialTerms) -> bool { return false; }
    fn has_income_cumulated_particy(_term: &WorkSocialTerms) -> bool { return false; }

    pub(crate) fn has_particy_with_adapters(&self, term: &WorkSocialTerms,
                                            income_term: i32, income_spec: i32,
                                            exemption_particy: fn (&WorkSocialTerms) -> bool,
                                            employment_particy: fn (&WorkSocialTerms) -> bool,
                                            agreements_particy: fn (&WorkSocialTerms) -> bool,
                                            cumulated_particy: fn (&WorkSocialTerms) -> bool) -> bool {
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

impl IProps for PropsSocialBase {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSocial for PropsSocialBase {
    fn max_annuals_basis(&self) -> i32 {
        self.max_annuals_basis
    }

    fn factor_employer(&self) -> Decimal {
        self.factor_employer
    }

    fn factor_employer_higher(&self) -> Decimal {
        self.factor_employer_higher
    }

    fn factor_employee(&self) -> Decimal {
        self.factor_employee
    }

    fn factor_employee_garant(&self) -> Decimal {
        self.factor_employee_garant
    }

    fn factor_employee_reduce(&self) -> Decimal {
        self.factor_employee_reduce
    }

    fn margin_income_emp(&self) -> i32 {
        self.margin_income_emp
    }

    fn margin_income_agr(&self) -> i32 {
        self.margin_income_agr
    }

    fn value_equals(&self, other_social: &BoxSocialProps) -> bool {
        return self.max_annuals_basis == other_social.max_annuals_basis() &&
            self.factor_employer == other_social.factor_employer() &&
            self.factor_employer_higher == other_social.factor_employer_higher() &&
            self.factor_employee == other_social.factor_employee() &&
            self.factor_employee_garant == other_social.factor_employee_garant() &&
            self.factor_employee_reduce == other_social.factor_employee_reduce() &&
            self.margin_income_emp == other_social.margin_income_emp() &&
            self.margin_income_agr == other_social.margin_income_agr();
    }

    fn has_particy(&self, term: &WorkSocialTerms, income_term: i32, income_spec: i32) -> bool {
        return self.has_particy_with_adapters(term, income_term, income_spec,
                                         Self::has_term_exemption_particy,
                                         Self::has_income_based_employment_particy,
                                         Self::has_income_based_agreements_particy,
                                         Self::has_income_cumulated_particy);
    }

    fn rounded_employee_paym(&self, basis_result: i32) -> i32 {
        let factor_employee = operations_dec::divide(self.factor_employee, Decimal::from_i32(100).unwrap());
        return Self::int_insurance_round_up(operations_dec::multiply(Decimal::from_i32(basis_result).unwrap(), factor_employee))
    }

    fn rounded_employer_paym(&self, basis_result: i32) -> i32 {
        let factor_employer = operations_dec::divide(self.factor_employer, Decimal::from_i32(100).unwrap());
        return Self::int_insurance_round_up(operations_dec::multiply(Decimal::from_i32(basis_result).unwrap(), factor_employer));
    }

    fn result_overcaps(&self, base_suma: i32, over_caps: i32) -> (i32, i32) {
        let max_base_employee = max(0, base_suma - over_caps);
        let emp_base_overcaps = max(0, base_suma - max_base_employee);
        let val_base_overcaps = max(0, over_caps - emp_base_overcaps);
        return (max_base_employee, val_base_overcaps);
    }

    fn annuals_basis_cut(&self, income_list: &Vec<ParticySocialTarget>, annuity_basis: i32) -> (i32, i32, Vec<ParticySocialResult>) {
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
            let r = ParticySocialResult::new(
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

