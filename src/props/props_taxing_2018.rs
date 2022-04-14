use rust_decimal::Decimal;
use crate::factories::provider_factory::BoxTaxingProps;
use crate::props::props::IProps;
use crate::props::props_taxing_base::{IPropsTaxing, PropsTaxingBase};
use crate::service::contract_terms::WorkTaxingTerms;
use crate::service::operations_dec;
use crate::service::taxing_options::{TaxDeclBenfOption, TaxDeclDisabOption, TaxDeclSignOption, TaxNoneSignOption};
use crate::service::version_id::VersionId;

#[derive(Debug, Copy, Clone)]
pub struct PropsTaxing2018 {
    props: PropsTaxingBase,
}

#[allow(dead_code)]
impl PropsTaxing2018 {
    pub(crate) fn new(_version: VersionId,
                      _allowance_payer: i32,
                      _allowance_disab1st: i32,
                      _allowance_disab2nd: i32,
                      _allowance_disab3rd: i32,
                      _allowance_study: i32,
                      _allowance_child1st: i32,
                      _allowance_child2nd: i32,
                      _allowance_child3rd: i32,
                      _factor_advances: Decimal,
                      _factor_withhold: Decimal,
                      _factor_solidary: Decimal,
                      _factor_taxrate2: Decimal,
                      _min_amount_of_tax_bonus: i32,
                      _max_amount_of_tax_bonus: i32,
                      _margin_income_of_tax_bonus: i32,
                      _margin_income_of_rounding: i32,
                      _margin_income_of_withhold: i32,
                      _margin_income_of_solidary: i32,
                      _margin_income_of_taxrate2: i32,
                      _margin_income_of_wth_emp: i32,
                      _margin_income_of_wth_agr: i32) -> PropsTaxing2018 {
        PropsTaxing2018 {
            props: PropsTaxingBase::new(_version,
                    _allowance_payer,
                    _allowance_disab1st,
                    _allowance_disab2nd,
                    _allowance_disab3rd,
                    _allowance_study,
                    _allowance_child1st,
                    _allowance_child2nd,
                    _allowance_child3rd,
                    _factor_advances,
                    _factor_withhold,
                    _factor_solidary,
                    _factor_taxrate2,
                    _min_amount_of_tax_bonus,
                    _max_amount_of_tax_bonus,
                    _margin_income_of_tax_bonus,
                    _margin_income_of_rounding,
                    _margin_income_of_withhold,
                    _margin_income_of_solidary,
                    _margin_income_of_taxrate2,
                    _margin_income_of_wth_emp,
                    _margin_income_of_wth_agr),
        }
    }
    pub(crate) fn empty() -> PropsTaxing2018 {
        PropsTaxing2018 {
            props: PropsTaxingBase::empty(),
        }
    }
}

impl IProps for PropsTaxing2018 {
    fn get_version(&self) -> VersionId {
        self.props.get_version()
    }
}

impl IPropsTaxing for PropsTaxing2018 {
    fn allowance_payer(&self) -> i32 {
        self.props.allowance_payer()
    }

    fn allowance_disab1st(&self) -> i32 {
        self.props.allowance_disab1st()
    }

    fn allowance_disab2nd(&self) -> i32 {
        self.props.allowance_disab2nd()
    }

    fn allowance_disab3rd(&self) -> i32 {
        self.props.allowance_disab3rd()
    }

    fn allowance_study(&self) -> i32 {
        self.props.allowance_study()
    }

    fn allowance_child1st(&self) -> i32 {
        self.props.allowance_child1st()
    }

    fn allowance_child2nd(&self) -> i32 {
        self.props.allowance_child2nd()
    }

    fn allowance_child3rd(&self) -> i32 {
        self.props.allowance_child3rd()
    }

    fn factor_advances(&self) -> Decimal {
        self.props.factor_advances()
    }

    fn factor_withhold(&self) -> Decimal {
        self.props.factor_withhold()
    }

    fn factor_solidary(&self) -> Decimal {
        self.props.factor_solidary()
    }

    fn factor_taxrate2(&self) -> Decimal {
        self.props.factor_taxrate2()
    }

    fn min_amount_of_tax_bonus(&self) -> i32 {
        self.props.min_amount_of_tax_bonus()
    }

    fn max_amount_of_tax_bonus(&self) -> i32 {
        self.props.max_amount_of_tax_bonus()
    }

    fn margin_income_of_tax_bonus(&self) -> i32 {
        self.props.margin_income_of_tax_bonus()
    }

    fn margin_income_of_rounding(&self) -> i32 {
        self.props.margin_income_of_rounding()
    }

    fn margin_income_of_withhold(&self) -> i32 {
        self.props.margin_income_of_withhold()
    }

    fn margin_income_of_solidary(&self) -> i32 {
        self.props.margin_income_of_solidary()
    }

