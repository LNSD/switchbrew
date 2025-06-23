//! Handle types.

/// Internal macro to generate `raw::Handle` newtypes with common helpers.
macro_rules! define_handle_type {
    {
        $(#[$meta:meta])* $vis:vis struct $name:ident
    } => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $name($crate::raw::Handle);

        impl $name {
            /// Converts a raw handle to a [`$name`].
            ///
            /// # Safety
            ///
            /// Caller must guarantee that the raw handle is valid.
            pub unsafe fn from_raw(raw: $crate::raw::Handle) -> Self {
                Self(raw)
            }

            /// Returns `true` if the handle is valid.
            pub fn is_valid(&self) -> bool {
                self.0 != $crate::raw::INVALID_HANDLE
            }

            /// Converts the [`$name`] to a raw handle.
            pub fn to_raw(&self) -> $crate::raw::Handle {
                self.0
            }
        }

        impl ::core::cmp::PartialEq<$crate::raw::Handle> for $name {
            fn eq(&self, other: &$crate::raw::Handle) -> bool {
                &self.0 == other
            }
        }

        impl ::core::cmp::PartialEq<$name> for $crate::raw::Handle {
            fn eq(&self, other: &$name) -> bool {
                self == &other.0
            }
        }
    };
}
