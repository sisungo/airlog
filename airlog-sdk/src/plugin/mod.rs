//! Plugin system.

use once_cell::sync::Lazy;
use std::ffi::{c_char, CStr, CString};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompatInfo {
    abi: *const c_char,
    sdk: *const c_char,
}
impl CompatInfo {
    pub fn new() -> Self {
        static ABI: Lazy<CString> =
            Lazy::new(|| CString::new(super::built_info::RUSTC_VERSION).unwrap());
        static SDK: Lazy<CString> = Lazy::new(|| CString::new(env!("CARGO_PKG_VERSION")).unwrap());

        Self {
            abi: ABI.as_ptr(),
            sdk: SDK.as_ptr(),
        }
    }
}
impl PartialEq for CompatInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CStr::from_ptr(self.abi) == CStr::from_ptr(other.abi)
                && CStr::from_ptr(self.sdk) == CStr::from_ptr(other.sdk)
        }
    }
}

pub type PluginCompatInfoFn = extern "C" fn() -> CompatInfo;
pub type PluginInitFn = extern "Rust" fn(table: Box<dyn PluginTable>) -> ();

#[async_trait::async_trait]
pub trait PluginTable {
    /// Returns a handle to the `airlogd` main async runtime.
    fn async_runtime(&self) -> tokio::runtime::Handle;
}
