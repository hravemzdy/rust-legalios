use crate::service::version_id::VersionId;

pub trait IProps {
    fn get_version(&self) -> VersionId;
}