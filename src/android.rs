use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::AndroidNdkHandle;
use raw_window_handle::RawWindowHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;
use crate::impl_serde_handle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeAndroidNdkHandle {
    pub a_native_window: usize,
}

impl Into<AndroidNdkHandle> for SerdeAndroidNdkHandle {
    fn into(self) -> AndroidNdkHandle {
        let mut handle = AndroidNdkHandle::empty();
        handle.a_native_window = self.a_native_window as *mut c_void;
        handle
    }
}

impl From<AndroidNdkHandle> for SerdeAndroidNdkHandle {
    fn from(handle: AndroidNdkHandle) -> Self {
        Self {
            a_native_window: handle.a_native_window as usize,
        }
    }
}

impl_serde_handle!(SerdeAndroidNdkHandle, AndroidNdkHandle, AndroidNdk);
