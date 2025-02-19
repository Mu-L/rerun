// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/datatypes/selected_columns.fbs".

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
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: List of selected columns in a dataframe.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectedColumns {
    /// The time columns to include
    pub time_columns: Vec<crate::datatypes::Utf8>,

    /// The component columns to include
    pub component_columns: Vec<crate::blueprint::datatypes::ComponentColumnSelector>,
}

::re_types_core::macros::impl_into_cow!(SelectedColumns);

impl ::re_types_core::Loggable for SelectedColumns {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![
            Field::new(
                "time_columns",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    <crate::datatypes::Utf8>::arrow_datatype(),
                    false,
                ))),
                false,
            ),
            Field::new(
                "component_columns",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    <crate::blueprint::datatypes::ComponentColumnSelector>::arrow_datatype(),
                    false,
                ))),
                false,
            ),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let fields = Fields::from(vec![
                Field::new(
                    "time_columns",
                    DataType::List(std::sync::Arc::new(Field::new(
                        "item",
                        <crate::datatypes::Utf8>::arrow_datatype(),
                        false,
                    ))),
                    false,
                ),
                Field::new(
                    "component_columns",
                    DataType::List(std::sync::Arc::new(Field::new(
                        "item",
                        <crate::blueprint::datatypes::ComponentColumnSelector>::arrow_datatype(),
                        false,
                    ))),
                    false,
                ),
            ]);
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(StructArray::new(
                fields,
                vec![
                    {
                        let (somes, time_columns): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.time_columns.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let time_columns_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                time_columns
                                    .iter()
                                    .map(|opt| opt.as_ref().map_or(0, |datum| datum.len())),
                            );
                            let time_columns_inner_data: Vec<_> =
                                time_columns.into_iter().flatten().flatten().collect();
                            let time_columns_inner_validity: Option<arrow::buffer::NullBuffer> =
                                None;
                            as_array_ref(ListArray::try_new(
                                std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::Utf8>::arrow_datatype(),
                                    false,
                                )),
                                offsets,
                                {
                                    let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                        time_columns_inner_data.iter().map(|datum| datum.0.len()),
                                    );
                                    #[allow(clippy::unwrap_used)]
                                    let capacity = offsets.last().copied().unwrap() as usize;
                                    let mut buffer_builder =
                                        arrow::array::builder::BufferBuilder::<u8>::new(capacity);
                                    for data in &time_columns_inner_data {
                                        buffer_builder.append_slice(data.0.as_bytes());
                                    }
                                    let inner_data: arrow::buffer::Buffer = buffer_builder.finish();

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    as_array_ref(unsafe {
                                        StringArray::new_unchecked(
                                            offsets,
                                            inner_data,
                                            time_columns_inner_validity,
                                        )
                                    })
                                },
                                time_columns_validity,
                            )?)
                        }
                    },
                    {
                        let (somes, component_columns): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum =
                                    datum.as_ref().map(|datum| datum.component_columns.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let component_columns_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                component_columns
                                    .iter()
                                    .map(|opt| opt.as_ref().map_or(0, |datum| datum.len())),
                            );
                            let component_columns_inner_data: Vec<_> =
                                component_columns.into_iter().flatten().flatten().collect();
                            let component_columns_inner_validity: Option<
                                arrow::buffer::NullBuffer,
                            > = None;
                            as_array_ref(ListArray::try_new(std::sync::Arc::new(Field::new("item",
                        < crate ::blueprint::datatypes::ComponentColumnSelector >
                        ::arrow_datatype(), false)), offsets, { _ =
                        component_columns_inner_validity; crate
                        ::blueprint::datatypes::ComponentColumnSelector::to_arrow_opt(component_columns_inner_data
                        .into_iter().map(Some)) ? }, component_columns_validity,) ?)
                        }
                    },
                ],
                validity,
            ))
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.blueprint.datatypes.SelectedColumns")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.columns());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name().as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let time_columns = {
                    if !arrays_by_name.contains_key("time_columns") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "time_columns",
                        ))
                        .with_context("rerun.blueprint.datatypes.SelectedColumns");
                    }
                    let arrow_data = &**arrays_by_name["time_columns"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow::array::ListArray>()
                            .ok_or_else(|| {
                                let expected = DataType::List(std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::Utf8>::arrow_datatype(),
                                    false,
                                )));
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.blueprint.datatypes.SelectedColumns#time_columns",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                {
                                    let arrow_data_inner = arrow_data_inner
                                        .as_any()
                                        .downcast_ref::<StringArray>()
                                        .ok_or_else(|| {
                                            let expected = DataType::Utf8;
                                            let actual = arrow_data_inner.data_type().clone();
                                            DeserializationError::datatype_mismatch(expected, actual)
                                        })
                                        .with_context(
                                            "rerun.blueprint.datatypes.SelectedColumns#time_columns",
                                        )?;
                                    let arrow_data_inner_buf = arrow_data_inner.values();
                                    let offsets = arrow_data_inner.offsets();
                                    ZipValidity::new_with_validity(
                                            offsets.windows(2),
                                            arrow_data_inner.nulls(),
                                        )
                                        .map(|elem| {
                                            elem
                                                .map(|window| {
                                                    let start = window[0] as usize;
                                                    let end = window[1] as usize;
                                                    let len = end - start;
                                                    if arrow_data_inner_buf.len() < end {
                                                        return Err(
                                                            DeserializationError::offset_slice_oob(
                                                                (start, end),
                                                                arrow_data_inner_buf.len(),
                                                            ),
                                                        );
                                                    }

                                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                                    let data = arrow_data_inner_buf
                                                        .slice_with_length(start, len);
                                                    Ok(data)
                                                })
                                                .transpose()
                                        })
                                        .map(|res_or_opt| {
                                            res_or_opt
                                                .map(|res_or_opt| {
                                                    res_or_opt
                                                        .map(|v| crate::datatypes::Utf8(
                                                            ::re_types_core::ArrowString::from(v),
                                                        ))
                                                })
                                        })
                                        .collect::<DeserializationResult<Vec<Option<_>>>>()
                                        .with_context(
                                            "rerun.blueprint.datatypes.SelectedColumns#time_columns",
                                        )?
                                        .into_iter()
                                }
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            ZipValidity::new_with_validity(
                                    offsets.windows(2),
                                    arrow_data.nulls(),
                                )
                                .map(|elem| {
                                    elem
                                        .map(|window| {
                                            let start = window[0] as usize;
                                            let end = window[1] as usize;
                                            if arrow_data_inner.len() < end {
                                                return Err(
                                                    DeserializationError::offset_slice_oob(
                                                        (start, end),
                                                        arrow_data_inner.len(),
                                                    ),
                                                );
                                            }

                                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                            let data = unsafe {
                                                arrow_data_inner.get_unchecked(start..end)
                                            };
                                            let data = data
                                                .iter()
                                                .cloned()
                                                .map(Option::unwrap_or_default)
                                                .collect();
                                            Ok(data)
                                        })
                                        .transpose()
                                })
                                .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                            .into_iter()
                    }
                };
                let component_columns = {
                    if !arrays_by_name.contains_key("component_columns") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "component_columns",
                        ))
                        .with_context("rerun.blueprint.datatypes.SelectedColumns");
                    }
                    let arrow_data = &**arrays_by_name["component_columns"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow::array::ListArray>()
                            .ok_or_else(|| {
                                let expected = DataType::List(
                                    std::sync::Arc::new(
                                        Field::new(
                                            "item",
                                            <crate::blueprint::datatypes::ComponentColumnSelector>::arrow_datatype(),
                                            false,
                                        ),
                                    ),
                                );
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.blueprint.datatypes.SelectedColumns#component_columns",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::blueprint::datatypes::ComponentColumnSelector::from_arrow_opt(
                                        arrow_data_inner,
                                    )
                                    .with_context(
                                        "rerun.blueprint.datatypes.SelectedColumns#component_columns",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            ZipValidity::new_with_validity(
                                    offsets.windows(2),
                                    arrow_data.nulls(),
                                )
                                .map(|elem| {
                                    elem
                                        .map(|window| {
                                            let start = window[0] as usize;
                                            let end = window[1] as usize;
                                            if arrow_data_inner.len() < end {
                                                return Err(
                                                    DeserializationError::offset_slice_oob(
                                                        (start, end),
                                                        arrow_data_inner.len(),
                                                    ),
                                                );
                                            }

                                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                            let data = unsafe {
                                                arrow_data_inner.get_unchecked(start..end)
                                            };
                                            let data = data
                                                .iter()
                                                .cloned()
                                                .map(Option::unwrap_or_default)
                                                .collect();
                                            Ok(data)
                                        })
                                        .transpose()
                                })
                                .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                            .into_iter()
                    }
                };
                ZipValidity::new_with_validity(
                    ::itertools::izip!(time_columns, component_columns),
                    arrow_data.nulls(),
                )
                .map(|opt| {
                    opt.map(|(time_columns, component_columns)| {
                        Ok(Self {
                            time_columns: time_columns
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.blueprint.datatypes.SelectedColumns#time_columns",
                                )?,
                            component_columns: component_columns
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.blueprint.datatypes.SelectedColumns#component_columns",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.datatypes.SelectedColumns")?
            }
        })
    }
}

impl ::re_byte_size::SizeBytes for SelectedColumns {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.time_columns.heap_size_bytes() + self.component_columns.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::datatypes::Utf8>>::is_pod()
            && <Vec<crate::blueprint::datatypes::ComponentColumnSelector>>::is_pod()
    }
}
