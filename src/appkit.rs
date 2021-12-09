use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::AppKitHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;
use crate::impl_serde_handle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeAppKitHandle {
    pub ns_window: usize,
    pub ns_view: usize,
}

impl Into<AppKitHandle> for SerdeAppKitHandle {
    fn into(self) -> AppKitHandle {
        let mut handle = AppKitHandle::empty();
        handle.ns_window = self.ns_window as *mut c_void;
        handle.ns_view = self.ns_view as *mut c_void;
        handle
    }
}

impl From<AppKitHandle> for SerdeAppKitHandle {
    fn from(handle: AppKitHandle) -> Self {
        Self {
            ns_window: handle.ns_window as usize,
            ns_view: handle.ns_view as usize,
        }
    }
}

impl_serde_handle!(SerdeAppKitHandle, AppKitHandle, AppKit);
