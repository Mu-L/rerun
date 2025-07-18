// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/arrows2d.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/draw_order.hpp"
#include "../components/position2d.hpp"
#include "../components/radius.hpp"
#include "../components/show_labels.hpp"
#include "../components/text.hpp"
#include "../components/vector2d.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 2D arrows with optional colors, radii, labels, etc.
    ///
    /// ## Example
    ///
    /// ### Simple batch of 2D arrows
    /// ![image](https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_arrow2d");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log(
    ///         "arrows",
    ///         rerun::Arrows2D::from_vectors({{1.0f, 0.0f}, {0.0f, -1.0f}, {-0.7f, 0.7f}})
    ///             .with_radii(0.025f)
    ///             .with_origins({{0.25f, 0.0f}, {0.25f, 0.0f}, {-0.1f, -0.1f}})
    ///             .with_colors({{255, 0, 0}, {0, 255, 0}, {127, 0, 255}})
    ///             .with_labels({"right", "up", "left-down"})
    ///     );
    /// }
    /// ```
    struct Arrows2D {
        /// All the vectors for each arrow in the batch.
        std::optional<ComponentBatch> vectors;

        /// All the origin (base) positions for each arrow in the batch.
        ///
        /// If no origins are set, (0, 0) is used as the origin for each arrow.
        std::optional<ComponentBatch> origins;

        /// Optional radii for the arrows.
        ///
        /// The shaft is rendered as a line with `radius = 0.5 * radius`.
        /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
        std::optional<ComponentBatch> radii;

        /// Optional colors for the points.
        std::optional<ComponentBatch> colors;

        /// Optional text labels for the arrows.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        std::optional<ComponentBatch> labels;

        /// Whether the text labels should be shown.
        ///
        /// If not set, labels will automatically appear when there is exactly one label for this entity
        /// or the number of instances on this entity is under a certain threshold.
        std::optional<ComponentBatch> show_labels;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<ComponentBatch> draw_order;

        /// Optional class Ids for the points.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        std::optional<ComponentBatch> class_ids;

      public:
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.Arrows2D";

        /// `ComponentDescriptor` for the `vectors` field.
        static constexpr auto Descriptor_vectors = ComponentDescriptor(
            ArchetypeName, "Arrows2D:vectors", Loggable<rerun::components::Vector2D>::ComponentType
        );
        /// `ComponentDescriptor` for the `origins` field.
        static constexpr auto Descriptor_origins = ComponentDescriptor(
            ArchetypeName, "Arrows2D:origins",
            Loggable<rerun::components::Position2D>::ComponentType
        );
        /// `ComponentDescriptor` for the `radii` field.
        static constexpr auto Descriptor_radii = ComponentDescriptor(
            ArchetypeName, "Arrows2D:radii", Loggable<rerun::components::Radius>::ComponentType
        );
        /// `ComponentDescriptor` for the `colors` field.
        static constexpr auto Descriptor_colors = ComponentDescriptor(
            ArchetypeName, "Arrows2D:colors", Loggable<rerun::components::Color>::ComponentType
        );
        /// `ComponentDescriptor` for the `labels` field.
        static constexpr auto Descriptor_labels = ComponentDescriptor(
            ArchetypeName, "Arrows2D:labels", Loggable<rerun::components::Text>::ComponentType
        );
        /// `ComponentDescriptor` for the `show_labels` field.
        static constexpr auto Descriptor_show_labels = ComponentDescriptor(
            ArchetypeName, "Arrows2D:show_labels",
            Loggable<rerun::components::ShowLabels>::ComponentType
        );
        /// `ComponentDescriptor` for the `draw_order` field.
        static constexpr auto Descriptor_draw_order = ComponentDescriptor(
            ArchetypeName, "Arrows2D:draw_order",
            Loggable<rerun::components::DrawOrder>::ComponentType
        );
        /// `ComponentDescriptor` for the `class_ids` field.
        static constexpr auto Descriptor_class_ids = ComponentDescriptor(
            ArchetypeName, "Arrows2D:class_ids", Loggable<rerun::components::ClassId>::ComponentType
        );

      public: // START of extensions from arrows2d_ext.cpp:
        /// Creates new 2D arrows pointing in the given directions, with a base at the origin (0, 0).
        static Arrows2D from_vectors(Collection<components::Vector2D> vectors_) {
            return Arrows2D().with_vectors(vectors_);
        }

        // END of extensions from arrows2d_ext.cpp, start of generated code:

      public:
        Arrows2D() = default;
        Arrows2D(Arrows2D&& other) = default;
        Arrows2D(const Arrows2D& other) = default;
        Arrows2D& operator=(const Arrows2D& other) = default;
        Arrows2D& operator=(Arrows2D&& other) = default;

        /// Update only some specific fields of a `Arrows2D`.
        static Arrows2D update_fields() {
            return Arrows2D();
        }

        /// Clear all the fields of a `Arrows2D`.
        static Arrows2D clear_fields();

        /// All the vectors for each arrow in the batch.
        Arrows2D with_vectors(const Collection<rerun::components::Vector2D>& _vectors) && {
            vectors = ComponentBatch::from_loggable(_vectors, Descriptor_vectors).value_or_throw();
            return std::move(*this);
        }

        /// All the origin (base) positions for each arrow in the batch.
        ///
        /// If no origins are set, (0, 0) is used as the origin for each arrow.
        Arrows2D with_origins(const Collection<rerun::components::Position2D>& _origins) && {
            origins = ComponentBatch::from_loggable(_origins, Descriptor_origins).value_or_throw();
            return std::move(*this);
        }

        /// Optional radii for the arrows.
        ///
        /// The shaft is rendered as a line with `radius = 0.5 * radius`.
        /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
        Arrows2D with_radii(const Collection<rerun::components::Radius>& _radii) && {
            radii = ComponentBatch::from_loggable(_radii, Descriptor_radii).value_or_throw();
            return std::move(*this);
        }

        /// Optional colors for the points.
        Arrows2D with_colors(const Collection<rerun::components::Color>& _colors) && {
            colors = ComponentBatch::from_loggable(_colors, Descriptor_colors).value_or_throw();
            return std::move(*this);
        }

        /// Optional text labels for the arrows.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        Arrows2D with_labels(const Collection<rerun::components::Text>& _labels) && {
            labels = ComponentBatch::from_loggable(_labels, Descriptor_labels).value_or_throw();
            return std::move(*this);
        }

        /// Whether the text labels should be shown.
        ///
        /// If not set, labels will automatically appear when there is exactly one label for this entity
        /// or the number of instances on this entity is under a certain threshold.
        Arrows2D with_show_labels(const rerun::components::ShowLabels& _show_labels) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `show_labels` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_show_labels` should
        /// be used when logging a single row's worth of data.
        Arrows2D with_many_show_labels(const Collection<rerun::components::ShowLabels>& _show_labels
        ) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            return std::move(*this);
        }

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        Arrows2D with_draw_order(const rerun::components::DrawOrder& _draw_order) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `draw_order` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_draw_order` should
        /// be used when logging a single row's worth of data.
        Arrows2D with_many_draw_order(const Collection<rerun::components::DrawOrder>& _draw_order
        ) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
            return std::move(*this);
        }

        /// Optional class Ids for the points.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        Arrows2D with_class_ids(const Collection<rerun::components::ClassId>& _class_ids) && {
            class_ids =
                ComponentBatch::from_loggable(_class_ids, Descriptor_class_ids).value_or_throw();
            return std::move(*this);
        }

        /// Partitions the component data into multiple sub-batches.
        ///
        /// Specifically, this transforms the existing `ComponentBatch` data into `ComponentColumn`s
        /// instead, via `ComponentBatch::partitioned`.
        ///
        /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
        ///
        /// The specified `lengths` must sum to the total length of the component batch.
        Collection<ComponentColumn> columns(const Collection<uint32_t>& lengths_);

        /// Partitions the component data into unit-length sub-batches.
        ///
        /// This is semantically similar to calling `columns` with `std::vector<uint32_t>(n, 1)`,
        /// where `n` is automatically guessed.
        Collection<ComponentColumn> columns();
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Arrows2D> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(const archetypes::Arrows2D& archetype);
    };
} // namespace rerun
