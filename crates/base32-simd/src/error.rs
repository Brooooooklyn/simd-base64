use core::fmt;

/// Base32 Error
pub struct Error(());

impl Error {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Error(())
    }
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt("Base32Error", f)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Display>::fmt("Base32Error", f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err($crate::error::Error::new());
        }
    };
}

#[allow(unused_macros)]
macro_rules! try_ {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return Err(Error::new()),
        }
    };
}
