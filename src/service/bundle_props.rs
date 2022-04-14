use crate::factories::provider_factory::{BoxHealthProps, BoxSalaryProps, BoxSocialProps, BoxTaxingProps};
use crate::props::props_health::PropsHealth;
use crate::service::period::{IPeriod, Period};
use crate::props::props_salary::PropsSalary;
use crate::props::props_social::PropsSocial;
use crate::props::props_taxing::PropsTaxing;

pub trait IBundleProps {
    fn period_props(&self) -> Period;
    fn salary_props(&self) -> &BoxSalaryProps;
    fn health_props(&self) -> &BoxHealthProps;
    fn social_props(&self) -> &BoxSocialProps;
    fn taxing_props(&self) -> &BoxTaxingProps;

    fn get_period_year(&self) -> i16;
    fn get_period_month(&self) -> i16;
    fn get_period_code(&self) -> i32;
}

pub struct BundleProps {
    period_props: Period,
    salary_props: BoxSalaryProps,
    health_props: BoxHealthProps,
    social_props: BoxSocialProps,
    taxing_props: BoxTaxingProps,
}

impl BundleProps {
    pub(crate) fn new(period: &dyn IPeriod, salary: BoxSalaryProps, health: BoxHealthProps, social: BoxSocialProps, taxing: BoxTaxingProps) -> BundleProps {
        BundleProps {
            period_props: Period::get(period.get_code()),
            salary_props: salary,
            health_props: health,
            social_props: social,
            taxing_props: taxing,
        }
    }
    pub fn empty(period: &dyn IPeriod) -> BundleProps {
        BundleProps {
            period_props: Period::get(period.get_code()),
            salary_props: Box::new(PropsSalary::empty()),
            health_props: Box::new(PropsHealth::empty()),
            social_props: Box::new(PropsSocial::empty()),
            taxing_props: Box::new(PropsTaxing::empty()),
        }
    }
}

impl IBundleProps for BundleProps {
    fn period_props(&self) -> Period {
        self.period_props
    }
    fn salary_props(&self) -> &BoxSalaryProps {
        &self.salary_props
    }
    fn health_props(&self) -> &BoxHealthProps {
        &self.health_props
    }
    fn social_props(&self) -> &BoxSocialProps {
        &self.social_props
    }
    fn taxing_props(&self) -> &BoxTaxingProps {
        &self.taxing_props
    }
    fn get_period_year(&self) -> i16 {
        self.period_props.year()
    }
    fn get_period_month(&self) -> i16 {
        self.period_props.month()
    }
    fn get_period_code(&self) -> i32 {
        self.period_props.get_code()
    }
}