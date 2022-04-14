#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum WorkContractTerms {
    WorktermEmployment1 = 0,
    WorktermContracterA = 1,
    WorktermContracterT = 2,
    WorktermPartnerStat = 3,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum WorkTaxingTerms {
    TaxingTermByContract = 0,
    TaxingTermEmployments = 1,
    TaxingTermAgreemTask = 2,
    TaxingTermStatutPart = 3,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum WorkHealthTerms {
    HealthTermByContract = 0,
    HealthTermEmployments = 1,
    HealthTermAgreemWork = 2,
    HealthTermAgreemTask = 3,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum WorkSocialTerms {
    SocialTermByContract = 0,
    SocialTermEmployments = 1,
    SocialTermSmallsEmpl = 2,
    SocialTermShortsMeet = 3,
    SocialTermShortsDeny = 4,
    SocialTermAgreemTask = 5,
}

