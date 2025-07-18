// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AffixFuzzer2 {
    pub fuzz1101: Option<SerializedComponentBatch>,
    pub fuzz1102: Option<SerializedComponentBatch>,
    pub fuzz1103: Option<SerializedComponentBatch>,
    pub fuzz1104: Option<SerializedComponentBatch>,
    pub fuzz1105: Option<SerializedComponentBatch>,
    pub fuzz1106: Option<SerializedComponentBatch>,
    pub fuzz1107: Option<SerializedComponentBatch>,
    pub fuzz1108: Option<SerializedComponentBatch>,
    pub fuzz1109: Option<SerializedComponentBatch>,
    pub fuzz1110: Option<SerializedComponentBatch>,
    pub fuzz1111: Option<SerializedComponentBatch>,
    pub fuzz1112: Option<SerializedComponentBatch>,
    pub fuzz1113: Option<SerializedComponentBatch>,
    pub fuzz1114: Option<SerializedComponentBatch>,
    pub fuzz1115: Option<SerializedComponentBatch>,
    pub fuzz1116: Option<SerializedComponentBatch>,
    pub fuzz1117: Option<SerializedComponentBatch>,
    pub fuzz1118: Option<SerializedComponentBatch>,
    pub fuzz1122: Option<SerializedComponentBatch>,
}

