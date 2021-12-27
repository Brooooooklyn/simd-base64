use core::fmt;

#[derive(Debug)]
pub struct Error(());

pub(crate) const ERROR: Error = Error(());

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UUID Error")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
