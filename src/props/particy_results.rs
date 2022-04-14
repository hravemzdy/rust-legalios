use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::service::contract_terms::{WorkHealthTerms, WorkSocialTerms, WorkTaxingTerms};

#[derive(Debug, Copy, Clone)]
pub struct ParticyHealthTarget {
    contract_code: i16,
    subject_type: WorkTaxingTerms,
    interest_code: i16,
    subject_term: WorkHealthTerms,
    particy_code: i16,
    targets_base: i32,
}
#[allow(dead_code)]
impl ParticyHealthTarget {
    pub(crate) fn new(_contract_code: i16,
                      _subject_type: WorkTaxingTerms,
                      _interest_code: i16,
                      _subject_term: WorkHealthTerms,
                      _particy_code: i16,
                      _targets_base: i32) -> ParticyHealthTarget {
        ParticyHealthTarget {
            contract_code: _contract_code,
            subject_type: _subject_type,
            interest_code: _interest_code,
            subject_term: _subject_term,
            particy_code: _particy_code,
            targets_base: _targets_base,
        }
    }
    pub(crate) fn get_contract_code(self) -> i16 {
        return self.contract_code
    }
    pub(crate) fn get_subject_type(self) -> WorkTaxingTerms {
        return self.subject_type
    }
    pub(crate) fn get_interest_code(self) -> i16 {
        return self.interest_code
    }
    pub(crate) fn get_subject_term(self) -> WorkHealthTerms {
        return self.subject_term
    }
    pub(crate) fn get_particy_code(self) -> i16 {
        return self.particy_code
    }
    pub(crate) fn get_targets_base(self) -> i32 {
        return self.targets_base
    }

