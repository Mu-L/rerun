// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/video_timestamp.fbs".

#pragma once

#include "../component_descriptor.hpp"
#include "../datatypes/video_timestamp.hpp"
#include "../result.hpp"

#include <chrono>
#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: Timestamp inside a `archetypes::AssetVideo`.
    struct VideoTimestamp {
        rerun::datatypes::VideoTimestamp timestamp;

      public: // START of extensions from video_timestamp_ext.cpp:
        /// Creates a new `VideoTimestamp` from a presentation timestamp as a chrono duration.
        template <typename TRep, typename TPeriod>
        VideoTimestamp(std::chrono::duration<TRep, TPeriod> time) {
            timestamp.timestamp_ns =
                std::chrono::duration_cast<std::chrono::nanoseconds>(time).count();
        }

        /// Creates a new `VideoTimestamp` from a presentation timestamp in seconds.
        static VideoTimestamp from_seconds(double seconds) {
            return VideoTimestamp(std::chrono::duration<double>(seconds));
        }

        /// Creates a new `VideoTimestamp` from a presentation timestamp in milliseconds.
        static VideoTimestamp from_milliseconds(double milliseconds) {
            return VideoTimestamp(std::chrono::duration<double, std::milli>(milliseconds));
        }

        /// Creates a new `VideoTimestamp` from a presentation timestamp in nanoseconds.
        static VideoTimestamp from_nanoseconds(int64_t nanoseconds) {
            return VideoTimestamp(std::chrono::nanoseconds(nanoseconds));
        }

        // END of extensions from video_timestamp_ext.cpp, start of generated code:

      public:
        VideoTimestamp() = default;

        VideoTimestamp(rerun::datatypes::VideoTimestamp timestamp_) : timestamp(timestamp_) {}

        VideoTimestamp& operator=(rerun::datatypes::VideoTimestamp timestamp_) {
            timestamp = timestamp_;
            return *this;
        }

        VideoTimestamp(int64_t timestamp_ns_) : timestamp(timestamp_ns_) {}

        VideoTimestamp& operator=(int64_t timestamp_ns_) {
            timestamp = timestamp_ns_;
            return *this;
        }

        /// Cast to the underlying VideoTimestamp datatype
        operator rerun::datatypes::VideoTimestamp() const {
            return timestamp;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::VideoTimestamp) == sizeof(components::VideoTimestamp));

    /// \private
    template <>
    struct Loggable<components::VideoTimestamp> {
        static constexpr ComponentDescriptor Descriptor = "rerun.components.VideoTimestamp";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::VideoTimestamp>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::VideoTimestamp` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::VideoTimestamp* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::VideoTimestamp>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::VideoTimestamp>::to_arrow(
                    &instances->timestamp,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
