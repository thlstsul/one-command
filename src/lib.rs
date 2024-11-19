#[cfg(not(feature = "async"))]
include!("blocking.rs");

#[cfg(feature = "async")]
pub mod blocking;

#[cfg(feature = "async")]
use std::ffi::OsStr;

/// Execute commands on the Windows platform,
/// without opening a window to maintain consistency with other system behaviors.
#[cfg(feature = "async")]
pub struct Command;

#[cfg(feature = "async")]
impl Command {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> async_process::Command {
        blocking::Command::new(program).into()
    }
}
