// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A depth image.
///
/// The shape of the `TensorData` must be mappable to an `HxW` tensor.
/// Each pixel corresponds to a depth value in units specified by `meter`.
///
/// ## Example
///
/// ### Depth to 3D example
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) =
///         rerun::RecordingStreamBuilder::new("rerun_example_depth_image").memory()?;
///
///     let mut image = Array::<u16, _>::from_elem((8, 12).f(), 65535);
///     image.slice_mut(s![0..4, 0..6]).fill(20000);
///     image.slice_mut(s![4..8, 6..12]).fill(45000);
///
///     let depth_image = rerun::DepthImage::try_from(image.clone())?.with_meter(10000.0);
///
///     // If we log a pinhole camera model, the depth gets automatically back-projected to 3D
///     rec.log(
///         "world/camera",
///         &rerun::Pinhole::from_focal_length_and_resolution(
///             [20.0, 20.0],
///             [image.shape()[1] as f32, image.shape()[0] as f32],
///         ),
///     )?;
///
///     rec.log("world/camera/depth", &depth_image)?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/1200w.png">
///   <img src="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct DepthImage {
    /// The depth-image data. Should always be a rank-2 tensor.
    pub data: crate::components::TensorData,

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    pub meter: Option<crate::components::DepthMeter>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.TensorData".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.DepthImageIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.DepthMeter".into(),
            "rerun.components.DrawOrder".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.TensorData".into(),
            "rerun.components.DepthImageIndicator".into(),
            "rerun.components.DepthMeter".into(),
            "rerun.components.DrawOrder".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

impl DepthImage {
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`DepthImage`] [`::re_types_core::Archetype`]
pub type DepthImageIndicator = ::re_types_core::GenericIndicatorComponent<DepthImage>;

impl ::re_types_core::Archetype for DepthImage {
    type Indicator = DepthImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.DepthImage".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: DepthImageIndicator = DepthImageIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow(
        arrow_data: impl IntoIterator<Item = (arrow2::datatypes::Field, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let data = {
            let array = arrays_by_name
                .get("rerun.components.TensorData")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#data")?;
            <crate::components::TensorData>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#data")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#data")?
        };
        let meter = if let Some(array) = arrays_by_name.get("rerun.components.DepthMeter") {
            Some({
                <crate::components::DepthMeter>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.DepthImage#meter")?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(DeserializationError::missing_data)
                    .with_context("rerun.archetypes.DepthImage#meter")?
            })
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            Some({
                <crate::components::DrawOrder>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.DepthImage#draw_order")?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(DeserializationError::missing_data)
                    .with_context("rerun.archetypes.DepthImage#draw_order")?
            })
        } else {
            None
        };
        Ok(Self {
            data,
            meter,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for DepthImage {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.data as &dyn ComponentBatch).into()),
            self.meter
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }
}

impl DepthImage {
    pub fn new(data: impl Into<crate::components::TensorData>) -> Self {
        Self {
            data: data.into(),
            meter: None,
            draw_order: None,
        }
    }

    pub fn with_meter(mut self, meter: impl Into<crate::components::DepthMeter>) -> Self {
        self.meter = Some(meter.into());
        self
    }

    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }
}
