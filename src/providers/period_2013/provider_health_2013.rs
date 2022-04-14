use rust_decimal::Decimal;
use crate::factories::provider_factory::BoxHealthProps;
use crate::props::props_health_2012::PropsHealth2012;
use crate::providers::history_const_health::HistoryConstHealth;
use crate::providers::period_2013::history_const_health_2013::{HistoryConstHealth2013, HistoryConstHealth2013var08, HistoryConstHealth2013var11};
use crate::providers::props_provider::{IPropsHealthProvider};
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderHealth2013 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderHealth2013 {
    pub(crate) fn new() -> ProviderHealth2013 {
        ProviderHealth2013 {
            version: VersionId::get(HistoryConstHealth2013::VERSION_CODE)
        }
    }
    fn min_monthly_basis(&self, _period: &dyn IPeriod) -> i32 {
        if _period.is_period_greater_or_equal_than(2013, 8) {
            return HistoryConstHealth2013var08::MIN_MONTHLY_BASIS;
        }
        HistoryConstHealth2013::MIN_MONTHLY_BASIS
    }

    fn max_annuals_basis(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2013::MAX_ANNUALS_BASIS
    }

    fn lim_monthly_state(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2013::LIM_MONTHLY_STATE
    }

    fn lim_monthly_dis50(&self, _period: &dyn IPeriod) -> i32 {
        if _period.is_period_greater_or_equal_than(2013, 11) {
            return HistoryConstHealth2013var11::LIM_MONTHLY_DIS50;
        }
        HistoryConstHealth2013::LIM_MONTHLY_DIS50
    }

    fn factor_compound(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstHealth2013::FACTOR_COMPOUND
    }

    fn factor_employee(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstHealth2013::FACTOR_EMPLOYEE
    }

    fn margin_income_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2013::MARGIN_INCOME_EMP
    }

    fn margin_income_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2013::MARGIN_INCOME_AGR
    }
}

impl IPropsHealthProvider for ProviderHealth2013 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> BoxHealthProps {
        Box::new(PropsHealth2012::new(self.version,
                         self.min_monthly_basis(_period),
                         self.max_annuals_basis(_period),
                         self.lim_monthly_state(_period),
                         self.lim_monthly_dis50(_period),
                         self.factor_compound(_period),
                         self.factor_employee(_period),
                         self.margin_income_emp(_period),
                         self.margin_income_agr(_period)))
    }
}
