//! Log a scalar over time.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar_multiple_plots").spawn()?;
    let mut lcg_state = 0_i64;

    // Set up plot styling:
    // They are logged static as they don't change over time and apply to all timelines.
    // Log two lines series under a shared root so that they show in the same plot by default.
    rec.log_static(
        "trig/sin",
        &rerun::SeriesLines::new()
            .with_colors([[255, 0, 0]])
            .with_names(["sin(0.01t)"]),
    )?;
    rec.log_static(
        "trig/cos",
        &rerun::SeriesLines::new()
            .with_colors([[0, 255, 0]])
            .with_names(["cos(0.01t)"]),
    )?;

    // NOTE: `SeriesLines` and `SeriesPoints` can both be logged without any associated data
    //       (all fields are optional). In `v0.24` we removed indicators, which now results in
    //       no data logged at all, when no fields are specified. Therefore we log a circle shape
    //       here. More information: https://github.com/rerun-io/rerun/issues/10512

    // Log scattered points under a different root so that they show in a different plot by default.
    rec.log_static(
        "scatter/lcg",
        &rerun::SeriesPoints::new().with_markers([rerun::components::MarkerShape::Circle]),
    )?;

    for t in 0..((std::f32::consts::TAU * 2.0 * 100.0) as i64) {
        rec.set_time_sequence("step", t);

        // Log two time series under a shared root so that they show in the same plot by default.
        rec.log(
            "trig/sin",
            &rerun::Scalars::single((t as f64 / 100.0).sin()),
        )?;
        rec.log(
            "trig/cos",
            &rerun::Scalars::single((t as f64 / 100.0).cos()),
        )?;

        // Log scattered points under a different root so that it shows in a different plot by default.
        lcg_state = (1140671485_i64
            .wrapping_mul(lcg_state)
            .wrapping_add(128201163))
            % 16777216; // simple linear congruency generator
        rec.log("scatter/lcg", &rerun::Scalars::single(lcg_state as f64))?;
    }

    Ok(())
}
