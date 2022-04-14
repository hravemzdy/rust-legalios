use crate::factories::bundle_builder::{BundleBuilder, IBundleBuilder};
use crate::service::period::IPeriod;
use crate::service::bundle_props::{BundleProps};
use crate::service::history_result_errors::HistoryResultError;

pub trait IServiceLegalios {
    fn get_bundle(&self, _period: &dyn IPeriod) -> Result<BundleProps, HistoryResultError>;
}

pub struct ServiceLegalios {
    builder: BundleBuilder
}

impl IServiceLegalios for ServiceLegalios {
    fn get_bundle(&self, _period: &dyn IPeriod) -> Result<BundleProps, HistoryResultError> {
        let option_bundle = self.builder.get_bundle(_period);
        match option_bundle {
            Some(bundle) => Ok(bundle),
            None => Err(HistoryResultError::bundle_none_error()),
        }
    }
}

#[allow(dead_code)]
impl ServiceLegalios {
    pub fn new() -> ServiceLegalios {
        ServiceLegalios {
            builder: BundleBuilder::new()
        }
    }
}