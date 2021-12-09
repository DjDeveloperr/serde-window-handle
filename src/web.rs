use raw_window_handle::RawWindowHandle;
use serde::Serialize;
use serde::Deserialize;
use raw_window_handle::WebHandle;

use crate::SerdeWindowHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeWebHandle {
    pub id: u32,
}

impl Into<WebHandle> for SerdeWebHandle {
    fn into(self) -> WebHandle {
        let mut handle = WebHandle::empty();
        handle.id = self.id;
        handle
    }
}

impl From<WebHandle> for SerdeWebHandle {
    fn from(handle: WebHandle) -> Self {
        Self {
            id: handle.id,
        }
    }
}

impl_serde_handle!(SerdeWebHandle, WebHandle, Web);
