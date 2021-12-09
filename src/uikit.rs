use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::UiKitHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeUiKitHandle {
    pub ui_window: usize,
    pub ui_view: usize,
    pub ui_view_controller: usize,
}

impl Into<UiKitHandle> for SerdeUiKitHandle {
    fn into(self) -> UiKitHandle {
        let mut handle = UiKitHandle::empty();
        handle.ui_window = self.ui_window as *mut c_void;
        handle.ui_view = self.ui_view as *mut c_void;
        handle.ui_view_controller = self.ui_view_controller as *mut c_void;
        handle
    }
}

impl From<UiKitHandle> for SerdeUiKitHandle {
    fn from(handle: UiKitHandle) -> Self {
        Self {
            ui_window: handle.ui_window as usize,
            ui_view: handle.ui_view as usize,
            ui_view_controller: handle.ui_view_controller as usize,
        }
    }
}

impl_serde_handle!(SerdeUiKitHandle, UiKitHandle, UiKit);
