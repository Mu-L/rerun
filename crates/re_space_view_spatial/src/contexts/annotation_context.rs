use re_types::{archetypes::AnnotationContext, Archetype, ComponentNameSet};
use re_viewer_context::{AnnotationMap, NamedViewSystem, ViewContextSystem, ViewSystemName};

#[derive(Default)]
pub struct AnnotationSceneContext(pub AnnotationMap);

impl NamedViewSystem for AnnotationSceneContext {
    fn name() -> ViewSystemName {
        "AnnotationSceneContext".into()
    }
}

impl ViewContextSystem for AnnotationSceneContext {
    fn compatible_component_sets(&self) -> Vec<ComponentNameSet> {
        vec![
            AnnotationContext::required_components()
                .iter()
                .map(ToOwned::to_owned)
                .collect(), //
        ]
    }

    fn execute(
        &mut self,
        ctx: &mut re_viewer_context::ViewerContext<'_>,
        query: &re_viewer_context::ViewQuery<'_>,
    ) {
        re_tracing::profile_function!();
        // We create a list of *all* entities here, do not only iterate over those with annotation context.
        // TODO(andreas): But knowing ahead of time where we have annotation contexts could be used for optimization.
        self.0
            .load(ctx, &query.latest_at_query(), query.iter_entities());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
