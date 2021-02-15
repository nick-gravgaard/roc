use snafu::{Backtrace, ErrorCompat, Snafu};

//import errors as follows:
// `use crate::error::OutOfBounds;`
// *not* `use crate::error::EdError::OutOfBounds;`
// see https://github.com/shepmaster/snafu/issues/211

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum UIError {
    #[snafu(display(
        "OutOfBounds: index {} was out of bounds for {} with length {}.",
        index,
        collection_name,
        len
    ))]
    OutOfBounds {
        index: usize,
        collection_name: String,
        len: usize,
        backtrace: Backtrace,
    },

    #[snafu(display("InvalidSelection: {}.", err_msg))]
    InvalidSelection {
        err_msg: String,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "FileOpenFailed: failed to open file with path {} with the following error: {}.",
        path_str,
        err_msg
    ))]
    FileOpenFailed { path_str: String, err_msg: String },

    #[snafu(display("TextBufReadFailed: the file {} could be opened but we encountered the following error while trying to read it: {}.", path_str, err_msg))]
    TextBufReadFailed { path_str: String, err_msg: String },
}

pub type UIResult<T, E = UIError> = std::result::Result<T, E>;

impl From<UIError> for String {
    fn from(ui_error: UIError) -> Self {
        format!("{}", ui_error)
    }
}