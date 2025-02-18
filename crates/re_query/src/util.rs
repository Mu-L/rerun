use re_arrow_store::{DataStore, LatestAtQuery, RangeQuery, TimeInt, TimeRange, Timeline};
use re_data_store::ExtraQueryHistory;
use re_log_types::EntityPath;
use re_types_core::Archetype;

use crate::{query_archetype, range::range_archetype, ArchetypeView};

pub fn query_archetype_with_history<'a, A: Archetype + 'a, const N: usize>(
    store: &'a DataStore,
    timeline: &'a Timeline,
    time: &'a TimeInt,
    history: &ExtraQueryHistory,
    ent_path: &'a EntityPath,
) -> crate::Result<impl Iterator<Item = ArchetypeView<A>> + 'a> {
    let visible_history = match timeline.typ() {
        re_log_types::TimeType::Time => history.nanos,
        re_log_types::TimeType::Sequence => history.sequences,
    };

    if visible_history == 0 {
        let latest_query = LatestAtQuery::new(*timeline, *time);
        let latest = query_archetype::<A>(store, &latest_query, ent_path)?;

        Ok(itertools::Either::Left(std::iter::once(latest)))
    } else {
        let min_time = *time - TimeInt::from(visible_history);
        let range_query = RangeQuery::new(*timeline, TimeRange::new(min_time, *time));

        let range = range_archetype::<A, N>(store, &range_query, ent_path);

        Ok(itertools::Either::Right(range.map(|(_, entity)| entity)))
    }
}
