use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::XlibHandle;
use raw_window_handle::XcbHandle;
use raw_window_handle::WaylandHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeXlibHandle {
    pub window: u64,
    pub display: usize,
    // pub visual_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeXcbHandle {
    pub window: u32,
    pub connection: usize,
    // pub visual_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeWaylandHandle {
    pub surface: usize,
    pub display: usize,
}

impl Into<XlibHandle> for SerdeXlibHandle {
    fn into(self) -> XlibHandle {
        let mut handle = XlibHandle::empty();
        handle.window = self.window;
        handle.display = self.display as *mut c_void;
        // handle.visual_id = self.visual_id;
        handle
    }
}

impl From<XlibHandle> for SerdeXlibHandle {
    fn from(handle: XlibHandle) -> Self {
        Self {
            window: handle.window,
            display: handle.display as usize,
            // visual_id: handle.visual_id,
        }
    }
}

impl_serde_handle!(SerdeXlibHandle, XlibHandle, Xlib);

impl Into<XcbHandle> for SerdeXcbHandle {
    fn into(self) -> XcbHandle {
        let mut handle = XcbHandle::empty();
        handle.window = self.window;
        handle.connection = self.connection as *mut c_void;
        // handle.visual_id = self.visual_id;
        handle
    }
}

impl From<XcbHandle> for SerdeXcbHandle {
    fn from(handle: XcbHandle) -> Self {
        Self {
            window: handle.window,
            connection: handle.connection as usize,
            // visual_id: handle.visual_id,
        }
    }
}

impl_serde_handle!(SerdeXcbHandle, XcbHandle, Xcb);

impl Into<WaylandHandle> for SerdeWaylandHandle {
    fn into(self) -> WaylandHandle {
        let mut handle = WaylandHandle::empty();
        handle.surface = self.surface as *mut c_void;
        handle.display = self.display as *mut c_void;
        handle
    }
}

impl From<WaylandHandle> for SerdeWaylandHandle {
    fn from(handle: WaylandHandle) -> Self {
        Self {
            surface: handle.surface as usize,
            display: handle.display as usize,
        }
    }
}

impl_serde_handle!(SerdeWaylandHandle, WaylandHandle, Wayland);
