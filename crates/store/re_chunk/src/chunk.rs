use std::sync::atomic::{AtomicU64, Ordering};

use ahash::HashMap;
use arrow::{
    array::{
        Array as ArrowArray, ArrayRef as ArrowArrayRef, FixedSizeBinaryArray,
        ListArray as ArrowListArray,
    },
    buffer::{NullBuffer as ArrowNullBuffer, ScalarBuffer as ArrowScalarBuffer},
};
use itertools::{izip, Either, Itertools};
use nohash_hasher::IntMap;

use re_arrow_util::ArrowArrayDowncastRef as _;
use re_byte_size::SizeBytes as _;
use re_log_types::{EntityPath, ResolvedTimeRange, Time, TimeInt, TimePoint, Timeline};
use re_types_core::{
    ComponentDescriptor, ComponentName, DeserializationError, Loggable, SerializationError,
};

use crate::{ChunkId, RowId};

// ---

/// Errors that can occur when creating/manipulating a [`Chunk`]s, directly or indirectly through
/// the use of a [`crate::ChunkBatcher`].
#[derive(thiserror::Error, Debug)]
pub enum ChunkError {
    #[error("Detected malformed Chunk: {reason}")]
    Malformed { reason: String },

    #[error(transparent)]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("{kind} index out of bounds: {index} (len={len})")]
    IndexOutOfBounds {
        kind: String,
        len: usize,
        index: usize,
    },

