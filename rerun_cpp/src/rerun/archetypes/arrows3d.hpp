// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/arrows3d.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/position3d.hpp"
#include "../components/radius.hpp"
#include "../components/show_labels.hpp"
#include "../components/text.hpp"
#include "../components/vector3d.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 3D arrows with optional colors, radii, labels, etc.
    ///
    /// ## Example
    ///
    /// ### Simple batch of 3D arrows
    /// ![image](https://static.rerun.io/arrow3d_simple/55e2f794a520bbf7527d7b828b0264732146c5d0/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <cmath>
    /// #include <vector>
    ///
    /// constexpr float TAU = 6.28318530717958647692528676655900577f;
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_arrow3d");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     std::vector<rerun::Position3D> origins;
    ///     std::vector<rerun::Vector3D> vectors;
    ///     std::vector<rerun::Color> colors;
    ///
    ///     for (int i = 0; i <100; ++i) {
    ///         origins.push_back({0, 0, 0});
    ///
    ///         float angle = TAU * static_cast<float>(i) * 0.01f;
    ///         float length = static_cast<float>(log2(i + 1));
    ///         vectors.push_back({length * sinf(angle), 0.0, length * cosf(angle)});
    ///
    ///         uint8_t c = static_cast<uint8_t>(round(angle / TAU * 255.0f));
    ///         colors.push_back({static_cast<uint8_t>(255 - c), c, 128, 128});
    ///     }
    ///
    ///     rec.log(
    ///         "arrows",
    ///         rerun::Arrows3D::from_vectors(vectors).with_origins(origins).with_colors(colors)
    ///     );
    /// }
    /// ```
    struct Arrows3D {
        /// All the vectors for each arrow in the batch.
        std::optional<ComponentBatch> vectors;

        /// All the origin (base) positions for each arrow in the batch.
        ///
        /// If no origins are set, (0, 0, 0) is used as the origin for each arrow.
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

        /// Optional choice of whether the text labels should be shown by default.
        std::optional<ComponentBatch> show_labels;

        /// Optional class Ids for the points.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        std::optional<ComponentBatch> class_ids;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.Arrows3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.Arrows3D";

        /// `ComponentDescriptor` for the `vectors` field.
        static constexpr auto Descriptor_vectors = ComponentDescriptor(
            ArchetypeName, "vectors",
            Loggable<rerun::components::Vector3D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `origins` field.
        static constexpr auto Descriptor_origins = ComponentDescriptor(
            ArchetypeName, "origins",
            Loggable<rerun::components::Position3D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `radii` field.
        static constexpr auto Descriptor_radii = ComponentDescriptor(
            ArchetypeName, "radii", Loggable<rerun::components::Radius>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `colors` field.
        static constexpr auto Descriptor_colors = ComponentDescriptor(
            ArchetypeName, "colors", Loggable<rerun::components::Color>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `labels` field.
        static constexpr auto Descriptor_labels = ComponentDescriptor(
            ArchetypeName, "labels", Loggable<rerun::components::Text>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `show_labels` field.
        static constexpr auto Descriptor_show_labels = ComponentDescriptor(
            ArchetypeName, "show_labels",
            Loggable<rerun::components::ShowLabels>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `class_ids` field.
        static constexpr auto Descriptor_class_ids = ComponentDescriptor(
            ArchetypeName, "class_ids",
            Loggable<rerun::components::ClassId>::Descriptor.component_name
        );

      public: // START of extensions from arrows3d_ext.cpp:
        /// Creates new 3D arrows pointing in the given directions, with a base at the origin (0, 0,
        /// 0).
        static Arrows3D from_vectors(Collection<components::Vector3D> vectors_) {
            return Arrows3D().with_vectors(vectors_);
        }

        // END of extensions from arrows3d_ext.cpp, start of generated code:

      public:
        Arrows3D() = default;
        Arrows3D(Arrows3D&& other) = default;
        Arrows3D(const Arrows3D& other) = default;
        Arrows3D& operator=(const Arrows3D& other) = default;
        Arrows3D& operator=(Arrows3D&& other) = default;

        /// Update only some specific fields of a `Arrows3D`.
        static Arrows3D update_fields() {
            return Arrows3D();
        }

        /// Clear all the fields of a `Arrows3D`.
        static Arrows3D clear_fields();

        /// All the vectors for each arrow in the batch.
        Arrows3D with_vectors(const Collection<rerun::components::Vector3D>& _vectors) && {
            vectors = ComponentBatch::from_loggable(_vectors, Descriptor_vectors).value_or_throw();
            return std::move(*this);
        }

        /// All the origin (base) positions for each arrow in the batch.
        ///
        /// If no origins are set, (0, 0, 0) is used as the origin for each arrow.
        Arrows3D with_origins(const Collection<rerun::components::Position3D>& _origins) && {
            origins = ComponentBatch::from_loggable(_origins, Descriptor_origins).value_or_throw();
            return std::move(*this);
        }

        /// Optional radii for the arrows.
        ///
        /// The shaft is rendered as a line with `radius = 0.5 * radius`.
        /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
        Arrows3D with_radii(const Collection<rerun::components::Radius>& _radii) && {
            radii = ComponentBatch::from_loggable(_radii, Descriptor_radii).value_or_throw();
            return std::move(*this);
        }

        /// Optional colors for the points.
        Arrows3D with_colors(const Collection<rerun::components::Color>& _colors) && {
            colors = ComponentBatch::from_loggable(_colors, Descriptor_colors).value_or_throw();
            return std::move(*this);
        }

        /// Optional text labels for the arrows.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        Arrows3D with_labels(const Collection<rerun::components::Text>& _labels) && {
            labels = ComponentBatch::from_loggable(_labels, Descriptor_labels).value_or_throw();
            return std::move(*this);
        }

        /// Optional choice of whether the text labels should be shown by default.
        Arrows3D with_show_labels(const rerun::components::ShowLabels& _show_labels) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `show_labels` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_show_labels` should
        /// be used when logging a single row's worth of data.
        Arrows3D with_many_show_labels(const Collection<rerun::components::ShowLabels>& _show_labels
        ) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            return std::move(*this);
        }

        /// Optional class Ids for the points.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        Arrows3D with_class_ids(const Collection<rerun::components::ClassId>& _class_ids) && {
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
    struct AsComponents<archetypes::Arrows3D> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(const archetypes::Arrows3D& archetype);
    };
} // namespace rerun
