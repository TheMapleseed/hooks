//! Shared macros for typed provider event enums.

/// Define a provider event enum with `as_str`, `Display`, and `AsRef<str>`.
#[macro_export]
macro_rules! provider_events {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $variant:ident => $wire:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Provider-canonical event / topic string.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $wire,)+
                }
            }

            /// Every known event variant for this provider.
            #[must_use]
            pub const fn all() -> &'static [Self] {
                &[$(Self::$variant,)+]
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}