    #[error("Serialization: {0}")]
    Serialization(#[from] SerializationError),

    #[error("Deserialization: {0}")]
    Deserialization(#[from] DeserializationError),

    #[error(transparent)]
    UnsupportedTimeType(#[from] re_sorbet::UnsupportedTimeType),

    #[error(transparent)]
    WrongDatatypeError(#[from] re_sorbet::WrongDatatypeError),

    #[error(transparent)]
    MismatchedChunkSchemaError(#[from] re_sorbet::MismatchedChunkSchemaError),

    #[error(transparent)]
    InvalidSorbetSchema(#[from] re_sorbet::SorbetError),
}

pub type ChunkResult<T> = Result<T, ChunkError>;

// ---

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChunkComponents(
    // TODO(#6576): support non-list based columns?
    //
    // NOTE: The extra `ComponentName` layer is needed because it is very common to want to look
    // for anything matching a `ComponentName`, without any further tags specified.
    pub IntMap<ComponentName, IntMap<ComponentDescriptor, ArrowListArray>>,
);

impl ChunkComponents {
    /// Like `Self::insert`, but automatically infers the [`ComponentName`] layer.
    #[inline]
    pub fn insert_descriptor(
        &mut self,
        component_desc: ComponentDescriptor,
        list_array: ArrowListArray,
    ) -> Option<ArrowListArray> {
        self.0
            .entry(component_desc.component_name)
            .or_default()
            .insert(component_desc, list_array)
    }

    /// Returns all list arrays for the given component name.
    ///
    /// I.e semantically equivalent to `get("MyComponent:*.*")`
    #[inline]
    pub fn get_by_component_name<'a>(
        &'a self,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = &'a ArrowListArray> {
        self.get(component_name).map_or_else(
            || itertools::Either::Left(std::iter::empty()),
            |per_desc| itertools::Either::Right(per_desc.values()),
        )
    }

    #[inline]
    pub fn get_by_descriptor(
        &self,
        component_desc: &ComponentDescriptor,
    ) -> Option<&ArrowListArray> {
        self.get(&component_desc.component_name)
            .and_then(|per_desc| per_desc.get(component_desc))
    }

    #[inline]
    pub fn get_by_descriptor_mut(
        &mut self,
        component_desc: &ComponentDescriptor,
    ) -> Option<&mut ArrowListArray> {
        self.get_mut(&component_desc.component_name)
            .and_then(|per_desc| per_desc.get_mut(component_desc))
    }

    #[inline]
    pub fn iter_flattened(&self) -> impl Iterator<Item = (&ComponentDescriptor, &ArrowListArray)> {
        self.0.values().flatten()
    }

    #[inline]
    pub fn into_iter_flattened(
        self,
    ) -> impl Iterator<Item = (ComponentDescriptor, ArrowListArray)> {
        self.0.into_values().flatten()
    }
}

impl std::ops::Deref for ChunkComponents {
    type Target = IntMap<ComponentName, IntMap<ComponentDescriptor, ArrowListArray>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ChunkComponents {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(ComponentDescriptor, ArrowListArray)> for ChunkComponents {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (ComponentDescriptor, ArrowListArray)>>(iter: T) -> Self {
        let mut this = Self::default();
        {
            for (component_desc, list_array) in iter {
                this.insert_descriptor(component_desc, list_array);
            }
        }
        this
    }
}

// TODO(cmc): Kinda disgusting but it makes our lives easier during the interim, as long as we're
// in this weird halfway in-between state where we still have a bunch of things indexed by name only.
impl FromIterator<(ComponentName, ArrowListArray)> for ChunkComponents {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (ComponentName, ArrowListArray)>>(iter: T) -> Self {
        iter.into_iter()
            .map(|(component_name, list_array)| {
                let component_desc = ComponentDescriptor::new(component_name);
                (component_desc, list_array)
            })
            .collect()
    }
}

/// Dense arrow-based storage of N rows of multi-component multi-temporal data for a specific entity.
///
/// This is our core datastructure for logging, storing, querying and transporting data around.
///
/// The chunk as a whole is always ascendingly sorted by [`RowId`] before it gets manipulated in any way.
/// Its time columns might or might not be ascendingly sorted, depending on how the data was logged.
///
/// This is the in-memory representation of a chunk, optimized for efficient manipulation of the
/// data within. For transport, see [`re_sorbet::ChunkBatch`] instead.
#[derive(Debug)]
pub struct Chunk {
    pub(crate) id: ChunkId,

    pub(crate) entity_path: EntityPath,

    /// The heap size of this chunk in bytes.
    ///
    /// Must be cached as it is very costly to compute, and needs to be computed repeatedly on the
    /// hot path (e.g. during garbage collection).
    pub(crate) heap_size_bytes: AtomicU64,

    /// Is the chunk as a whole sorted by [`RowId`]?
    pub(crate) is_sorted: bool,

    /// The respective [`RowId`]s for each row of data.
    pub(crate) row_ids: FixedSizeBinaryArray,

    /// The time columns.
    ///
    /// Each column must be the same length as `row_ids`.
    ///
    /// Empty if this is a static chunk.
    pub(crate) timelines: IntMap<Timeline, TimeColumn>,

    /// A sparse `ListArray` for each component.
    ///
    /// Each `ListArray` must be the same length as `row_ids`.
    ///
    /// Sparse so that we can e.g. log a `Position` at one timestamp but not a `Color`.
    pub(crate) components: ChunkComponents,
}

impl PartialEq for Chunk {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let Self {
            id,
            entity_path,
            heap_size_bytes: _,
            is_sorted,
            row_ids,
            timelines,
            components,
        } = self;

        *id == other.id
            && *entity_path == other.entity_path
            && *is_sorted == other.is_sorted
            && *row_ids == other.row_ids
            && *timelines == other.timelines
            && *components == other.components
    }
}

impl Chunk {
    /// Returns any list-array that matches the given [`ComponentName`].
    ///
    /// This is undefined behavior if there are more than one component with that name.
    //
    // TODO(cmc): Kinda disgusting but it makes our lives easier during the interim, as long as we're
    // in this weird halfway in-between state where we still have a bunch of things indexed by name only.
    #[inline]
    pub fn get_first_component(&self, component_name: &ComponentName) -> Option<&ArrowListArray> {
        self.components
            .get(component_name)
            .and_then(|per_desc| per_desc.values().next())
    }
}

impl Chunk {
    /// Returns a version of us with a new [`ChunkId`].
    ///
    /// Reminder:
    /// * The returned [`Chunk`] will re-use the exact same [`RowId`]s as `self`.
    /// * Duplicated [`RowId`]s in the `ChunkStore` is undefined behavior.
    #[must_use]
    #[inline]
    pub fn with_id(mut self, id: ChunkId) -> Self {
        self.id = id;
        self
    }

    /// Returns `true` is two [`Chunk`]s are similar, although not byte-for-byte equal.
    ///
    /// In particular, this ignores chunks and row IDs, as well as temporal timestamps.
    ///
    /// Useful for tests.
    pub fn are_similar(lhs: &Self, rhs: &Self) -> bool {
        let Self {
            id: _,
            entity_path,
            heap_size_bytes: _,
            is_sorted: _,
            row_ids: _,
            timelines,
            components,
        } = lhs;

        *entity_path == rhs.entity_path
            && timelines.keys().collect_vec() == rhs.timelines.keys().collect_vec()
            && {
                let timelines: IntMap<_, _> = timelines
                    .iter()
                    .map(|(timeline, time_chunk)| (*timeline, time_chunk))
                    .filter(|(timeline, _time_chunk)| {
                        timeline.typ() != re_log_types::TimeType::Time
                    })
                    .collect();
                let rhs_timelines: IntMap<_, _> = rhs
                    .timelines
                    .iter()
                    .map(|(timeline, time_chunk)| (*timeline, time_chunk))
                    .filter(|(timeline, _time_chunk)| {
                        timeline.typ() != re_log_types::TimeType::Time
                    })
                    .collect();
                timelines == rhs_timelines
            }
            && *components == rhs.components
    }

    // Only used for tests atm
    pub fn are_equal(&self, other: &Self) -> bool {
        let Self {
            id,
            entity_path,
            heap_size_bytes: _,
            is_sorted,
            row_ids,
            timelines,
            components,
        } = self;

        let my_components: IntMap<_, _> = components
            .values()
            .flat_map(|per_desc| {
                per_desc
                    .iter()
                    .map(|(descr, list_array)| (descr.clone(), list_array))
            })
            .collect();

        let other_components: IntMap<_, _> = other
            .components
            .values()
            .flat_map(|per_desc| {
                per_desc
                    .iter()
                    .map(|(descr, list_array)| (descr.clone(), list_array))
            })
            .collect();

        *id == other.id
            && *entity_path == other.entity_path
            && *is_sorted == other.is_sorted
            && row_ids == &other.row_ids
            && *timelines == other.timelines
            && my_components == other_components
    }
}

impl Clone for Chunk {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            entity_path: self.entity_path.clone(),
            heap_size_bytes: AtomicU64::new(self.heap_size_bytes.load(Ordering::Relaxed)),
            is_sorted: self.is_sorted,
            row_ids: self.row_ids.clone(),
            timelines: self.timelines.clone(),
            components: self.components.clone(),
        }
    }
}

impl Chunk {
    /// Clones the chunk and assign new IDs to the resulting chunk and its rows.
    ///
    /// `first_row_id` will become the [`RowId`] of the first row in the duplicated chunk.
    /// Each row after that will be monotonically increasing.
    #[inline]
    pub fn clone_as(&self, id: ChunkId, first_row_id: RowId) -> Self {
        let row_ids = std::iter::from_fn({
            let mut row_id = first_row_id;
            move || {
                let yielded = row_id;
                row_id = row_id.next();
                Some(yielded)
            }
        })
        .take(self.row_ids.len())
        .collect_vec();

        Self {
            id,
            row_ids: RowId::arrow_from_slice(&row_ids),
            ..self.clone()
        }
    }

    /// Clones the chunk into a new chunk without any time data.
    #[inline]
    pub fn into_static(mut self) -> Self {
        self.timelines.clear();
        self
    }

    /// Clones the chunk into a new chunk where all [`RowId`]s are [`RowId::ZERO`].
    pub fn zeroed(self) -> Self {
        let row_ids = std::iter::repeat(RowId::ZERO)
            .take(self.row_ids.len())
            .collect_vec();

        let row_ids = RowId::arrow_from_slice(&row_ids);

        Self { row_ids, ..self }
    }

    /// Computes the time range covered by each individual component column on each timeline.
    ///
    /// This is different from the time range covered by the [`Chunk`] as a whole because component
    /// columns are potentially sparse.
    ///
    /// This is crucial for indexing and queries to work properly.
    //
    // TODO(cmc): This needs to be stored in chunk metadata and transported across IPC.
    #[inline]
    pub fn time_range_per_component(
        &self,
    ) -> IntMap<Timeline, IntMap<ComponentName, IntMap<ComponentDescriptor, ResolvedTimeRange>>>
    {
        re_tracing::profile_function!();

        self.timelines
            .iter()
            .map(|(&timeline, time_column)| {
                (
                    timeline,
                    time_column.time_range_per_component(&self.components),
                )
            })
            .collect()
    }

    /// The cumulative number of events in this chunk.
    ///
    /// I.e. how many _component batches_ ("cells") were logged in total?
    //
    // TODO(cmc): This needs to be stored in chunk metadata and transported across IPC.
    #[inline]
    pub fn num_events_cumulative(&self) -> u64 {
        // Reminder: component columns are sparse, we must take a look at the validity bitmaps.
        self.components
            .values()
            .flat_map(|per_desc| per_desc.values())
            .map(|list_array| {
                list_array.nulls().map_or_else(
                    || list_array.len() as u64,
                    |validity| validity.len() as u64 - validity.null_count() as u64,
                )
            })
            .sum()
    }

    /// The cumulative number of events in this chunk for each _unique_ timestamp.
    ///
    /// I.e. how many _component batches_ ("cells") were logged in total at each timestamp?
    ///
    /// Keep in mind that a timestamp can appear multiple times in a [`Chunk`].
    /// This method will do a sum accumulation to account for these cases (i.e. every timestamp in
    /// the returned vector is guaranteed to be unique).
    pub fn num_events_cumulative_per_unique_time(
        &self,
        timeline: &Timeline,
    ) -> Vec<(TimeInt, u64)> {
        re_tracing::profile_function!();

        if self.is_static() {
            return vec![(TimeInt::STATIC, self.num_events_cumulative())];
        }

        let Some(time_column) = self.timelines().get(timeline) else {
            return Vec::new();
        };

        let time_range = time_column.time_range();
        if time_range.min() == time_range.max() {
            return vec![(time_range.min(), self.num_events_cumulative())];
        }

        let counts = if time_column.is_sorted() {
            self.num_events_cumulative_per_unique_time_sorted(time_column)
        } else {
            self.num_events_cumulative_per_unique_time_unsorted(time_column)
        };

        debug_assert!(counts
            .iter()
            .tuple_windows::<(_, _)>()
            .all(|((time1, _), (time2, _))| time1 < time2));

        counts
    }

    fn num_events_cumulative_per_unique_time_sorted(
        &self,
        time_column: &TimeColumn,
    ) -> Vec<(TimeInt, u64)> {
        re_tracing::profile_function!();

        debug_assert!(time_column.is_sorted());

        // NOTE: This is used on some very hot paths (time panel rendering).
        // Performance trumps readability. Optimized empirically.

        // Raw, potentially duplicated counts (because timestamps aren't necessarily unique).
        let mut counts_raw = vec![0u64; self.num_rows()];
        {
            self.components
                .values()
                .flat_map(|per_desc| per_desc.values())
                .for_each(|list_array| {
                    if let Some(validity) = list_array.nulls() {
                        validity
                            .iter()
                            .enumerate()
                            .for_each(|(i, is_valid)| counts_raw[i] += is_valid as u64);
                    } else {
                        counts_raw.iter_mut().for_each(|count| *count += 1);
                    }
                });
        }

        let mut counts = Vec::with_capacity(counts_raw.len());

        let Some(mut cur_time) = time_column.times().next() else {
            return Vec::new();
        };
        let mut cur_count = 0;
        izip!(time_column.times(), counts_raw).for_each(|(time, count)| {
            if time == cur_time {
                cur_count += count;
            } else {
                counts.push((cur_time, cur_count));
                cur_count = count;
                cur_time = time;
            }
        });

        if counts.last().map(|(time, _)| *time) != Some(cur_time) {
            counts.push((cur_time, cur_count));
        }

        counts
    }

    fn num_events_cumulative_per_unique_time_unsorted(
        &self,
        time_column: &TimeColumn,
    ) -> Vec<(TimeInt, u64)> {
        re_tracing::profile_function!();

        debug_assert!(!time_column.is_sorted());

        // NOTE: This is used on some very hot paths (time panel rendering).

        let result_unordered = self
            .components
            .values()
            .flat_map(|per_desc| per_desc.values())
            .fold(HashMap::default(), |acc, list_array| {
                if let Some(validity) = list_array.nulls() {
                    time_column.times().zip(validity.iter()).fold(
                        acc,
                        |mut acc, (time, is_valid)| {
                            *acc.entry(time).or_default() += is_valid as u64;
                            acc
                        },
                    )
                } else {
                    time_column.times().fold(acc, |mut acc, time| {
                        *acc.entry(time).or_default() += 1;
                        acc
                    })
                }
            });

        let mut result = result_unordered.into_iter().collect_vec();
        result.sort_by_key(|val| val.0);
        result
    }

    /// The number of events in this chunk for the specified component.
    ///
    /// I.e. how many _component batches_ ("cells") were logged in total for this component?
    //
    // TODO(cmc): This needs to be stored in chunk metadata and transported across IPC.
    #[inline]
    pub fn num_events_for_component(&self, component_name: ComponentName) -> Option<u64> {
        // Reminder: component columns are sparse, we must check validity bitmap.
        self.get_first_component(&component_name).map(|list_array| {
            list_array.nulls().map_or_else(
                || list_array.len() as u64,
                |validity| validity.len() as u64 - validity.null_count() as u64,
            )
        })
    }

    /// Computes the `RowId` range covered by each individual component column on each timeline.
    ///
    /// This is different from the `RowId` range covered by the [`Chunk`] as a whole because component
    /// columns are potentially sparse.
    ///
    /// This is crucial for indexing and queries to work properly.
    //
    // TODO(cmc): This needs to be stored in chunk metadata and transported across IPC.
    pub fn row_id_range_per_component(
        &self,
    ) -> IntMap<ComponentName, IntMap<ComponentDescriptor, (RowId, RowId)>> {
        re_tracing::profile_function!();

        let row_ids = self.row_ids().collect_vec();

        if self.is_sorted() {
            self.components
                .iter()
                .map(|(component_name, per_desc)| {
                    (
                        *component_name,
                        per_desc
                            .iter()
                            .filter_map(|(component_desc, list_array)| {
                                let mut row_id_min = None;
                                let mut row_id_max = None;

                                for (i, &row_id) in row_ids.iter().enumerate() {
                                    if list_array.is_valid(i) {
                                        row_id_min = Some(row_id);
                                    }
                                }
                                for (i, &row_id) in row_ids.iter().enumerate().rev() {
                                    if list_array.is_valid(i) {
                                        row_id_max = Some(row_id);
                                    }
                                }

                                Some((component_desc.clone(), (row_id_min?, row_id_max?)))
                            })
                            .collect(),
                    )
                })
                .collect()
        } else {
            self.components
                .iter()
                .map(|(component_name, per_desc)| {
                    (
                        *component_name,
                        per_desc
                            .iter()
                            .filter_map(|(component_desc, list_array)| {
                                let mut row_id_min = Some(RowId::MAX);
                                let mut row_id_max = Some(RowId::ZERO);

                                for (i, &row_id) in row_ids.iter().enumerate() {
                                    if list_array.is_valid(i) && Some(row_id) > row_id_min {
                                        row_id_min = Some(row_id);
                                    }
                                }
                                for (i, &row_id) in row_ids.iter().enumerate().rev() {
                                    if list_array.is_valid(i) && Some(row_id) < row_id_max {
                                        row_id_max = Some(row_id);
                                    }
                                }

                                Some((component_desc.clone(), (row_id_min?, row_id_max?)))
                            })
                            .collect(),
                    )
                })
                .collect()
        }
    }
}

// ---

#[derive(Debug, Clone, PartialEq)]
pub struct TimeColumn {
    pub(crate) timeline: Timeline,

    /// Every single timestamp for this timeline.
    ///
    /// * This might or might not be sorted, depending on how the data was logged.
    /// * This is guaranteed to always be dense, because chunks are split anytime a timeline is
    ///   added or removed.
    /// * This cannot ever contain `TimeInt::STATIC`, since static data doesn't even have timelines.
    ///
    /// When this buffer is converted to an arrow array, it's datatype will depend
    /// on the timeline type, so it will either become a
    /// [`arrow::array::Int64Array`] or a [`arrow::array::TimestampNanosecondArray`].
    pub(crate) times: ArrowScalarBuffer<i64>,

    /// Is [`Self::times`] sorted?
    ///
    /// This is completely independent of [`Chunk::is_sorted`]: a timeline doesn't necessarily
    /// follow the global [`RowId`]-based order, although it does in most cases (happy path).
    pub(crate) is_sorted: bool,

    /// The time range covered by [`Self::times`].
    ///
    /// Not necessarily contiguous! Just the min and max value found in [`Self::times`].
    pub(crate) time_range: ResolvedTimeRange,
}

/// Errors when deserializing/parsing/reading a column of time data.
#[derive(Debug, thiserror::Error)]
pub enum TimeColumnError {
    #[error("Time columns had nulls, but should be dense")]
    ContainsNulls,

    #[error("Unsupported data type : {0}")]
    UnsupportedDataType(arrow::datatypes::DataType),
}

impl Chunk {
    /// Creates a new [`Chunk`].
    ///
    /// This will fail if the passed in data is malformed in any way -- see [`Self::sanity_check`]
    /// for details.
    ///
    /// Iff you know for sure whether the data is already appropriately sorted or not, specify `is_sorted`.
    /// When left unspecified (`None`), it will be computed in O(n) time.
    ///
    /// For a row-oriented constructor, see [`Self::builder`].
    pub fn new(
        id: ChunkId,
        entity_path: EntityPath,
        is_sorted: Option<bool>,
        row_ids: FixedSizeBinaryArray,
        timelines: IntMap<Timeline, TimeColumn>,
        components: ChunkComponents,
    ) -> ChunkResult<Self> {
        let mut chunk = Self {
            id,
            entity_path,
            heap_size_bytes: AtomicU64::new(0),
            is_sorted: false,
            row_ids,
            timelines,
            components,
        };

        chunk.is_sorted = is_sorted.unwrap_or_else(|| chunk.is_sorted_uncached());

        chunk.sanity_check()?;

        Ok(chunk)
    }

    /// Creates a new [`Chunk`].
    ///
    /// This will fail if the passed in data is malformed in any way -- see [`Self::sanity_check`]
    /// for details.
    ///
    /// Iff you know for sure whether the data is already appropriately sorted or not, specify `is_sorted`.
    /// When left unspecified (`None`), it will be computed in O(n) time.
    ///
    /// For a row-oriented constructor, see [`Self::builder`].
    pub fn from_native_row_ids(
        id: ChunkId,
        entity_path: EntityPath,
        is_sorted: Option<bool>,
        row_ids: &[RowId],
        timelines: IntMap<Timeline, TimeColumn>,
        components: ChunkComponents,
    ) -> ChunkResult<Self> {
        re_tracing::profile_function!();
        let row_ids = RowId::arrow_from_slice(row_ids);
        Self::new(id, entity_path, is_sorted, row_ids, timelines, components)
    }

    /// Creates a new [`Chunk`].
    ///
    /// This will fail if the passed in data is malformed in any way -- see [`Self::sanity_check`]
    /// for details.
    ///
    /// The data is assumed to be sorted in `RowId`-order. Sequential `RowId`s will be generated for each
    /// row in the chunk.
    pub fn from_auto_row_ids(
        id: ChunkId,
        entity_path: EntityPath,
        timelines: IntMap<Timeline, TimeColumn>,
        components: ChunkComponents,
    ) -> ChunkResult<Self> {
        let count = components
            .iter_flattened()
            .next()
            .map_or(0, |(_, list_array)| list_array.len());

        let row_ids = std::iter::from_fn({
            let tuid: re_tuid::Tuid = *id;
            let mut row_id = RowId::from_tuid(tuid.next());
            move || {
                let yielded = row_id;
                row_id = row_id.next();
                Some(yielded)
            }
        })
        .take(count)
        .collect_vec();

        Self::from_native_row_ids(id, entity_path, Some(true), &row_ids, timelines, components)
    }

    /// Simple helper for [`Self::new`] for static data.
    ///
    /// For a row-oriented constructor, see [`Self::builder`].
    #[inline]
    pub fn new_static(
        id: ChunkId,
        entity_path: EntityPath,
        is_sorted: Option<bool>,
        row_ids: FixedSizeBinaryArray,
        components: ChunkComponents,
    ) -> ChunkResult<Self> {
        Self::new(
            id,
            entity_path,
            is_sorted,
            row_ids,
            Default::default(),
            components,
        )
    }

    #[inline]
    pub fn empty(id: ChunkId, entity_path: EntityPath) -> Self {
        Self {
            id,
            entity_path,
            heap_size_bytes: Default::default(),
            is_sorted: true,
            row_ids: RowId::arrow_from_slice(&[]),
            timelines: Default::default(),
            components: Default::default(),
        }
    }

    /// Unconditionally inserts an [`ArrowListArray`] as a component column.
    ///
    /// Removes and replaces the column if it already exists.
    ///
    /// This will fail if the end result is malformed in any way -- see [`Self::sanity_check`].
    #[inline]
    pub fn add_component(
        &mut self,
        component_desc: ComponentDescriptor,
        list_array: ArrowListArray,
    ) -> ChunkResult<()> {
        self.components
            .insert_descriptor(component_desc, list_array);
        self.sanity_check()
    }

    /// Unconditionally inserts a [`TimeColumn`].
    ///
    /// Removes and replaces the column if it already exists.
    ///
    /// This will fail if the end result is malformed in any way -- see [`Self::sanity_check`].
    #[inline]
    pub fn add_timeline(&mut self, chunk_timeline: TimeColumn) -> ChunkResult<()> {
        self.timelines
            .insert(chunk_timeline.timeline, chunk_timeline);
        self.sanity_check()
    }
}

impl TimeColumn {
    /// Creates a new [`TimeColumn`].
    ///
    /// Iff you know for sure whether the data is already appropriately sorted or not, specify `is_sorted`.
    /// When left unspecified (`None`), it will be computed in O(n) time.
    ///
    /// For a row-oriented constructor, see [`Self::builder`].
    pub fn new(is_sorted: Option<bool>, timeline: Timeline, times: ArrowScalarBuffer<i64>) -> Self {
        re_tracing::profile_function_if!(1000 < times.len(), format!("{} times", times.len()));

        let time_slice = times.as_ref();

        let is_sorted =
            is_sorted.unwrap_or_else(|| time_slice.windows(2).all(|times| times[0] <= times[1]));

        let time_range = if is_sorted {
            // NOTE: The 'or' in 'map_or' is never hit, but better safe than sorry.
            let min_time = time_slice
                .first()
                .copied()
                .map_or(TimeInt::MIN, TimeInt::new_temporal);
            let max_time = time_slice
                .last()
                .copied()
                .map_or(TimeInt::MAX, TimeInt::new_temporal);
            ResolvedTimeRange::new(min_time, max_time)
        } else {
            // NOTE: Do the iteration multiple times in a cache-friendly way rather than the opposite.
            // NOTE: The 'or' in 'unwrap_or' is never hit, but better safe than sorry.
            let min_time = time_slice
                .iter()
                .min()
                .copied()
                .map_or(TimeInt::MIN, TimeInt::new_temporal);
            let max_time = time_slice
                .iter()
                .max()
                .copied()
                .map_or(TimeInt::MAX, TimeInt::new_temporal);
            ResolvedTimeRange::new(min_time, max_time)
        };

        Self {
            timeline,
            times,
            is_sorted,
            time_range,
        }
    }

    /// Creates a new [`TimeColumn`] of sequence type.
    pub fn new_sequence(
        name: impl Into<re_log_types::TimelineName>,
        times: impl IntoIterator<Item = impl Into<i64>>,
    ) -> Self {
        let time_vec: Vec<_> = times.into_iter().map(|t| {
            let t = t.into();
            TimeInt::try_from(t)
                .unwrap_or_else(|_| {
                    re_log::error!(
                illegal_value = t,
                new_value = TimeInt::MIN.as_i64(),
                "TimeColumn::new_sequence() called with illegal value - clamped to minimum legal value"
            );
                    TimeInt::MIN
                })
                .as_i64()
        }).collect();

        Self::new(
            None,
            Timeline::new_sequence(name.into()),
            ArrowScalarBuffer::from(time_vec),
        )
    }

    /// Creates a new [`TimeColumn`] of sequence type.
    pub fn new_seconds(
        name: impl Into<re_log_types::TimelineName>,
        times: impl IntoIterator<Item = impl Into<f64>>,
    ) -> Self {
        let time_vec = times.into_iter().map(|t| {
            let t = t.into();
            let time = Time::from_seconds_since_epoch(t);
            TimeInt::try_from(time)
                .unwrap_or_else(|_| {
                    re_log::error!(
                illegal_value = t,
                new_value = TimeInt::MIN.as_i64(),
                "TimeColumn::new_seconds() called with illegal value - clamped to minimum legal value"
            );
                    TimeInt::MIN
                })
                .as_i64()
        }).collect_vec();

        Self::new(
            None,
            Timeline::new_temporal(name.into()),
            ArrowScalarBuffer::from(time_vec),
        )
    }

    /// Creates a new [`TimeColumn`] of nanoseconds type.
    pub fn new_nanos(
        name: impl Into<re_log_types::TimelineName>,
        times: impl IntoIterator<Item = impl Into<i64>>,
    ) -> Self {
        let time_vec = times
            .into_iter()
            .map(|t| {
                let t = t.into();
                let time = Time::from_ns_since_epoch(t);
                TimeInt::try_from(time)
                    .unwrap_or_else(|_| {
                        re_log::error!(
                illegal_value = t,
                new_value = TimeInt::MIN.as_i64(),
                "TimeColumn::new_nanos() called with illegal value - clamped to minimum legal value"
            );
                        TimeInt::MIN
                    })
                    .as_i64()
            })
            .collect_vec();

        Self::new(
            None,
            Timeline::new_temporal(name.into()),
            ArrowScalarBuffer::from(time_vec),
        )
    }

    /// Parse the given [`ArrowArray`] as a time column.
    ///
    /// Results in an error if the array is of the wrong datatype, or if it contains nulls.
    pub fn read_array(array: &dyn ArrowArray) -> Result<ArrowScalarBuffer<i64>, TimeColumnError> {
        #![allow(clippy::manual_map)]

        if array.null_count() > 0 {
            Err(TimeColumnError::ContainsNulls)
        } else {
            Self::read_nullable_array(array).map(|(times, _nulls)| times)
        }
    }

    /// Parse the given [`ArrowArray`] as a time column where null values are acceptable.
    ///
    /// Results in an error if the array is of the wrong datatype.
    pub fn read_nullable_array(
        array: &dyn ArrowArray,
    ) -> Result<(ArrowScalarBuffer<i64>, Option<ArrowNullBuffer>), TimeColumnError> {
        #![allow(clippy::manual_map)]

        // Sequence timelines are i64, but time columns are nanoseconds (also as i64).
        if let Some(times) = array.downcast_array_ref::<arrow::array::Int64Array>() {
            Ok((times.values().clone(), times.nulls().cloned()))
        } else if let Some(times) =
            array.downcast_array_ref::<arrow::array::TimestampNanosecondArray>()
        {
            Ok((times.values().clone(), times.nulls().cloned()))
        } else if let Some(times) =
            array.downcast_array_ref::<arrow::array::Time64NanosecondArray>()
        {
            Ok((times.values().clone(), times.nulls().cloned()))
        } else if let Some(times) =
            array.downcast_array_ref::<arrow::array::DurationNanosecondArray>()
        {
            Ok((times.values().clone(), times.nulls().cloned()))
        } else {
            Err(TimeColumnError::UnsupportedDataType(
                array.data_type().clone(),
            ))
        }
    }
}

// ---

impl Chunk {
    #[inline]
    pub fn id(&self) -> ChunkId {
        self.id
    }

    #[inline]
    pub fn entity_path(&self) -> &EntityPath {
        &self.entity_path
    }

    /// How many columns in total? Includes control, time, and component columns.
    #[inline]
    pub fn num_columns(&self) -> usize {
        let Self {
            id: _,
            entity_path: _, // not an actual column
            heap_size_bytes: _,
            is_sorted: _,
            row_ids: _,
            timelines,
            components,
        } = self;

        1 /* row_ids */ + timelines.len() + components.len()
    }

    #[inline]
    pub fn num_controls(&self) -> usize {
        _ = self;
        1 /* row_ids */
    }

    #[inline]
    pub fn num_timelines(&self) -> usize {
        self.timelines.len()
    }

    #[inline]
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    #[inline]
    pub fn num_rows(&self) -> usize {
        self.row_ids.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.num_rows() == 0
    }

    #[inline]
    pub fn row_ids_array(&self) -> &FixedSizeBinaryArray {
        &self.row_ids
    }

    #[inline]
    pub fn row_ids_slice(&self) -> &[RowId] {
        RowId::slice_from_arrow(&self.row_ids)
    }

    /// All the [`RowId`] in this chunk.
    ///
    /// This could be in any order if this chunk is unsorted.
    #[inline]
    pub fn row_ids(&self) -> impl ExactSizeIterator<Item = RowId> + '_ {
        self.row_ids_slice().iter().copied()
    }

    /// Returns an iterator over the [`RowId`]s of a [`Chunk`], for a given component.
    ///
    /// This is different than [`Self::row_ids`]: it will only yield `RowId`s for rows at which
    /// there is data for the specified `component_name`.
    #[inline]
    pub fn component_row_ids(
        &self,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = RowId> + '_ {
        let Some(list_array) = self.get_first_component(component_name) else {
            return Either::Left(std::iter::empty());
        };

        let row_ids = self.row_ids();

        if let Some(validity) = list_array.nulls() {
            Either::Right(Either::Left(
                row_ids
                    .enumerate()
                    .filter_map(|(i, o)| validity.is_valid(i).then_some(o)),
            ))
        } else {
            Either::Right(Either::Right(row_ids))
        }
    }

    /// Returns the [`RowId`]-range covered by this [`Chunk`].
    ///
    /// `None` if the chunk `is_empty`.
    ///
    /// This is O(1) if the chunk is sorted, O(n) otherwise.
    #[inline]
    pub fn row_id_range(&self) -> Option<(RowId, RowId)> {
        if self.is_empty() {
            return None;
        }

        let row_ids = self.row_ids_slice();

        #[allow(clippy::unwrap_used)] // checked above
        Some(if self.is_sorted() {
            (
                row_ids.first().copied().unwrap(),
                row_ids.last().copied().unwrap(),
            )
        } else {
            (
                row_ids.iter().min().copied().unwrap(),
                row_ids.iter().max().copied().unwrap(),
            )
        })
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.timelines.is_empty()
    }

    #[inline]
    pub fn timelines(&self) -> &IntMap<Timeline, TimeColumn> {
        &self.timelines
    }

    #[inline]
    pub fn component_names(&self) -> impl Iterator<Item = ComponentName> + '_ {
        self.components.keys().copied()
    }

    #[inline]
    pub fn component_descriptors(&self) -> impl Iterator<Item = ComponentDescriptor> + '_ {
        self.components
            .values()
            .flat_map(|per_desc| per_desc.keys())
            .cloned()
    }

    #[inline]
    pub fn components(&self) -> &ChunkComponents {
        &self.components
    }

    /// Computes the maximum value for each and every timeline present across this entire chunk,
    /// and returns the corresponding [`TimePoint`].
    #[inline]
    pub fn timepoint_max(&self) -> TimePoint {
        self.timelines
            .iter()
            .map(|(timeline, info)| (*timeline, info.time_range.max()))
            .collect()
    }
}

impl std::fmt::Display for Chunk {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let batch = self.to_record_batch().map_err(|err| {
            re_log::error_once!("couldn't display Chunk: {err}");
            std::fmt::Error
        })?;
        re_format_arrow::format_record_batch_with_width(&batch, f.width()).fmt(f)
    }
}

impl TimeColumn {
    #[inline]
    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.timeline.name()
    }

