// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/force_iterations.fbs".

#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentType};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Specifies how often this force should be applied per iteration.
///
/// Increasing this parameter can lead to better results at the cost of longer computation time.
///
/// ⚠️ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ForceIterations(pub crate::datatypes::UInt64);

impl ::re_types_core::Component for ForceIterations {
    #[inline]
    fn name() -> ComponentType {
        "rerun.blueprint.components.ForceIterations".into()
    }
}

::re_types_core::macros::impl_into_cow!(ForceIterations);

impl ::re_types_core::Loggable for ForceIterations {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::UInt64::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::UInt64::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::UInt64::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::UInt64::from_arrow(arrow_data).map(|v| v.into_iter().map(Self).collect())
    }
}

impl<T: Into<crate::datatypes::UInt64>> From<T> for ForceIterations {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::UInt64> for ForceIterations {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::UInt64 {
        &self.0
    }
}

impl std::ops::Deref for ForceIterations {
    type Target = crate::datatypes::UInt64;

    #[inline]
    fn deref(&self) -> &crate::datatypes::UInt64 {
        &self.0
    }
}

impl std::ops::DerefMut for ForceIterations {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::UInt64 {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for ForceIterations {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::UInt64>::is_pod()
    }
}
