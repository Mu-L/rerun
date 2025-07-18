// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/text_log.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/color.hpp"
#include "../components/text.hpp"
#include "../components/text_log_level.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A log entry in a text log, comprised of a text body and its log level.
    ///
    /// ## Example
    ///
    /// ### text_log_integration:
    /// ![image](https://static.rerun.io/text_log_integration/9737d0c986325802a9885499d6fcc773b1736488/full.png)
    ///
    /// ```cpp
    /// #include <loguru.hpp>
    /// #include <rerun.hpp>
    ///
    /// void loguru_to_rerun(void* user_data, const loguru::Message& message) {
    ///     // NOTE: `rerun::RecordingStream` is thread-safe.
    ///     const rerun::RecordingStream* rec = reinterpret_cast<const rerun::RecordingStream*>(user_data);
    ///
    ///     rerun::TextLogLevel level;
    ///     if (message.verbosity == loguru::Verbosity_FATAL) {
    ///         level = rerun::TextLogLevel::Critical;
    ///     } else if (message.verbosity == loguru::Verbosity_ERROR) {
    ///         level = rerun::TextLogLevel::Error;
    ///     } else if (message.verbosity == loguru::Verbosity_WARNING) {
    ///         level = rerun::TextLogLevel::Warning;
    ///     } else if (message.verbosity == loguru::Verbosity_INFO) {
    ///         level = rerun::TextLogLevel::Info;
    ///     } else if (message.verbosity == loguru::Verbosity_1) {
    ///         level = rerun::TextLogLevel::Debug;
    ///     } else if (message.verbosity == loguru::Verbosity_2) {
    ///         level = rerun::TextLogLevel::Trace;
    ///     } else {
    ///         level = rerun::TextLogLevel(std::to_string(message.verbosity));
    ///     }
    ///
    ///     rec->log(
    ///         "logs/handler/text_log_integration",
    ///         rerun::TextLog(message.message).with_level(level)
    ///     );
    /// }
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_text_log_integration");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Log a text entry directly:
    ///     rec.log(
    ///         "logs",
    ///         rerun::TextLog("this entry has loglevel TRACE").with_level(rerun::TextLogLevel::Trace)
    ///     );
    ///
    ///     loguru::add_callback(
    ///         "rerun",
    ///         loguru_to_rerun,
    ///         const_cast<void*>(reinterpret_cast<const void*>(&rec)),
    ///         loguru::Verbosity_INFO
    ///     );
    ///
    ///     LOG_F(INFO, "This INFO log got added through the standard logging interface");
    ///
    ///     loguru::remove_callback("rerun"); // we need to do this before `rec` goes out of scope
    /// }
    /// ```
    struct TextLog {
        /// The body of the message.
        std::optional<ComponentBatch> text;

        /// The verbosity level of the message.
        ///
        /// This can be used to filter the log messages in the Rerun Viewer.
        std::optional<ComponentBatch> level;

        /// Optional color to use for the log line in the Rerun Viewer.
        std::optional<ComponentBatch> color;

      public:
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.TextLog";

        /// `ComponentDescriptor` for the `text` field.
        static constexpr auto Descriptor_text = ComponentDescriptor(
            ArchetypeName, "TextLog:text", Loggable<rerun::components::Text>::ComponentType
        );
        /// `ComponentDescriptor` for the `level` field.
        static constexpr auto Descriptor_level = ComponentDescriptor(
            ArchetypeName, "TextLog:level", Loggable<rerun::components::TextLogLevel>::ComponentType
        );
        /// `ComponentDescriptor` for the `color` field.
        static constexpr auto Descriptor_color = ComponentDescriptor(
            ArchetypeName, "TextLog:color", Loggable<rerun::components::Color>::ComponentType
        );

      public:
        TextLog() = default;
        TextLog(TextLog&& other) = default;
        TextLog(const TextLog& other) = default;
        TextLog& operator=(const TextLog& other) = default;
        TextLog& operator=(TextLog&& other) = default;

        explicit TextLog(rerun::components::Text _text)
            : text(ComponentBatch::from_loggable(std::move(_text), Descriptor_text).value_or_throw()
              ) {}

        /// Update only some specific fields of a `TextLog`.
        static TextLog update_fields() {
            return TextLog();
        }

        /// Clear all the fields of a `TextLog`.
        static TextLog clear_fields();

        /// The body of the message.
        TextLog with_text(const rerun::components::Text& _text) && {
            text = ComponentBatch::from_loggable(_text, Descriptor_text).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `text` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_text` should
        /// be used when logging a single row's worth of data.
        TextLog with_many_text(const Collection<rerun::components::Text>& _text) && {
            text = ComponentBatch::from_loggable(_text, Descriptor_text).value_or_throw();
            return std::move(*this);
        }

        /// The verbosity level of the message.
        ///
        /// This can be used to filter the log messages in the Rerun Viewer.
        TextLog with_level(const rerun::components::TextLogLevel& _level) && {
            level = ComponentBatch::from_loggable(_level, Descriptor_level).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `level` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_level` should
        /// be used when logging a single row's worth of data.
        TextLog with_many_level(const Collection<rerun::components::TextLogLevel>& _level) && {
            level = ComponentBatch::from_loggable(_level, Descriptor_level).value_or_throw();
            return std::move(*this);
        }

        /// Optional color to use for the log line in the Rerun Viewer.
        TextLog with_color(const rerun::components::Color& _color) && {
            color = ComponentBatch::from_loggable(_color, Descriptor_color).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `color` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_color` should
        /// be used when logging a single row's worth of data.
        TextLog with_many_color(const Collection<rerun::components::Color>& _color) && {
            color = ComponentBatch::from_loggable(_color, Descriptor_color).value_or_throw();
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
    struct AsComponents<archetypes::TextLog> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(const archetypes::TextLog& archetype);
    };
} // namespace rerun
