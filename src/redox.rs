use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::OrbitalHandle;
use core::ffi::c_void;

use crate::SerdeWindowHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeOrbitalHandle {
    pub window: usize,
}

impl Into<OrbitalHandle> for SerdeOrbitalHandle {
    fn into(self) -> OrbitalHandle {
        let mut handle = OrbitalHandle::empty();
        handle.window = self.window as *mut c_void;
        handle
    }
}

impl From<OrbitalHandle> for SerdeOrbitalHandle {
    fn from(handle: OrbitalHandle) -> Self {
        Self {
            window: handle.window as usize,
        }
    }
}

impl_serde_handle!(SerdeOrbitalHandle, OrbitalHandle, Orbital);
