use crate::factories::provider_factory::{BoxHealthProps, BoxSalaryProps, BoxSocialProps, BoxTaxingProps};
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) trait IPropsSalaryProvider {
    fn get_version(&self) -> VersionId;
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSalaryProps;
}
pub(crate) trait IPropsHealthProvider {
    fn get_version(&self) -> VersionId;
    fn get_props(&self, _period: &dyn IPeriod) -> BoxHealthProps;
}
pub(crate) trait IPropsSocialProvider {
    fn get_version(&self) -> VersionId;
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSocialProps;
}
pub(crate) trait IPropsTaxingProvider {
    fn get_version(&self) -> VersionId;
    fn get_props(&self, _period: &dyn IPeriod) -> BoxTaxingProps;
}