    pub(crate) fn add_target_base(&mut self, _targets_base: i32) -> i32 {
        self.targets_base += _targets_base;
        return self.targets_base;
    }
    pub(crate) fn income_score(self) -> i32 {
        let result_type: i32 = match self.subject_type {
            WorkTaxingTerms::TaxingTermEmployments => 900,
            WorkTaxingTerms::TaxingTermAgreemTask => 100,
            WorkTaxingTerms::TaxingTermStatutPart => 500,
            WorkTaxingTerms::TaxingTermByContract => 0,
        };
        let result_base: i32 = match self.subject_term {
            WorkHealthTerms::HealthTermEmployments => 9000,
            WorkHealthTerms::HealthTermAgreemWork => 5000,
            WorkHealthTerms::HealthTermAgreemTask => 4000,
            WorkHealthTerms::HealthTermByContract => 0,
        };
        let mut interest_res: i32 = 0;
        if self.interest_code == 1 {
            interest_res = 10000;
        }
        let mut particy_res: i32 = 0;
        if self.particy_code == 1 {
            particy_res = 100000;
        }
        return result_type + result_base + interest_res + particy_res;
    }
    pub(crate) fn result_comparator() -> impl FnMut(&ParticyHealthTarget, &ParticyHealthTarget) -> Ordering {
        move |x: &ParticyHealthTarget, y: &ParticyHealthTarget| -> Ordering {
            let x_income_score = x.income_score();
            let y_income_score = y.income_score();

            if x_income_score == y_income_score {
                if x.contract_code < y.contract_code {
                    return Less;
                }
                if x.contract_code > y.contract_code {
                    return Greater;
                }
                return Equal;
            }
            if y_income_score < x_income_score {
                return Less;
            }
            if y_income_score > x_income_score {
                return Greater;
            }
            return Equal;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParticyHealthResult {
    contract_code: i16,
    subject_type: WorkTaxingTerms,
    interest_code: i16,
    subject_term: WorkHealthTerms,
    particy_code: i16,
    targets_base: i32,
    results_base: i32,
}

#[allow(dead_code)]
impl ParticyHealthResult {
    pub(crate) fn new(_contract_code: i16,
                      _subject_type: WorkTaxingTerms,
                      _interest_code: i16,
                      _subject_term: WorkHealthTerms,
                      _particy_code: i16,
                      _targets_base: i32,
                      _results_base: i32) -> ParticyHealthResult {
        ParticyHealthResult {
            contract_code: _contract_code,
            subject_type: _subject_type,
            interest_code: _interest_code,
            subject_term: _subject_term,
            particy_code: _particy_code,
            targets_base: _targets_base,
            results_base: _results_base,
        }
    }
    pub(crate) fn get_contract_code(self) -> i16 {
        return self.contract_code
    }
    pub(crate) fn get_subject_type(self) -> WorkTaxingTerms {
        return self.subject_type
    }
    pub(crate) fn get_interest_code(self) -> i16 {
        return self.interest_code
    }
    pub(crate) fn get_subject_term(self) -> WorkHealthTerms {
        return self.subject_term
    }
    pub(crate) fn get_particy_code(self) -> i16 {
        return self.particy_code
    }
    pub(crate) fn get_targets_base(self) -> i32 {
        return self.targets_base
    }
    pub(crate) fn get_results_base(self) -> i32 {
        return self.results_base
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParticySocialTarget {
    contract_code: i16,
    subject_type: WorkTaxingTerms,
    interest_code: i16,
    subject_term: WorkSocialTerms,
    particy_code: i16,
    targets_base: i32,
}
#[allow(dead_code)]
impl ParticySocialTarget {
    pub(crate) fn new(_contract_code: i16,
                      _subject_type: WorkTaxingTerms,
                      _interest_code: i16,
                      _subject_term: WorkSocialTerms,
                      _particy_code: i16,
                      _targets_base: i32) -> ParticySocialTarget {
        ParticySocialTarget {
            contract_code: _contract_code,
            subject_type: _subject_type,
            interest_code: _interest_code,
            subject_term: _subject_term,
            particy_code: _particy_code,
            targets_base: _targets_base,
        }
    }
    pub(crate) fn get_contract_code(self) -> i16 {
        return self.contract_code
    }
    pub(crate) fn get_subject_type(self) -> WorkTaxingTerms {
        return self.subject_type
    }
    pub(crate) fn get_interest_code(self) -> i16 {
        return self.interest_code
    }
    pub(crate) fn get_subject_term(self) -> WorkSocialTerms {
        return self.subject_term
    }
    pub(crate) fn get_particy_code(self) -> i16 {
        return self.particy_code
    }
    pub(crate) fn get_targets_base(self) -> i32 {
        return self.targets_base
    }
    pub(crate) fn add_target_base(&mut self, _targets_base: i32) -> i32 {
        self.targets_base += _targets_base;
        return self.targets_base;
    }
    pub(crate) fn income_score(self) -> i32 {
        let result_type: i32 = match self.subject_type {
            WorkTaxingTerms::TaxingTermEmployments => 900,
            WorkTaxingTerms::TaxingTermAgreemTask => 100,
            WorkTaxingTerms::TaxingTermStatutPart => 500,
            WorkTaxingTerms::TaxingTermByContract => 0,
        };
        let result_base: i32 = match self.subject_term {
            WorkSocialTerms::SocialTermEmployments => 9000,
            WorkSocialTerms::SocialTermSmallsEmpl => 5000,
            WorkSocialTerms::SocialTermShortsMeet => 4000,
            WorkSocialTerms::SocialTermShortsDeny => 0,
            WorkSocialTerms::SocialTermAgreemTask => 0,
            WorkSocialTerms::SocialTermByContract => 0,
        };
        let mut interest_res: i32 = 0;
        if self.interest_code == 1 {
            interest_res = 10000;
        }
        let mut particy_res: i32 = 0;
        if self.particy_code == 1 {
            particy_res = 100000;
        }
        return result_type + result_base + interest_res + particy_res;
    }
    pub(crate) fn result_comparator() -> impl FnMut(&ParticySocialTarget, &ParticySocialTarget) -> Ordering {
        move |x: &ParticySocialTarget, y: &ParticySocialTarget| -> Ordering {
            let x_income_score = x.income_score();
            let y_income_score = y.income_score();

            if x_income_score == y_income_score {
                if x.contract_code < y.contract_code {
                    return Less;
                }
                if x.contract_code > y.contract_code {
                    return Greater;
                }
                return Equal;
            }
            if y_income_score < x_income_score {
                return Less;
            }
            if y_income_score > x_income_score {
                return Greater;
            }
            return Equal;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParticySocialResult {
    contract_code: i16,
    subject_type: WorkTaxingTerms,
    interest_code: i16,
    subject_term: WorkSocialTerms,
    particy_code: i16,
    targets_base: i32,
    results_base: i32,
}
#[allow(dead_code)]
impl ParticySocialResult {
    pub(crate) fn new(_contract_code: i16,
                      _subject_type: WorkTaxingTerms,
                      _interest_code: i16,
                      _subject_term: WorkSocialTerms,
                      _particy_code: i16,
                      _targets_base: i32,
                      _results_base: i32) -> ParticySocialResult {
        ParticySocialResult {
            contract_code: _contract_code,
            subject_type: _subject_type,
            interest_code: _interest_code,
            subject_term: _subject_term,
            particy_code: _particy_code,
            targets_base: _targets_base,
            results_base: _results_base,
        }
    }
    pub(crate) fn get_contract_code(self) -> i16 {
        return self.contract_code
    }
    pub(crate) fn get_subject_type(self) -> WorkTaxingTerms {
        return self.subject_type
    }
    pub(crate) fn get_interest_code(self) -> i16 {
        return self.interest_code
    }
    pub(crate) fn get_subject_term(self) -> WorkSocialTerms {
        return self.subject_term
    }
    pub(crate) fn get_particy_code(self) -> i16 {
        return self.particy_code
    }
    pub(crate) fn get_targets_base(self) -> i32 {
        return self.targets_base
    }
    pub(crate) fn get_results_base(self) -> i32 {
        return self.results_base
    }
}

