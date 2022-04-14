use crate::factories::provider_factory::BoxSalaryProps;
use crate::props::props::IProps;
use crate::props::props_salary::PropsSalary;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2011::history_const_salary_2011::HistoryConstSalary2011;
use crate::providers::props_provider::{IPropsSalaryProvider};
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderSalary2011 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderSalary2011 {
    pub(crate) fn new() -> ProviderSalary2011 {
        ProviderSalary2011 {
            version: VersionId::get(HistoryConstSalary2011::VERSION_CODE)
        }
    }
    fn working_shift_week(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2011::WORKING_SHIFT_WEEK
    }

    fn working_shift_time(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2011::WORKING_SHIFT_TIME
    }
    fn min_monthly_wage(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2011::MIN_MONTHLY_WAGE
    }
    fn min_hourly_wage(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSalary2011::MIN_HOURLY_WAGE
    }
}

impl IProps for ProviderSalary2011 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSalaryProvider for ProviderSalary2011 {
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

