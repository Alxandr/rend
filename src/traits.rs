macro_rules! impl_unop {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl ::core::ops::$trait for $name {
            type Output = <$prim as ::core::ops::$trait>::Output;

            #[inline]
            fn $fn(self) -> Self::Output {
                self.to_native().$fn()
            }
        }
    };
}

macro_rules! impl_binop_nonzero {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl_binop_both!($trait::$fn ($name, $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, $prim) -> $prim);

        impl_binop_one!($trait::$fn ($name, $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, $name) -> $prim);
        impl_binop_one!($trait::$fn ($name, &'_ $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, &'_ $name) -> $prim);
    };
}

macro_rules! impl_binop_one {
    ($trait:ident::$fn:ident ($self:ty, $other:ty) -> $output:ty) => {
        impl ::core::ops::$trait<$other> for $self {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $other) -> Self::Output {
                self.to_native().$fn(other.to_native())
            }
        }
    };
}

macro_rules! impl_binop_both {
    ($trait:ident::$fn:ident ($self:ty, $other:ty) -> $output:ty) => {
        impl ::core::ops::$trait<$other> for $self {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $other) -> Self::Output {
                self.to_native().$fn(other)
            }
        }

        impl ::core::ops::$trait<$self> for $other {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $self) -> Self::Output {
                self.$fn(other.to_native())
            }
        }
    };
}

macro_rules! impl_binop {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl_binop_both!($trait::$fn ($name, $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, $prim) -> $prim);
        impl_binop_both!($trait::$fn ($name, &'_ $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, &'_ $prim) -> $prim);

        impl_binop_one!($trait::$fn ($name, $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, $name) -> $prim);
        impl_binop_one!($trait::$fn ($name, &'_ $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, &'_ $name) -> $prim);
    };
}

macro_rules! impl_binassign_nonzero {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl ::core::ops::$trait<$prim> for $name {
            #[inline]
            fn $fn(&mut self, other: $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<$name> for $name {
            #[inline]
            fn $fn(&mut self, other: $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }
    };
}

macro_rules! impl_binassign {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl ::core::ops::$trait<$prim> for $name {
            #[inline]
            fn $fn(&mut self, other: $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<$name> for $name {
            #[inline]
            fn $fn(&mut self, other: $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<&'_ $prim> for $name {
            #[inline]
            fn $fn(&mut self, other: &'_ $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<&'_ $name> for $name {
            #[inline]
            fn $fn(&mut self, other: &'_ $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }
    };
}

macro_rules! impl_clone_and_copy {
    (for $name:ident) => {
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl Copy for $name {}
    };
}

macro_rules! impl_fmt {
    ($trait:ident for $name:ident) => {
        impl ::core::fmt::$trait for $name {
            #[inline]
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.to_native(), f)
            }
        }
    };
}

macro_rules! impl_default {
    (for $name:ident: $prim:ty) => {
        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::from_native(<$prim>::default())
            }
        }
    };
}

macro_rules! impl_from {
    (for $name:ident: $prim:ty) => {
        impl From<$prim> for $name {
            fn from(value: $prim) -> Self {
                Self::from_native(value)
            }
        }

        impl<'a> From<&'a $prim> for $name {
            fn from(value: &'a $prim) -> Self {
                Self::from_native(*value)
            }
        }

        impl From<$name> for $prim {
            fn from(value: $name) -> Self {
                value.to_native()
            }
        }

        impl<'a> From<&'a $name> for $prim {
            fn from(value: &'a $name) -> Self {
                value.to_native()
            }
        }
    };
}

macro_rules! impl_hash {
    (for $name:ident) => {
        impl core::hash::Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.to_native().hash(state);
            }
        }
    };
}

macro_rules! impl_ord {
    (for $name:ident) => {
        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.to_native().cmp(&other.to_native())
            }
        }
    };
}

macro_rules! impl_partial_eq_and_eq {
    (for $name:ident: $prim:ty) => {
        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl PartialEq<$prim> for $name {
            #[inline]
            fn eq(&self, other: &$prim) -> bool {
                self.to_native().eq(other)
            }
        }

        impl PartialEq<$name> for $prim {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self.eq(&other.to_native())
            }
        }

        impl Eq for $name {}
    };
}

macro_rules! impl_partial_ord {
    (for $name:ident: $prim:ty) => {
        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Self,
            ) -> Option<::core::cmp::Ordering> {
                self.to_native().partial_cmp(&other.to_native())
            }
        }

        impl PartialOrd<$prim> for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$prim,
            ) -> Option<::core::cmp::Ordering> {
                self.to_native().partial_cmp(other)
            }
        }
    };
}

macro_rules! impl_product_and_sum {
    (for $name:ident) => {
        impl ::core::iter::Product for $name {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::from_native(iter.map(|x| x.to_native()).product())
            }
        }

        impl ::core::iter::Sum for $name {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::from_native(iter.map(|x| x.to_native()).sum())
            }
        }
    };
}

/// # Safety
///
/// An impl of `CheckBytes` with a `check_bytes` function that is a no-op must
/// be sound for `$name`.
macro_rules! unsafe_impl_check_bytes_noop {
    (for $name:ident) => {
        #[cfg(feature = "validation")]
        impl<C: ?Sized> bytecheck::CheckBytes<C> for $name {
            type Error = core::convert::Infallible;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                _: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                // SAFETY: The invoker of this macro has guaranteed that an impl
                // of `CheckBytes` with a `check_bytes` function that is a no-op
                // is sound.
                Ok(unsafe { &*value })
            }
        }
    };
}