    #[inline]
    pub fn time_range(&self) -> ResolvedTimeRange {
        self.time_range
    }

    #[inline]
    pub fn times_buffer(&self) -> &ArrowScalarBuffer<i64> {
        &self.times
    }

    /// Returns an array with the appropriate datatype.
    #[inline]
    pub fn times_array(&self) -> ArrowArrayRef {
        self.timeline.typ().make_arrow_array(self.times.clone())
    }

    #[inline]
    pub fn times_raw(&self) -> &[i64] {
        self.times.as_ref()
    }

    #[inline]
    pub fn times(&self) -> impl DoubleEndedIterator<Item = TimeInt> + '_ {
        self.times_raw().iter().copied().map(TimeInt::new_temporal)
    }

    #[inline]
    pub fn num_rows(&self) -> usize {
        self.times.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.num_rows() == 0
    }

    /// Computes the time range covered by each individual component column.
    ///
    /// This is different from the time range covered by the [`TimeColumn`] as a whole
    /// because component columns are potentially sparse.
    ///
    /// This is crucial for indexing and queries to work properly.
    //
    // TODO(cmc): This needs to be stored in chunk metadata and transported across IPC.
    pub fn time_range_per_component(
        &self,
        components: &ChunkComponents,
    ) -> IntMap<ComponentName, IntMap<ComponentDescriptor, ResolvedTimeRange>> {
        let times = self.times_raw();
        components
            .iter()
            .map(|(component_name, per_desc)| {
                (
                    *component_name,
                    per_desc
                        .iter()
                        .filter_map(|(component_desc, list_array)| {
                            if let Some(validity) = list_array.nulls() {
                                // Potentially sparse

                                if validity.is_empty() {
                                    return None;
                                }

                                let is_dense = validity.null_count() == 0;
                                if is_dense {
                                    return Some((component_desc.clone(), self.time_range));
                                }

                                let mut time_min = TimeInt::MAX;
                                for (i, time) in times.iter().copied().enumerate() {
                                    if validity.is_valid(i) {
                                        time_min = TimeInt::new_temporal(time);
                                        break;
                                    }
                                }

                                let mut time_max = TimeInt::MIN;
                                for (i, time) in times.iter().copied().enumerate().rev() {
                                    if validity.is_valid(i) {
                                        time_max = TimeInt::new_temporal(time);
                                        break;
                                    }
                                }

                                Some((
                                    component_desc.clone(),
                                    ResolvedTimeRange::new(time_min, time_max),
                                ))
                            } else {
                                // Dense

                                Some((component_desc.clone(), self.time_range))
                            }
                        })
                        .collect(),
                )
            })
            .collect()
    }
}