impl AffixFuzzer2 {
    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1101`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer1`].
    #[inline]
    pub fn descriptor_fuzz1101() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1101".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer1".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1102`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer2`].
    #[inline]
    pub fn descriptor_fuzz1102() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1102".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer2".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1103`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer3`].
    #[inline]
    pub fn descriptor_fuzz1103() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1103".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer3".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1104`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer4`].
    #[inline]
    pub fn descriptor_fuzz1104() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1104".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer4".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1105`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer5`].
    #[inline]
    pub fn descriptor_fuzz1105() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1105".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer5".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1106`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer6`].
    #[inline]
    pub fn descriptor_fuzz1106() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1106".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer6".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1107`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer7`].
    #[inline]
    pub fn descriptor_fuzz1107() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1107".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer7".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1108`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer8`].
    #[inline]
    pub fn descriptor_fuzz1108() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1108".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer8".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1109`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer9`].
    #[inline]
    pub fn descriptor_fuzz1109() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1109".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer9".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1110`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer10`].
    #[inline]
    pub fn descriptor_fuzz1110() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1110".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer10".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1111`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer11`].
    #[inline]
    pub fn descriptor_fuzz1111() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1111".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer11".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1112`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer12`].
    #[inline]
    pub fn descriptor_fuzz1112() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1112".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer12".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1113`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer13`].
    #[inline]
    pub fn descriptor_fuzz1113() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1113".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer13".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1114`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer14`].
    #[inline]
    pub fn descriptor_fuzz1114() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1114".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer14".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1115`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer15`].
    #[inline]
    pub fn descriptor_fuzz1115() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1115".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer15".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1116`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer16`].
    #[inline]
    pub fn descriptor_fuzz1116() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1116".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer16".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1117`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer17`].
    #[inline]
    pub fn descriptor_fuzz1117() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1117".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer17".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1118`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer18`].
    #[inline]
    pub fn descriptor_fuzz1118() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1118".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer18".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz1122`].
    ///
    /// The corresponding component is [`crate::testing::components::AffixFuzzer22`].
    #[inline]
    pub fn descriptor_fuzz1122() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.testing.archetypes.AffixFuzzer2".into()),
            component: "AffixFuzzer2:fuzz1122".into(),
            component_type: Some("rerun.testing.components.AffixFuzzer22".into()),
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            AffixFuzzer2::descriptor_fuzz1101(),
            AffixFuzzer2::descriptor_fuzz1102(),
            AffixFuzzer2::descriptor_fuzz1103(),
            AffixFuzzer2::descriptor_fuzz1104(),
            AffixFuzzer2::descriptor_fuzz1105(),
            AffixFuzzer2::descriptor_fuzz1106(),
            AffixFuzzer2::descriptor_fuzz1107(),
            AffixFuzzer2::descriptor_fuzz1108(),
            AffixFuzzer2::descriptor_fuzz1109(),
            AffixFuzzer2::descriptor_fuzz1110(),
            AffixFuzzer2::descriptor_fuzz1111(),
            AffixFuzzer2::descriptor_fuzz1112(),
            AffixFuzzer2::descriptor_fuzz1113(),
            AffixFuzzer2::descriptor_fuzz1114(),
            AffixFuzzer2::descriptor_fuzz1115(),
            AffixFuzzer2::descriptor_fuzz1116(),
            AffixFuzzer2::descriptor_fuzz1117(),
            AffixFuzzer2::descriptor_fuzz1118(),
            AffixFuzzer2::descriptor_fuzz1122(),
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            AffixFuzzer2::descriptor_fuzz1101(),
            AffixFuzzer2::descriptor_fuzz1102(),
            AffixFuzzer2::descriptor_fuzz1103(),
            AffixFuzzer2::descriptor_fuzz1104(),
            AffixFuzzer2::descriptor_fuzz1105(),
            AffixFuzzer2::descriptor_fuzz1106(),
            AffixFuzzer2::descriptor_fuzz1107(),
            AffixFuzzer2::descriptor_fuzz1108(),
            AffixFuzzer2::descriptor_fuzz1109(),
            AffixFuzzer2::descriptor_fuzz1110(),
            AffixFuzzer2::descriptor_fuzz1111(),
            AffixFuzzer2::descriptor_fuzz1112(),
            AffixFuzzer2::descriptor_fuzz1113(),
            AffixFuzzer2::descriptor_fuzz1114(),
            AffixFuzzer2::descriptor_fuzz1115(),
            AffixFuzzer2::descriptor_fuzz1116(),
            AffixFuzzer2::descriptor_fuzz1117(),
            AffixFuzzer2::descriptor_fuzz1118(),
            AffixFuzzer2::descriptor_fuzz1122(),
        ]
    });

impl AffixFuzzer2 {
    /// The total number of components in the archetype: 19 required, 0 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 19usize;
}

impl ::re_types_core::Archetype for AffixFuzzer2 {
    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer2".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Affix fuzzer 2"
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let fuzz1101 = arrays_by_descr
            .get(&Self::descriptor_fuzz1101())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1101()));
        let fuzz1102 = arrays_by_descr
            .get(&Self::descriptor_fuzz1102())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1102()));
        let fuzz1103 = arrays_by_descr
            .get(&Self::descriptor_fuzz1103())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1103()));
        let fuzz1104 = arrays_by_descr
            .get(&Self::descriptor_fuzz1104())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1104()));
        let fuzz1105 = arrays_by_descr
            .get(&Self::descriptor_fuzz1105())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1105()));
        let fuzz1106 = arrays_by_descr
            .get(&Self::descriptor_fuzz1106())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1106()));
        let fuzz1107 = arrays_by_descr
            .get(&Self::descriptor_fuzz1107())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1107()));
        let fuzz1108 = arrays_by_descr
            .get(&Self::descriptor_fuzz1108())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1108()));
        let fuzz1109 = arrays_by_descr
            .get(&Self::descriptor_fuzz1109())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1109()));
        let fuzz1110 = arrays_by_descr
            .get(&Self::descriptor_fuzz1110())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1110()));
        let fuzz1111 = arrays_by_descr
            .get(&Self::descriptor_fuzz1111())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1111()));
        let fuzz1112 = arrays_by_descr
            .get(&Self::descriptor_fuzz1112())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1112()));
        let fuzz1113 = arrays_by_descr
            .get(&Self::descriptor_fuzz1113())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1113()));
        let fuzz1114 = arrays_by_descr
            .get(&Self::descriptor_fuzz1114())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1114()));
        let fuzz1115 = arrays_by_descr
            .get(&Self::descriptor_fuzz1115())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1115()));
        let fuzz1116 = arrays_by_descr
            .get(&Self::descriptor_fuzz1116())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1116()));
        let fuzz1117 = arrays_by_descr
            .get(&Self::descriptor_fuzz1117())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1117()));
        let fuzz1118 = arrays_by_descr
            .get(&Self::descriptor_fuzz1118())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1118()));
        let fuzz1122 = arrays_by_descr
            .get(&Self::descriptor_fuzz1122())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_fuzz1122()));
        Ok(Self {
            fuzz1101,
            fuzz1102,
            fuzz1103,
            fuzz1104,
            fuzz1105,
            fuzz1106,
            fuzz1107,
            fuzz1108,
            fuzz1109,
            fuzz1110,
            fuzz1111,
            fuzz1112,
            fuzz1113,
            fuzz1114,
            fuzz1115,
            fuzz1116,
            fuzz1117,
            fuzz1118,
            fuzz1122,
        })
    }
}

impl ::re_types_core::AsComponents for AffixFuzzer2 {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            self.fuzz1101.clone(),
            self.fuzz1102.clone(),
            self.fuzz1103.clone(),
            self.fuzz1104.clone(),
            self.fuzz1105.clone(),
            self.fuzz1106.clone(),
            self.fuzz1107.clone(),
            self.fuzz1108.clone(),
            self.fuzz1109.clone(),
            self.fuzz1110.clone(),
            self.fuzz1111.clone(),
            self.fuzz1112.clone(),
            self.fuzz1113.clone(),
            self.fuzz1114.clone(),
            self.fuzz1115.clone(),
            self.fuzz1116.clone(),
            self.fuzz1117.clone(),
            self.fuzz1118.clone(),
            self.fuzz1122.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for AffixFuzzer2 {}

impl AffixFuzzer2 {
    /// Create a new `AffixFuzzer2`.
    #[inline]
    pub fn new(
        fuzz1101: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer1>>,
        fuzz1102: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer2>>,
        fuzz1103: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer3>>,
        fuzz1104: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer4>>,
        fuzz1105: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer5>>,
        fuzz1106: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer6>>,
        fuzz1107: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer7>>,
        fuzz1108: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer8>>,
        fuzz1109: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer9>>,
        fuzz1110: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer10>>,
        fuzz1111: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer11>>,
        fuzz1112: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer12>>,
        fuzz1113: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer13>>,
        fuzz1114: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer14>>,
        fuzz1115: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer15>>,
        fuzz1116: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer16>>,
        fuzz1117: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer17>>,
        fuzz1118: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer18>>,
        fuzz1122: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer22>>,
    ) -> Self {
        Self {
            fuzz1101: try_serialize_field(Self::descriptor_fuzz1101(), fuzz1101),
            fuzz1102: try_serialize_field(Self::descriptor_fuzz1102(), fuzz1102),
            fuzz1103: try_serialize_field(Self::descriptor_fuzz1103(), fuzz1103),
            fuzz1104: try_serialize_field(Self::descriptor_fuzz1104(), fuzz1104),
            fuzz1105: try_serialize_field(Self::descriptor_fuzz1105(), fuzz1105),
            fuzz1106: try_serialize_field(Self::descriptor_fuzz1106(), fuzz1106),
            fuzz1107: try_serialize_field(Self::descriptor_fuzz1107(), fuzz1107),
            fuzz1108: try_serialize_field(Self::descriptor_fuzz1108(), fuzz1108),
            fuzz1109: try_serialize_field(Self::descriptor_fuzz1109(), fuzz1109),
            fuzz1110: try_serialize_field(Self::descriptor_fuzz1110(), fuzz1110),
            fuzz1111: try_serialize_field(Self::descriptor_fuzz1111(), fuzz1111),
            fuzz1112: try_serialize_field(Self::descriptor_fuzz1112(), fuzz1112),
            fuzz1113: try_serialize_field(Self::descriptor_fuzz1113(), fuzz1113),
            fuzz1114: try_serialize_field(Self::descriptor_fuzz1114(), fuzz1114),
            fuzz1115: try_serialize_field(Self::descriptor_fuzz1115(), fuzz1115),
            fuzz1116: try_serialize_field(Self::descriptor_fuzz1116(), fuzz1116),
            fuzz1117: try_serialize_field(Self::descriptor_fuzz1117(), fuzz1117),
            fuzz1118: try_serialize_field(Self::descriptor_fuzz1118(), fuzz1118),
            fuzz1122: try_serialize_field(Self::descriptor_fuzz1122(), fuzz1122),
        }
    }

    /// Update only some specific fields of a `AffixFuzzer2`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `AffixFuzzer2`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            fuzz1101: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer1::arrow_empty(),
                Self::descriptor_fuzz1101(),
            )),
            fuzz1102: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer2::arrow_empty(),
                Self::descriptor_fuzz1102(),
            )),
            fuzz1103: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer3::arrow_empty(),
                Self::descriptor_fuzz1103(),
            )),
            fuzz1104: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer4::arrow_empty(),
                Self::descriptor_fuzz1104(),
            )),
            fuzz1105: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer5::arrow_empty(),
                Self::descriptor_fuzz1105(),
            )),
            fuzz1106: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer6::arrow_empty(),
                Self::descriptor_fuzz1106(),
            )),
            fuzz1107: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer7::arrow_empty(),
                Self::descriptor_fuzz1107(),
            )),
            fuzz1108: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer8::arrow_empty(),
                Self::descriptor_fuzz1108(),
            )),
            fuzz1109: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer9::arrow_empty(),
                Self::descriptor_fuzz1109(),
            )),
            fuzz1110: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer10::arrow_empty(),
                Self::descriptor_fuzz1110(),
            )),
            fuzz1111: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer11::arrow_empty(),
                Self::descriptor_fuzz1111(),
            )),
            fuzz1112: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer12::arrow_empty(),
                Self::descriptor_fuzz1112(),
            )),
            fuzz1113: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer13::arrow_empty(),
                Self::descriptor_fuzz1113(),
            )),
            fuzz1114: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer14::arrow_empty(),
                Self::descriptor_fuzz1114(),
            )),
            fuzz1115: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer15::arrow_empty(),
                Self::descriptor_fuzz1115(),
            )),
            fuzz1116: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer16::arrow_empty(),
                Self::descriptor_fuzz1116(),
            )),
            fuzz1117: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer17::arrow_empty(),
                Self::descriptor_fuzz1117(),
            )),
            fuzz1118: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer18::arrow_empty(),
                Self::descriptor_fuzz1118(),
            )),
            fuzz1122: Some(SerializedComponentBatch::new(
                crate::testing::components::AffixFuzzer22::arrow_empty(),
                Self::descriptor_fuzz1122(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [
            self.fuzz1101
                .map(|fuzz1101| fuzz1101.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1102
                .map(|fuzz1102| fuzz1102.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1103
                .map(|fuzz1103| fuzz1103.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1104
                .map(|fuzz1104| fuzz1104.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1105
                .map(|fuzz1105| fuzz1105.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1106
                .map(|fuzz1106| fuzz1106.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1107
                .map(|fuzz1107| fuzz1107.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1108
                .map(|fuzz1108| fuzz1108.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1109
                .map(|fuzz1109| fuzz1109.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1110
                .map(|fuzz1110| fuzz1110.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1111
                .map(|fuzz1111| fuzz1111.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1112
                .map(|fuzz1112| fuzz1112.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1113
                .map(|fuzz1113| fuzz1113.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1114
                .map(|fuzz1114| fuzz1114.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1115
                .map(|fuzz1115| fuzz1115.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1116
                .map(|fuzz1116| fuzz1116.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1117
                .map(|fuzz1117| fuzz1117.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1118
                .map(|fuzz1118| fuzz1118.partitioned(_lengths.clone()))
                .transpose()?,
            self.fuzz1122
                .map(|fuzz1122| fuzz1122.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        Ok(columns.into_iter().flatten())
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_fuzz1101 = self.fuzz1101.as_ref().map(|b| b.array.len());
        let len_fuzz1102 = self.fuzz1102.as_ref().map(|b| b.array.len());
        let len_fuzz1103 = self.fuzz1103.as_ref().map(|b| b.array.len());
        let len_fuzz1104 = self.fuzz1104.as_ref().map(|b| b.array.len());
        let len_fuzz1105 = self.fuzz1105.as_ref().map(|b| b.array.len());
        let len_fuzz1106 = self.fuzz1106.as_ref().map(|b| b.array.len());
        let len_fuzz1107 = self.fuzz1107.as_ref().map(|b| b.array.len());
        let len_fuzz1108 = self.fuzz1108.as_ref().map(|b| b.array.len());
        let len_fuzz1109 = self.fuzz1109.as_ref().map(|b| b.array.len());
        let len_fuzz1110 = self.fuzz1110.as_ref().map(|b| b.array.len());
        let len_fuzz1111 = self.fuzz1111.as_ref().map(|b| b.array.len());
        let len_fuzz1112 = self.fuzz1112.as_ref().map(|b| b.array.len());
        let len_fuzz1113 = self.fuzz1113.as_ref().map(|b| b.array.len());
        let len_fuzz1114 = self.fuzz1114.as_ref().map(|b| b.array.len());
        let len_fuzz1115 = self.fuzz1115.as_ref().map(|b| b.array.len());
        let len_fuzz1116 = self.fuzz1116.as_ref().map(|b| b.array.len());
        let len_fuzz1117 = self.fuzz1117.as_ref().map(|b| b.array.len());
        let len_fuzz1118 = self.fuzz1118.as_ref().map(|b| b.array.len());
        let len_fuzz1122 = self.fuzz1122.as_ref().map(|b| b.array.len());
        let len = None
            .or(len_fuzz1101)
            .or(len_fuzz1102)
            .or(len_fuzz1103)
            .or(len_fuzz1104)
            .or(len_fuzz1105)
            .or(len_fuzz1106)
            .or(len_fuzz1107)
            .or(len_fuzz1108)
            .or(len_fuzz1109)
            .or(len_fuzz1110)
            .or(len_fuzz1111)
            .or(len_fuzz1112)
            .or(len_fuzz1113)
            .or(len_fuzz1114)
            .or(len_fuzz1115)
            .or(len_fuzz1116)
            .or(len_fuzz1117)
            .or(len_fuzz1118)
            .or(len_fuzz1122)
            .unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    #[inline]
    pub fn with_fuzz1101(
        mut self,
        fuzz1101: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer1>>,
    ) -> Self {
        self.fuzz1101 = try_serialize_field(Self::descriptor_fuzz1101(), fuzz1101);
        self
    }

    #[inline]
    pub fn with_fuzz1102(
        mut self,
        fuzz1102: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer2>>,
    ) -> Self {
        self.fuzz1102 = try_serialize_field(Self::descriptor_fuzz1102(), fuzz1102);
        self
    }

    #[inline]
    pub fn with_fuzz1103(
        mut self,
        fuzz1103: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer3>>,
    ) -> Self {
        self.fuzz1103 = try_serialize_field(Self::descriptor_fuzz1103(), fuzz1103);
        self
    }

    #[inline]
    pub fn with_fuzz1104(
        mut self,
        fuzz1104: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer4>>,
    ) -> Self {
        self.fuzz1104 = try_serialize_field(Self::descriptor_fuzz1104(), fuzz1104);
        self
    }

    #[inline]
    pub fn with_fuzz1105(
        mut self,
        fuzz1105: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer5>>,
    ) -> Self {
        self.fuzz1105 = try_serialize_field(Self::descriptor_fuzz1105(), fuzz1105);
        self
    }

    #[inline]
    pub fn with_fuzz1106(
        mut self,
        fuzz1106: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer6>>,
    ) -> Self {
        self.fuzz1106 = try_serialize_field(Self::descriptor_fuzz1106(), fuzz1106);
        self
    }

    #[inline]
    pub fn with_fuzz1107(
        mut self,
        fuzz1107: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer7>>,
    ) -> Self {
        self.fuzz1107 = try_serialize_field(Self::descriptor_fuzz1107(), fuzz1107);
        self
    }

    #[inline]
    pub fn with_fuzz1108(
        mut self,
        fuzz1108: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer8>>,
    ) -> Self {
        self.fuzz1108 = try_serialize_field(Self::descriptor_fuzz1108(), fuzz1108);
        self
    }

    #[inline]
    pub fn with_fuzz1109(
        mut self,
        fuzz1109: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer9>>,
    ) -> Self {
        self.fuzz1109 = try_serialize_field(Self::descriptor_fuzz1109(), fuzz1109);
        self
    }

    #[inline]
    pub fn with_fuzz1110(
        mut self,
        fuzz1110: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer10>>,
    ) -> Self {
        self.fuzz1110 = try_serialize_field(Self::descriptor_fuzz1110(), fuzz1110);
        self
    }

    #[inline]
    pub fn with_fuzz1111(
        mut self,
        fuzz1111: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer11>>,
    ) -> Self {
        self.fuzz1111 = try_serialize_field(Self::descriptor_fuzz1111(), fuzz1111);
        self
    }

    #[inline]
    pub fn with_fuzz1112(
        mut self,
        fuzz1112: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer12>>,
    ) -> Self {
        self.fuzz1112 = try_serialize_field(Self::descriptor_fuzz1112(), fuzz1112);
        self
    }

    #[inline]
    pub fn with_fuzz1113(
        mut self,
        fuzz1113: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer13>>,
    ) -> Self {
        self.fuzz1113 = try_serialize_field(Self::descriptor_fuzz1113(), fuzz1113);
        self
    }

    #[inline]
    pub fn with_fuzz1114(
        mut self,
        fuzz1114: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer14>>,
    ) -> Self {
        self.fuzz1114 = try_serialize_field(Self::descriptor_fuzz1114(), fuzz1114);
        self
    }

    #[inline]
    pub fn with_fuzz1115(
        mut self,
        fuzz1115: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer15>>,
    ) -> Self {
        self.fuzz1115 = try_serialize_field(Self::descriptor_fuzz1115(), fuzz1115);
        self
    }

    #[inline]
    pub fn with_fuzz1116(
        mut self,
        fuzz1116: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer16>>,
    ) -> Self {
        self.fuzz1116 = try_serialize_field(Self::descriptor_fuzz1116(), fuzz1116);
        self
    }

    #[inline]
    pub fn with_fuzz1117(
        mut self,
        fuzz1117: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer17>>,
    ) -> Self {
        self.fuzz1117 = try_serialize_field(Self::descriptor_fuzz1117(), fuzz1117);
        self
    }

    #[inline]
    pub fn with_fuzz1118(
        mut self,
        fuzz1118: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer18>>,
    ) -> Self {
        self.fuzz1118 = try_serialize_field(Self::descriptor_fuzz1118(), fuzz1118);
        self
    }

    #[inline]
    pub fn with_fuzz1122(
        mut self,
        fuzz1122: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer22>>,
    ) -> Self {
        self.fuzz1122 = try_serialize_field(Self::descriptor_fuzz1122(), fuzz1122);
        self
    }
}

impl ::re_byte_size::SizeBytes for AffixFuzzer2 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.fuzz1101.heap_size_bytes()
            + self.fuzz1102.heap_size_bytes()
            + self.fuzz1103.heap_size_bytes()
            + self.fuzz1104.heap_size_bytes()
            + self.fuzz1105.heap_size_bytes()
            + self.fuzz1106.heap_size_bytes()
            + self.fuzz1107.heap_size_bytes()
            + self.fuzz1108.heap_size_bytes()
            + self.fuzz1109.heap_size_bytes()
            + self.fuzz1110.heap_size_bytes()
            + self.fuzz1111.heap_size_bytes()
            + self.fuzz1112.heap_size_bytes()
            + self.fuzz1113.heap_size_bytes()
            + self.fuzz1114.heap_size_bytes()
            + self.fuzz1115.heap_size_bytes()
            + self.fuzz1116.heap_size_bytes()
            + self.fuzz1117.heap_size_bytes()
            + self.fuzz1118.heap_size_bytes()
            + self.fuzz1122.heap_size_bytes()
    }
}
