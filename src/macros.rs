#[macro_export]
macro_rules! impl_serde_handle {
    ($serde:ident, $raw:ident, $variant:ident) => {
        impl Into<RawWindowHandle> for $serde {
            fn into(self) -> RawWindowHandle {
                RawWindowHandle::$variant(self.into())
            }
        }

        impl From<$raw> for SerdeWindowHandle {
            fn from(handle: $raw) -> Self {
                Self::$variant($serde::from(handle))
            }
        }

        impl TryFrom<RawWindowHandle> for $serde {
            type Error = &'static str;
        
            fn try_from(value: RawWindowHandle) -> Result<Self, Self::Error> {
                match value {
                    RawWindowHandle::$variant(handle) => Ok(Self::from(handle)),
                    _ => Err("Invalid handle")
                }
            }
        }
    };
}