    fn margin_income_of_taxrate2(&self) -> i32 {
        self.props.margin_income_of_taxrate2()
    }

    fn margin_income_of_wth_emp(&self) -> i32 {
        self.props.margin_income_of_wth_emp()
    }

    fn margin_income_of_wth_agr(&self) -> i32 {
        self.props.margin_income_of_wth_agr()
    }

    fn value_equals(&self, other_taxing: &BoxTaxingProps) -> bool {
        return self.props.value_equals(other_taxing);
    }

    fn has_withhold_income(&self, term_opt: WorkTaxingTerms, sgn_opt: TaxDeclSignOption, none_opt: TaxNoneSignOption, income_sum: i32) -> bool {
        //*****************************************************************************
        // Tax income for advance from Year 2014 to Year 2017
        //*****************************************************************************
        // - withhold tax (non-signed declaration) and income
        // if (period.Year >= 2018)
        // -- income from DPP is less than X CZK
        // -- income from low-income employment is less than X CZK
        // -- income from statutory employment and non-resident is always withhold tax

        let mut withhold_income: bool = false;
        if sgn_opt != TaxDeclSignOption::DeclTaxNoSigned {
            return false;
        }
        if none_opt != TaxNoneSignOption::NosignTaxWithhold {
            return false;
        }
        if term_opt == WorkTaxingTerms::TaxingTermAgreemTask {
            if self.margin_income_of_wth_agr() == 0 || income_sum <= self.margin_income_of_wth_agr() {
                if income_sum > 0 {
                    withhold_income = true;
                }
            }
        }
        else if term_opt == WorkTaxingTerms::TaxingTermEmployments {
            if self.margin_income_of_wth_emp() == 0 || income_sum <= self.margin_income_of_wth_emp() {
                if income_sum > 0 {
                    withhold_income = true;
                }
            }
        }
        else if term_opt == WorkTaxingTerms::TaxingTermStatutPart {
            if income_sum > 0 {
                withhold_income = true;
            }
        }
        return withhold_income;
    }

    fn benefit_allowance_payer(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32 {
        return self.props.benefit_allowance_payer(sign_opts, benefit_opts);
    }

    fn benefit_allowance_disab(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclDisabOption) -> i32 {
        return self.props.benefit_allowance_disab(sign_opts, benefit_opts);
    }

    fn benefit_allowance_study(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32 {
        return self.props.benefit_allowance_study(sign_opts, benefit_opts);
    }

    fn benefit_allowance_child(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption, benefit_ords: i32, disabel_opts: i32) -> i32 {
        return self.props.benefit_allowance_child(sign_opts, benefit_opts, benefit_ords, disabel_opts);
    }

    fn bonus_child_raw(&self, income: i32, benefit: i32, rebated: i32) -> i32 {
        return self.props.bonus_child_raw(income, benefit, rebated);
    }

    fn bonus_child_fix(&self, income: i32, benefit: i32, rebated: i32) -> i32 {
        return self.props.bonus_child_fix(income, benefit, rebated);
    }

    fn taxable_income_supers(&self, income_result: i32, health_result: i32, social_result: i32) -> i32 {
        return self.props.taxable_income_supers(income_result, health_result, social_result);
    }

    fn taxable_income_basis(&self, income_result: i32) -> i32 {
        return self.props.taxable_income_basis(income_result);
    }

    fn rounded_base_advances(&self, income_result: i32, health_result: i32, social_result: i32) -> i32 {
        return self.props.rounded_base_advances(income_result, health_result, social_result);
    }

    fn rounded_raw_base_advances(&self, income_result: i32) -> i32 {
        return self.props.rounded_raw_base_advances(income_result);
    }

    fn rounded_base_solidary(&self, income_result: i32) -> i32 {
        return self.props.rounded_base_solidary(income_result);
    }

    fn rounded_advances_paym(&self, _supers_result: i32, _basis_result: i32) -> i32 {
        let factor_advances = operations_dec::divide(self.factor_advances(), Decimal::from(100));

        let advance_taxing: i32;
        if _basis_result <= self.margin_income_of_rounding() {
            advance_taxing = PropsTaxingBase::int_tax_round_up(operations_dec::multiply(Decimal::from(_supers_result), factor_advances));
            return advance_taxing;
        }
        advance_taxing = PropsTaxingBase::int_tax_round_up(operations_dec::multiply(Decimal::from(_supers_result), factor_advances));
        return advance_taxing;
    }

    fn rounded_solidary_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_solidary_paym(basis_result);
    }

    fn rounded_base_withhold(&self, income_result: i32) -> i32 {
        return self.props.rounded_base_withhold(income_result);
    }

    fn rounded_withhold_paym(&self, supers_result: i32, basis_result: i32) -> i32 {
        return self.props.rounded_withhold_paym(supers_result, basis_result);
    }
}

