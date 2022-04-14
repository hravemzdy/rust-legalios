use crate::factories::provider_factory::BoxSalaryProps;
use crate::props::props::IProps;
use crate::props::props_salary::PropsSalary;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2013::history_const_salary_2013::{HistoryConstSalary2013, HistoryConstSalary2013var08};
use crate::providers::props_provider::{IPropsSalaryProvider};
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderSalary2013 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderSalary2013 {
    pub(crate) fn new() -> ProviderSalary2013 {
        ProviderSalary2013 {
            version: VersionId::get(HistoryConstSalary2013::VERSION_CODE)
        }
    }
    fn working_shift_week(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2013::WORKING_SHIFT_WEEK
    }

    fn working_shift_time(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2013::WORKING_SHIFT_TIME
    }
    fn min_monthly_wage(&self, _period: &dyn IPeriod) -> i32 {
        if _period.is_period_greater_or_equal_than(2013, 8) {
            return HistoryConstSalary2013var08::MIN_MONTHLY_WAGE;
        }
        HistoryConstSalary2013::MIN_MONTHLY_WAGE
    }
    fn min_hourly_wage(&self, _period: &dyn IPeriod) -> i32 {
        if _period.is_period_greater_or_equal_than(2013, 8) {
            return HistoryConstSalary2013var08::MIN_HOURLY_WAGE;
        }
        HistoryConstSalary2013::MIN_HOURLY_WAGE
    }
}

impl IProps for ProviderSalary2013 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSalaryProvider for ProviderSalary2013 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSalaryProps {
        Box::new(PropsSalary::new(self.version,
                         self.working_shift_week(_period),
                         self.working_shift_time(_period),
                         self.min_monthly_wage(_period),
                         self.min_hourly_wage(_period)))
    }
}

