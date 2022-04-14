use std::fmt;

#[derive(Debug, Clone)]
pub struct HistoryResultError {
    message: String
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[allow(dead_code)]
impl HistoryResultError {
    pub(crate) fn bundle_none_error() -> HistoryResultError {
        HistoryResultError { message: String::from("service hasn't returned bundle") }
    }
    pub(crate) fn bundle_null_error() -> HistoryResultError {
        HistoryResultError { message: String::from("service returned null bundle") }
    }
    pub(crate) fn bundle_empty_error() -> HistoryResultError {
        HistoryResultError { message: String::from("service returned empty bundle") }
    }
    pub(crate) fn bundle_invalid_error() -> HistoryResultError {
        HistoryResultError { message: String::from("service returned invalid bundle") }
    }
}


// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for HistoryResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
