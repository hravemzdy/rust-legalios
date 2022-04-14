use rust_decimal::Decimal;
use crate::factories::provider_factory::{BoxHealthProps};
use crate::props::particy_results::{ParticyHealthResult, ParticyHealthTarget};
use crate::props::props::IProps;
use crate::props::props_health_base::{IPropsHealth, PropsHealthBase};
use crate::service::contract_terms::WorkHealthTerms;
use crate::service::version_id::VersionId;

#[derive(Debug, Copy, Clone)]
pub struct PropsHealth2012 {
    props: PropsHealthBase,
}

#[allow(dead_code)]
impl PropsHealth2012 {
    pub(crate) fn new(_version: VersionId,
                      _min_monthly_basis: i32,
                      _max_annuals_basis: i32,
                      _lim_monthly_state: i32,
                      _lim_monthly_dis50: i32,
                      _factor_compound: Decimal,
                      _factor_employee: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsHealth2012 {
        PropsHealth2012 {
            props: PropsHealthBase::new(_version,
                    _min_monthly_basis,
                    _max_annuals_basis,
                    _lim_monthly_state,
                    _lim_monthly_dis50,
                    _factor_compound,
                    _factor_employee,
                    _margin_income_emp,
                    _margin_income_agr),
        }
    }
    pub(crate) fn empty() -> PropsHealth2012 {
        PropsHealth2012 {
            props: PropsHealthBase::empty(),
        }
    }
    fn has_term_exemption_particy(_term: &WorkHealthTerms) -> bool {
        return false;
    }

    fn has_income_based_employment_particy(term: &WorkHealthTerms) -> bool {
        return match term {
            WorkHealthTerms::HealthTermAgreemWork => true,
            _ => false,
        };
    }

    fn has_income_based_agreements_particy(term: &WorkHealthTerms) -> bool {
        return match term {
            WorkHealthTerms::HealthTermAgreemTask => true,
            _ => false,
        };
    }

    fn has_income_cumulated_particy(term: &WorkHealthTerms) -> bool {
        match term {
            WorkHealthTerms::HealthTermEmployments => false,
            WorkHealthTerms::HealthTermAgreemWork => false,
            WorkHealthTerms::HealthTermAgreemTask => false,
            WorkHealthTerms::HealthTermByContract => false,
        }
    }
}

impl IProps for PropsHealth2012 {
    fn get_version(&self) -> VersionId { self.props.get_version() }
}

impl IPropsHealth for PropsHealth2012 {
    fn min_monthly_basis(&self) -> i32 { self.props.min_monthly_basis() }

    fn max_annuals_basis(&self) -> i32 {
        self.props.max_annuals_basis()
    }

    fn lim_monthly_state(&self) -> i32 {
        self.props.lim_monthly_state()
    }

    fn lim_monthly_dis50(&self) -> i32 {
        self.props.lim_monthly_dis50()
    }

    fn factor_compound(&self) -> Decimal {
        self.props.factor_compound()
    }

    fn factor_employee(&self) -> Decimal {
        self.props.factor_employee()
    }

    fn margin_income_emp(&self) -> i32 {
        self.props.margin_income_emp()
    }

    fn margin_income_agr(&self) -> i32 { self.props.margin_income_agr() }

    fn value_equals(&self, other_health: &BoxHealthProps) -> bool {
         return self.props.value_equals(other_health);
    }

    fn has_particy(&self, term: &WorkHealthTerms, income_term: i32, income_spec: i32) -> bool {
        return self.props.has_particy_with_adapters(term, income_term, income_spec,
                                         Self::has_term_exemption_particy,
                                         Self::has_income_based_employment_particy,
                                         Self::has_income_based_agreements_particy,
                                         Self::has_income_cumulated_particy);
    }

    fn rounded_compound_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_compound_paym(basis_result);
    }

    fn rounded_employee_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_employee_paym(basis_result);
    }

    fn rounded_augment_employee_paym(&self, basis_generals: i32, basis_augment: i32) -> i32 {
        return self.props.rounded_augment_employee_paym(basis_generals, basis_augment);
    }

    fn rounded_augment_employer_paym(&self, basis_generals: i32, base_employee: i32, base_employer: i32) -> i32 {
        return self.props.rounded_augment_employer_paym(basis_generals, base_employee, base_employer);
    }

    fn rounded_employer_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_employer_paym(basis_result);
    }

    fn annuals_basis_cut(&self, income_list: &Vec<ParticyHealthTarget>, annuity_basis: i32) -> (i32, i32, Vec<ParticyHealthResult>) {
         return self.props.annuals_basis_cut(income_list, annuity_basis);
    }
}
