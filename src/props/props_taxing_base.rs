use std::cmp::{max, min};
use std::ops::Neg;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::factories::provider_factory::BoxTaxingProps;
use crate::props::props::IProps;
use crate::service::contract_terms::WorkTaxingTerms;
use crate::service::taxing_options::{TaxDeclBenfOption, TaxDeclDisabOption, TaxDeclSignOption, TaxNoneSignOption};
use crate::service::{operations_dec, operations_round};
use crate::service::version_id::VersionId;

pub trait IPropsTaxing : IProps {
    fn allowance_payer(&self) -> i32;
    fn allowance_disab1st(&self) -> i32;
    fn allowance_disab2nd(&self) -> i32;
    fn allowance_disab3rd(&self) -> i32;
    fn allowance_study(&self) -> i32;
    fn allowance_child1st(&self) -> i32;
    fn allowance_child2nd(&self) -> i32;
    fn allowance_child3rd(&self) -> i32;
    fn factor_advances(&self) -> Decimal;
    fn factor_withhold(&self) -> Decimal;
    fn factor_solidary(&self) -> Decimal;
    fn factor_taxrate2(&self) -> Decimal;
    fn min_amount_of_tax_bonus(&self) -> i32;
    fn max_amount_of_tax_bonus(&self) -> i32;
    fn margin_income_of_tax_bonus(&self) -> i32;
    fn margin_income_of_rounding(&self) -> i32;
    fn margin_income_of_withhold(&self) -> i32;
    fn margin_income_of_solidary(&self) -> i32;
    fn margin_income_of_taxrate2(&self) -> i32;
    fn margin_income_of_wth_emp(&self) -> i32;
    fn margin_income_of_wth_agr(&self) -> i32;

