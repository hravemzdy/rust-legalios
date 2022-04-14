use rust_decimal::Decimal;
use crate::factories::provider_factory::BoxSocialProps;
use crate::props::props::IProps;
use crate::props::props_social::PropsSocial;
use crate::providers::history_const_social::HistoryConstSocial;
use crate::providers::period_2020::history_const_social_2020::HistoryConstSocial2020;
use crate::providers::props_provider::{IPropsSocialProvider};
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderSocial2020 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderSocial2020 {
    pub(crate) fn new() -> ProviderSocial2020 {
        ProviderSocial2020 {
            version: VersionId::get(HistoryConstSocial2020::VERSION_CODE)
        }
    }
    fn max_annuals_basis(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2020::MAX_ANNUALS_BASIS
    }

    fn factor_employer(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2020::FACTOR_EMPLOYER
    }

    fn factor_employer_higher(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2020::FACTOR_EMPLOYER_HIGHER
    }

    fn factor_employee(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2020::FACTOR_EMPLOYEE
    }

    fn factor_employee_garant(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2020::FACTOR_EMPLOYEE_GARANT
    }

    fn factor_employee_reduce(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2020::FACTOR_EMPLOYEE_REDUCE
    }

    fn margin_income_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2020::MARGIN_INCOME_EMP
    }

    fn margin_income_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2020::MARGIN_INCOME_AGR
    }
}

impl IProps for ProviderSocial2020 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSocialProvider for ProviderSocial2020 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSocialProps {
        Box::new(PropsSocial::new(self.version,
                         self.max_annuals_basis(_period),
                         self.factor_employer(_period),
                         self.factor_employer_higher(_period),
                         self.factor_employee(_period),
                         self.factor_employee_garant(_period),
                         self.factor_employee_reduce(_period),
                         self.margin_income_emp(_period),
                         self.margin_income_agr(_period)))
    }
}
