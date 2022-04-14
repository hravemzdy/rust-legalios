#[derive(Debug, Copy, Clone)]
pub struct VersionId {
    pub(crate) value: i16
}

#[allow(dead_code)]
impl VersionId {
    pub(crate) fn new() -> VersionId {
        VersionId { value: 0 }
    }
    pub fn get(val: i16) -> VersionId {
        VersionId { value: val }
    }
}