    fn value_equals(&self, other_taxing: &BoxTaxingProps) -> bool;
    fn has_withhold_income(&self, _term_opt: WorkTaxingTerms, _sgn_opt: TaxDeclSignOption, _none_opt: TaxNoneSignOption, _income_sum: i32) -> bool { false }
    fn benefit_allowance_payer(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32;
    fn benefit_allowance_disab(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclDisabOption) -> i32;
    fn benefit_allowance_study(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32;
    fn benefit_allowance_child(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption, benefit_ords: i32, disabel_opts: i32) -> i32;
    fn bonus_child_raw(&self, income: i32, benefit: i32, rebated: i32) -> i32;
    fn bonus_child_fix(&self, income: i32, benefit: i32, rebated: i32) -> i32;
    fn taxable_income_supers(&self, income_result: i32, health_result: i32, social_result: i32) -> i32;
    fn taxable_income_basis(&self, income_result: i32) -> i32;
    fn rounded_base_advances(&self, income_result: i32, health_result: i32, social_result: i32) -> i32;
    fn rounded_raw_base_advances(&self, income_result: i32) -> i32;
    fn rounded_base_solidary(&self, income_result: i32) -> i32;
    fn rounded_advances_paym(&self, _supers_result: i32, _basis_result: i32) -> i32 { 0 }
    fn rounded_solidary_paym(&self, basis_result: i32) -> i32;
    fn rounded_base_withhold(&self, income_result: i32) -> i32;
    fn rounded_withhold_paym(&self, supers_result: i32, basis_result: i32) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsTaxingBase {
    version: VersionId,
    allowance_payer: i32,
    allowance_disab1st: i32,
    allowance_disab2nd: i32,
    allowance_disab3rd: i32,
    allowance_study: i32,
    allowance_child1st: i32,
    allowance_child2nd: i32,
    allowance_child3rd: i32,
    factor_advances: Decimal,
    factor_withhold: Decimal,
    factor_solidary: Decimal,
    factor_taxrate2: Decimal,
    min_amount_of_tax_bonus: i32,
    max_amount_of_tax_bonus: i32,
    margin_income_of_tax_bonus: i32,
    margin_income_of_rounding: i32,
    margin_income_of_withhold: i32,
    margin_income_of_solidary: i32,
    margin_income_of_taxrate2: i32,
    margin_income_of_wth_emp: i32,
    margin_income_of_wth_agr: i32
}

#[allow(dead_code)]
impl PropsTaxingBase {
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
                      _margin_income_of_wth_agr: i32) -> PropsTaxingBase {
        PropsTaxingBase {
            version: _version,
            allowance_payer: _allowance_payer,
            allowance_disab1st: _allowance_disab1st,
            allowance_disab2nd: _allowance_disab2nd,
            allowance_disab3rd: _allowance_disab3rd,
            allowance_study: _allowance_study,
            allowance_child1st: _allowance_child1st,
            allowance_child2nd: _allowance_child2nd,
            allowance_child3rd: _allowance_child3rd,
            factor_advances: _factor_advances,
            factor_withhold: _factor_withhold,
            factor_solidary: _factor_solidary,
            factor_taxrate2: _factor_taxrate2,
            min_amount_of_tax_bonus: _min_amount_of_tax_bonus,
            max_amount_of_tax_bonus: _max_amount_of_tax_bonus,
            margin_income_of_tax_bonus: _margin_income_of_tax_bonus,
            margin_income_of_rounding: _margin_income_of_rounding,
            margin_income_of_withhold: _margin_income_of_withhold,
            margin_income_of_solidary: _margin_income_of_solidary,
            margin_income_of_taxrate2: _margin_income_of_taxrate2,
            margin_income_of_wth_emp: _margin_income_of_wth_emp,
            margin_income_of_wth_agr: _margin_income_of_wth_agr
        }
    }
    pub(crate) fn empty() -> PropsTaxingBase {
        PropsTaxingBase {
            version: VersionId::new(),
            allowance_payer: 0,
            allowance_disab1st: 0,
            allowance_disab2nd: 0,
            allowance_disab3rd: 0,
            allowance_study: 0,
            allowance_child1st: 0,
            allowance_child2nd: 0,
            allowance_child3rd: 0,
            factor_advances: Decimal::ZERO,
            factor_withhold: Decimal::ZERO,
            factor_solidary: Decimal::ZERO,
            factor_taxrate2: Decimal::ZERO,
            min_amount_of_tax_bonus: 0,
            max_amount_of_tax_bonus: 0,
            margin_income_of_tax_bonus: 0,
            margin_income_of_rounding: 0,
            margin_income_of_withhold: 0,
            margin_income_of_solidary: 0,
            margin_income_of_taxrate2: 0,
            margin_income_of_wth_emp: 0,
            margin_income_of_wth_agr: 0
        }
    }
    pub(crate) fn int_tax_round_up(value_dec: Decimal) -> i32 {
        return operations_round::round_up(value_dec);
    }
    pub(crate) fn int_tax_round_near_up(value_dec: Decimal, nearest: i32) -> i32 {
        return operations_round::near_round_up(value_dec, nearest);
    }
    pub(crate) fn int_tax_round_near_up100(value_dec: Decimal) -> i32 {
        return operations_round::near_round_up100(value_dec);
    }
    pub(crate) fn int_tax_round_down(value_dec: Decimal ) -> i32 {
        return operations_round::round_down(value_dec);
    }
    pub(crate) fn int_tax_round_near_down(value_dec: Decimal, nearest: i32) -> i32 {
        return operations_round::near_round_down(value_dec, nearest);
    }
    pub(crate) fn int_tax_round_near_down100(value_dec: Decimal) -> i32 {
        return operations_round::near_round_down100(value_dec);
    }
    pub(crate) fn dec_tax_round_up(value_dec: Decimal) -> Decimal {
        return operations_round::dec_round_up(value_dec)
    }
    pub(crate) fn dec_tax_round_near_up(value_dec: Decimal, nearest: i32) -> Decimal {
        return operations_round::dec_near_round_up(value_dec, nearest);
    }
    pub(crate) fn dec_tax_round_near_up100(value_dec: Decimal) -> Decimal {
        return operations_round::dec_near_round_up100(value_dec);
    }
    pub(crate) fn dec_tax_round_down(value_dec: Decimal) -> Decimal {
        return operations_round::dec_round_down(value_dec);
    }
    pub(crate) fn dec_tax_round_near_down(value_dec: Decimal, nearest: i32) -> Decimal {
        return operations_round::dec_near_round_down(value_dec, nearest);
    }
    pub(crate) fn dec_tax_round_near_down100(value_dec: Decimal) -> Decimal {
        return operations_round::dec_near_round_down100(value_dec);
    }
}

impl IProps for PropsTaxingBase {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsTaxing for PropsTaxingBase {
    fn allowance_payer(&self) -> i32 {
        self.allowance_payer
    }

    fn allowance_disab1st(&self) -> i32 {
        self.allowance_disab1st
    }

    fn allowance_disab2nd(&self) -> i32 {
        self.allowance_disab2nd
    }

    fn allowance_disab3rd(&self) -> i32 {
        self.allowance_disab3rd
    }

    fn allowance_study(&self) -> i32 {
        self.allowance_study
    }

    fn allowance_child1st(&self) -> i32 {
        self.allowance_child1st
    }

    fn allowance_child2nd(&self) -> i32 {
        self.allowance_child2nd
    }

    fn allowance_child3rd(&self) -> i32 {
        self.allowance_child3rd
    }

    fn factor_advances(&self) -> Decimal {
        self.factor_advances
    }

    fn factor_withhold(&self) -> Decimal {
        self.factor_withhold
    }

    fn factor_solidary(&self) -> Decimal {
        self.factor_solidary
    }

    fn factor_taxrate2(&self) -> Decimal {
        self.factor_taxrate2
    }

    fn min_amount_of_tax_bonus(&self) -> i32 {
        self.min_amount_of_tax_bonus
    }

    fn max_amount_of_tax_bonus(&self) -> i32 {
        self.max_amount_of_tax_bonus
    }

    fn margin_income_of_tax_bonus(&self) -> i32 {
        self.margin_income_of_tax_bonus
    }

    fn margin_income_of_rounding(&self) -> i32 {
        self.margin_income_of_rounding
    }

    fn margin_income_of_withhold(&self) -> i32 {
        self.margin_income_of_withhold
    }

    fn margin_income_of_solidary(&self) -> i32 {
        self.margin_income_of_solidary
    }

    fn margin_income_of_taxrate2(&self) -> i32 {
        self.margin_income_of_taxrate2
    }

    fn margin_income_of_wth_emp(&self) -> i32 {
        self.margin_income_of_wth_emp
    }

    fn margin_income_of_wth_agr(&self) -> i32 {
        self.margin_income_of_wth_agr
    }

    fn value_equals(&self, other_taxing: &BoxTaxingProps) -> bool {
        return self.allowance_payer == other_taxing.allowance_payer() &&
            self.allowance_disab1st == other_taxing.allowance_disab1st() &&
            self.allowance_disab2nd == other_taxing.allowance_disab2nd() &&
            self.allowance_disab3rd == other_taxing.allowance_disab3rd() &&
            self.allowance_study == other_taxing.allowance_study() &&
            self.allowance_child1st == other_taxing.allowance_child1st() &&
            self.allowance_child2nd == other_taxing.allowance_child2nd() &&
            self.allowance_child3rd == other_taxing.allowance_child3rd() &&
            self.factor_advances == other_taxing.factor_advances() &&
            self.factor_withhold == other_taxing.factor_withhold() &&
            self.factor_solidary == other_taxing.factor_solidary() &&
            self.factor_taxrate2 == other_taxing.factor_taxrate2() &&
            self.min_amount_of_tax_bonus == other_taxing.min_amount_of_tax_bonus() &&
            self.max_amount_of_tax_bonus == other_taxing.max_amount_of_tax_bonus() &&
            self.margin_income_of_tax_bonus == other_taxing.margin_income_of_tax_bonus() &&
            self.margin_income_of_rounding == other_taxing.margin_income_of_rounding() &&
            self.margin_income_of_withhold == other_taxing.margin_income_of_withhold() &&
            self.margin_income_of_solidary == other_taxing.margin_income_of_solidary() &&
            self.margin_income_of_taxrate2 == other_taxing.margin_income_of_taxrate2() &&
            self.margin_income_of_wth_emp == other_taxing.margin_income_of_wth_emp() &&
            self.margin_income_of_wth_agr == other_taxing.margin_income_of_wth_agr();
    }

    fn benefit_allowance_payer(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32 {
        let benefit_value: i32 = match sign_opts {
            TaxDeclSignOption::DeclTaxDoSigned =>
                match benefit_opts {
                    TaxDeclBenfOption::DeclTaxBenef1 => self.allowance_payer,
                    _ => 0,
                },
            _ => 0,
        };
        return benefit_value;
    }

    fn benefit_allowance_disab(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclDisabOption) -> i32 {
        let benefit_value: i32 =  match  sign_opts  {
            TaxDeclSignOption::DeclTaxDoSigned =>
                match benefit_opts {
                    TaxDeclDisabOption::DeclTaxDisab1 => self.allowance_disab1st,
                    TaxDeclDisabOption::DeclTaxDisab2 => self.allowance_disab2nd,
                    TaxDeclDisabOption::DeclTaxDisab3 => self.allowance_disab3rd,
                    _ => 0,
                },
            _ => 0,
        };
        return benefit_value;
    }

    fn benefit_allowance_study(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption) -> i32 {
        let benefit_value: i32 =  match sign_opts {
            TaxDeclSignOption::DeclTaxDoSigned =>
                match benefit_opts {
                    TaxDeclBenfOption::DeclTaxBenef1 => self.allowance_study,
                    _ => 0,
                },
            _ => 0,
        };
        return benefit_value;
    }

    fn benefit_allowance_child(&self, sign_opts: TaxDeclSignOption, benefit_opts: TaxDeclBenfOption, benefit_ords: i32, disabel_opts: i32) -> i32 {
        let benefit_units: i32 = match sign_opts {
            TaxDeclSignOption::DeclTaxDoSigned =>
                match benefit_ords {
                    0 => self.allowance_child1st,
                    1 => self.allowance_child2nd,
                    2 => self.allowance_child3rd,
                    _ => 0,
                },
            _ => 0,
        };
        let benefit_value: i32 = match benefit_opts {
            TaxDeclBenfOption::DeclTaxBenef1 =>
                match disabel_opts {
                    1 => benefit_units * 2,
                    _ => benefit_units,
                },
            _ => 0,
        };
        return benefit_value;
    }

    fn bonus_child_raw(&self, income: i32, benefit: i32, rebated: i32) -> i32 {
        let bonus_for_child_neg: Decimal = Decimal::from_i32(min(0, rebated - benefit)).unwrap();
        let mut bonus_for_child: Decimal = bonus_for_child_neg.neg();

        if self.margin_income_of_tax_bonus > 0 {
            if income < self.margin_income_of_tax_bonus {
                bonus_for_child = Decimal::ZERO;
            }
        }
        return operations_round::round_to_int(bonus_for_child);
    }

    fn bonus_child_fix(&self, income: i32, benefit: i32, rebated: i32) -> i32 {
        let child_bonus = self.bonus_child_raw(income, benefit, rebated);

        if self.min_amount_of_tax_bonus > 0 {
            if child_bonus < self.min_amount_of_tax_bonus {
                return 0;
            }
        }
        if self.max_amount_of_tax_bonus > 0 {
            if child_bonus > self.max_amount_of_tax_bonus {
                return self.max_amount_of_tax_bonus;
            }
        }
        return child_bonus;
    }

    fn taxable_income_supers(&self, income_result: i32, health_result: i32, social_result: i32) -> i32 {
        return self.taxable_income_basis(income_result + health_result + social_result);
    }

    fn taxable_income_basis(&self, income_result: i32) -> i32 {
        let taxable_super: i32 = max(0, income_result);
        return taxable_super
    }

    fn rounded_base_advances(&self, income_result: i32, health_result: i32, social_result: i32) -> i32 {
        let advance_base = self.rounded_raw_base_advances(income_result + health_result + social_result);

        return advance_base;
    }

    fn rounded_raw_base_advances(&self, income_result: i32) -> i32 {
        let amount_for_calc: i32 = self.taxable_income_basis(income_result);

        let advance_base: i32;
        if amount_for_calc <= self.margin_income_of_rounding {
            advance_base = Self::int_tax_round_up(Decimal::from_i32(amount_for_calc).unwrap());
        }
        else
        {
            advance_base = Self::int_tax_round_near_up100(Decimal::from_i32(amount_for_calc).unwrap());
        }
        return advance_base;
    }

    fn rounded_base_solidary(&self, income_result: i32) -> i32 {
        let mut solidary_base: i32 = 0;

        let taxable_income: i32 = max(0, income_result);
        if self.margin_income_of_solidary != 0 {
            solidary_base = max(0, taxable_income - self.margin_income_of_solidary);
        }
        return solidary_base;
    }

    fn rounded_solidary_paym(&self, basis_result: i32) -> i32 {
        let factor_solidary = operations_dec::divide(self.factor_solidary, Decimal::from_i32(100).unwrap());

        let mut solidary_taxing: i32 = 0;
        if self.margin_income_of_solidary != 0 {
            solidary_taxing = Self::int_tax_round_up(operations_dec::multiply(Decimal::from_i32(basis_result).unwrap(), factor_solidary));
        }

        return solidary_taxing;
    }

    fn rounded_base_withhold(&self, income_result: i32) -> i32 {
        let amount_for_calc: i32 = max(0, income_result);
        let withhold_base: i32 = Self::int_tax_round_down(Decimal::from_i32(amount_for_calc).unwrap());

        return withhold_base;
    }

    fn rounded_withhold_paym(&self, supers_result: i32, _basis_result: i32) -> i32 {
        let factor_withhold = operations_dec::divide(self.factor_withhold, Decimal::from_i32(100).unwrap());

        let mut withhold_taxing: i32 = max(0, supers_result);
        if withhold_taxing > 0 {
            withhold_taxing = Self::int_tax_round_down(operations_dec::multiply(Decimal::from_i32(supers_result).unwrap(), factor_withhold));
        }
        return withhold_taxing;
    }
}

