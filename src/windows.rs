use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::Win32Handle;
use raw_window_handle::WinRtHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeWin32Handle {
    pub hwnd: usize,
    pub hinstance: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeWinRtHandle {
    pub core_window: usize,
}

impl Into<Win32Handle> for SerdeWin32Handle {
    fn into(self) -> Win32Handle {
        let mut handle = Win32Handle::empty();
        handle.hwnd = self.hwnd as *mut c_void;
        handle.hinstance = self.hinstance as *mut c_void;
        handle
    }
}

impl From<Win32Handle> for SerdeWin32Handle {
    fn from(handle: Win32Handle) -> Self {
        Self {
            hwnd: handle.hwnd as usize,
            hinstance: handle.hinstance as usize,
        }
    }
}

impl_serde_handle!(SerdeWin32Handle, Win32Handle, Win32);

impl Into<WinRtHandle> for SerdeWinRtHandle {
    fn into(self) -> WinRtHandle {
        let mut handle = WinRtHandle::empty();
        handle.core_window = self.core_window as *mut c_void;
        handle
    }
}

impl From<WinRtHandle> for SerdeWinRtHandle {
    fn from(handle: WinRtHandle) -> Self {
        Self {
            core_window: handle.core_window as usize,
        }
    }
}

impl_serde_handle!(SerdeWinRtHandle, WinRtHandle, WinRt);