impl re_byte_size::SizeBytes for Chunk {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self {
            id,
            entity_path,
            heap_size_bytes,
            is_sorted,
            row_ids,
            timelines,
            components,
        } = self;

        let mut size_bytes = heap_size_bytes.load(Ordering::Relaxed);

        if size_bytes == 0 {
            size_bytes = id.heap_size_bytes()
                + entity_path.heap_size_bytes()
                + is_sorted.heap_size_bytes()
                + row_ids.heap_size_bytes()
                + timelines.heap_size_bytes()
                + components.heap_size_bytes();
            heap_size_bytes.store(size_bytes, Ordering::Relaxed);
        }

        size_bytes
    }
}

impl re_byte_size::SizeBytes for TimeColumn {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self {
            timeline,
            times,
            is_sorted,
            time_range,
        } = self;

        timeline.heap_size_bytes()
            + times.heap_size_bytes() // cheap
            + is_sorted.heap_size_bytes()
            + time_range.heap_size_bytes()
    }
}

// --- Sanity checks ---

impl Chunk {
    /// Returns an error if the Chunk's invariants are not upheld.
    ///
    /// Costly checks are only run in debug builds.
    #[track_caller]
    pub fn sanity_check(&self) -> ChunkResult<()> {
        re_tracing::profile_function!();

        let Self {
            id: _,
            entity_path: _,
            heap_size_bytes,
            is_sorted,
            row_ids,
            timelines,
            components,
        } = self;

        #[allow(clippy::collapsible_if)] // readability
        if cfg!(debug_assertions) {
            let measured = self.heap_size_bytes();
            let advertised = heap_size_bytes.load(Ordering::Relaxed);
            if advertised != measured {
                return Err(ChunkError::Malformed {
                    reason: format!(
                        "Chunk advertises a heap size of {} but we measure {} instead",
                        re_format::format_bytes(advertised as _),
                        re_format::format_bytes(measured as _),
                    ),
                });
            }
        }

        // Row IDs
        {
            if *row_ids.data_type() != RowId::arrow_datatype() {
                return Err(ChunkError::Malformed {
                    reason: format!(
                        "RowId data has the wrong datatype: expected {:?} but got {:?} instead",
                        RowId::arrow_datatype(),
                        *row_ids.data_type(),
                    ),
                });
            }

            #[allow(clippy::collapsible_if)] // readability
            if cfg!(debug_assertions) {
                if *is_sorted != self.is_sorted_uncached() {
                    return Err(ChunkError::Malformed {
                        reason: format!(
                            "Chunk is marked as {}sorted but isn't: {row_ids:?}",
                            if *is_sorted { "" } else { "un" },
                        ),
                    });
                }
            }
        }

        // Timelines
        for (timeline, time_column) in timelines {
            if time_column.times.len() != row_ids.len() {
                return Err(ChunkError::Malformed {
                    reason: format!(
                        "All timelines in a chunk must have the same number of timestamps, matching the number of row IDs.\
                         Found {} row IDs but {} timestamps for timeline {:?}",
                        row_ids.len(), time_column.times.len(), timeline.name(),
                    ),
                });
            }

            time_column.sanity_check()?;
        }

        // Components
        for (component_name, per_desc) in components.iter() {
            component_name.sanity_check();
            for (component_desc, list_array) in per_desc {
                component_desc.component_name.sanity_check();
                // Ensure that each cell is a list (we don't support mono-components yet).
                if let arrow::datatypes::DataType::List(_field) = list_array.data_type() {
                    // We don't check `field.is_nullable()` here because we support both.
                    // TODO(#6819): Remove support for inner nullability.
                } else {
                    return Err(ChunkError::Malformed {
                        reason: format!(
                            "The inner array in a chunked component batch must be a list, got {:?}",
                            list_array.data_type(),
                        ),
                    });
                }

                if list_array.len() != row_ids.len() {
                    return Err(ChunkError::Malformed {
                        reason: format!(
                            "All component batches in a chunk must have the same number of rows, matching the number of row IDs.\
                             Found {} row IDs but {} rows for component batch {component_desc}",
                            row_ids.len(), list_array.len(),
                        ),
                    });
                }

                let validity_is_empty = list_array
                    .nulls()
                    .is_some_and(|validity| validity.is_empty());
                if !self.is_empty() && validity_is_empty {
                    return Err(ChunkError::Malformed {
                        reason: format!(
                            "All component batches in a chunk must contain at least one non-null entry.\
                             Found a completely empty column for {component_desc}",
                        ),
                    });
                }
            }
        }

        Ok(())
    }
}

