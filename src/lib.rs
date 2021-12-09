mod android;
mod appkit;
#[macro_use]
mod macros;
mod redox;
mod uikit;
mod unix;
mod web;
mod windows;

pub use raw_window_handle;

use raw_window_handle::RawWindowHandle;
pub use android::SerdeAndroidNdkHandle;
pub use appkit::SerdeAppKitHandle;
pub use redox::SerdeOrbitalHandle;
pub use uikit::SerdeUiKitHandle;
pub use unix::SerdeXlibHandle;
pub use unix::SerdeXcbHandle;
pub use unix::SerdeWaylandHandle;
pub use web::SerdeWebHandle;
pub use windows::SerdeWin32Handle;
pub use windows::SerdeWinRtHandle;

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerdeWindowHandle {
    UiKit(SerdeUiKitHandle),
    AppKit(SerdeAppKitHandle),
    Orbital(SerdeOrbitalHandle),
    Xlib(SerdeXlibHandle),
    Xcb(SerdeXcbHandle),
    Wayland(SerdeWaylandHandle),
    Win32(SerdeWin32Handle),
    WinRt(SerdeWinRtHandle),
    Web(SerdeWebHandle),
    AndroidNdk(SerdeAndroidNdkHandle),
}

impl Into<RawWindowHandle> for SerdeWindowHandle {
    fn into(self) -> RawWindowHandle {
        match self {
            SerdeWindowHandle::UiKit(handle) => handle.into(),
            SerdeWindowHandle::AppKit(handle) => handle.into(),
            SerdeWindowHandle::Orbital(handle) => handle.into(),
            SerdeWindowHandle::Xlib(handle) => handle.into(),
            SerdeWindowHandle::Xcb(handle) => handle.into(),
            SerdeWindowHandle::Wayland(handle) => handle.into(),
            SerdeWindowHandle::Win32(handle) => handle.into(),
            SerdeWindowHandle::WinRt(handle) => handle.into(),
            SerdeWindowHandle::Web(handle) => handle.into(),
            SerdeWindowHandle::AndroidNdk(handle) => handle.into(),
        }
    }
}

impl From<RawWindowHandle> for SerdeWindowHandle {
    fn from(value: RawWindowHandle) -> Self {
        match value {
            RawWindowHandle::UiKit(handle) => Self::from(handle),
            RawWindowHandle::AppKit(handle) => Self::from(handle),
            RawWindowHandle::Orbital(handle) => Self::from(handle),
            RawWindowHandle::Xlib(handle) => Self::from(handle),
            RawWindowHandle::Xcb(handle) => Self::from(handle),
            RawWindowHandle::Wayland(handle) => Self::from(handle),
            RawWindowHandle::Win32(handle) => Self::from(handle),
            RawWindowHandle::WinRt(handle) => Self::from(handle),
            RawWindowHandle::Web(handle) => Self::from(handle),
            RawWindowHandle::AndroidNdk(handle) => Self::from(handle),
            _ => unreachable!(),
        }
    }
}
