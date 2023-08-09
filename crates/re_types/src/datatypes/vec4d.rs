// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A vector in 4D space.
#[derive(Clone, Debug, Default, Copy, PartialEq, PartialOrd)]
pub struct Vec4D(pub [f32; 4usize]);

impl<'a> From<Vec4D> for ::std::borrow::Cow<'a, Vec4D> {
    #[inline]
    fn from(value: Vec4D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Vec4D> for ::std::borrow::Cow<'a, Vec4D> {
    #[inline]
    fn from(value: &'a Vec4D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Vec4D {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<dyn Iterator<Item = Self::Item<'a>> + 'a>;
    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Vec4D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::FixedSizeList(
            Box::new(Field {
                name: "item".to_owned(),
                data_type: DataType::Float32,
                is_nullable: false,
                metadata: [].into(),
            }),
            4usize,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let data0_inner_data: Vec<_> = data0
                    .iter()
                    .flatten()
                    .flatten()
                    .cloned()
                    .map(Some)
                    .collect();
                let data0_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                FixedSizeListArray::new(
                    {
                        _ = extension_wrapper;
                        DataType::Extension(
                            "rerun.datatypes.Vec4D".to_owned(),
                            Box::new(DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                4usize,
                            )),
                            None,
                        )
                        .to_logical_type()
                        .clone()
                    },
                    PrimitiveArray::new(
                        {
                            _ = extension_wrapper;
                            DataType::Extension(
                                "rerun.datatypes.Vec4D".to_owned(),
                                Box::new(DataType::Float32),
                                None,
                            )
                            .to_logical_type()
                            .clone()
                        },
                        data0_inner_data
                            .into_iter()
                            .map(|v| v.unwrap_or_default())
                            .collect(),
                        data0_inner_bitmap,
                    )
                    .boxed(),
                    data0_bitmap,
                )
                .boxed()
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                .unwrap();
            if data.is_empty() {
                Vec::new()
            } else {
                let bitmap = data.validity().cloned();
                let offsets = (0..)
                    .step_by(4usize)
                    .zip((4usize..).step_by(4usize).take(data.len()));
                let data = &**data.values();
                let data = data
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.copied())
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()?;
                offsets
                    .enumerate()
                    .map(move |(i, (start, end))| {
                        bitmap
                            .as_ref()
                            .map_or(true, |bitmap| bitmap.get_bit(i))
                            .then(|| {
                                let data = data.get(start as usize..end as usize).ok_or(
                                    crate::DeserializationError::OffsetsMismatch {
                                        bounds: (start as usize, end as usize),
                                        len: data.len(),
                                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                                    },
                                )?;
                                let mut arr = [Default::default(); 4usize];

                                arr.copy_from_slice(data);
                                Ok(arr)
                            })
                            .transpose()
                    })
                    .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| {
            v.ok_or_else(|| crate::DeserializationError::MissingData {
                backtrace: ::backtrace::Backtrace::new_unresolved(),
            })
        })
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
        .map_err(|err| crate::DeserializationError::Context {
            location: "rerun.datatypes.Vec4D#xyzw".into(),
            source: Box::new(err),
        })?)
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::try_from_arrow_opt(data)?.into_iter()))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for Vec4D {}