impl TimeColumn {
    /// Returns an error if the Chunk's invariants are not upheld.
    ///
    /// Costly checks are only run in debug builds.
    #[track_caller]
    pub fn sanity_check(&self) -> ChunkResult<()> {
        let Self {
            timeline: _,
            times,
            is_sorted,
            time_range,
        } = self;

        let times = times.as_ref();

        #[allow(clippy::collapsible_if)] // readability
        if cfg!(debug_assertions) {
            if *is_sorted != times.windows(2).all(|times| times[0] <= times[1]) {
                return Err(ChunkError::Malformed {
                    reason: format!(
                        "Time column is marked as {}sorted but isn't: {times:?}",
                        if *is_sorted { "" } else { "un" },
                    ),
                });
            }
        }

        #[allow(clippy::collapsible_if)] // readability
        if cfg!(debug_assertions) {
            let is_tight_lower_bound = times.iter().any(|&time| time == time_range.min().as_i64());
            let is_tight_upper_bound = times.iter().any(|&time| time == time_range.max().as_i64());
            let is_tight_bound = is_tight_lower_bound && is_tight_upper_bound;

            if !self.is_empty() && !is_tight_bound {
                return Err(ChunkError::Malformed {
                    reason: "Time column's cached time range isn't a tight bound.".to_owned(),
                });
            }

            for &time in times {
                if time < time_range.min().as_i64() || time > time_range.max().as_i64() {
                    return Err(ChunkError::Malformed {
                        reason: format!(
                            "Time column's cached time range is wrong.\
                             Found a time value of {time} while its time range is {time_range:?}",
                        ),
                    });
                }

                if time == TimeInt::STATIC.as_i64() {
                    return Err(ChunkError::Malformed {
                        reason: "A chunk's timeline should never contain a static time value."
                            .to_owned(),
                    });
                }
            }
        }

        Ok(())
    }
}
