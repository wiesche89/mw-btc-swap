pub use crate::lib::clone::Clone;
pub use crate::lib::convert::{From, Into};
pub use crate::lib::default::Default;
pub use crate::lib::fmt::{self, Formatter};
pub use crate::lib::marker::PhantomData;
pub use crate::lib::option::Option::{self, None, Some};
pub use crate::lib::result::Result::{self, Err, Ok};

pub use self::string::from_utf8_lossy;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::lib::{ToString, Vec};

#[cfg(core_try_from)]
pub use crate::lib::convert::TryFrom;

mod string {
    use crate::lib::*;

    /// Standard library fallback for `from_utf8_lossy`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }

    /// Fallback for `String::from_utf8_lossy` when `alloc`/`std` are not available.
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> &str {
        str::from_utf8(bytes).unwrap_or("\u{fffd}\u{fffd}\u{fffd}")
    }
}
