// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/uuid.fbs".

#pragma once

#include "../component_descriptor.hpp"
#include "../result.hpp"

#include <array>
#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class FixedSizeListBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: A 16-byte UUID.
    struct Uuid {
        /// The raw bytes representing the UUID.
        std::array<uint8_t, 16> bytes;

      public:
        Uuid() = default;

        Uuid(std::array<uint8_t, 16> bytes_) : bytes(bytes_) {}

        Uuid& operator=(std::array<uint8_t, 16> bytes_) {
            bytes = bytes_;
            return *this;
        }
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Uuid> {
        static constexpr ComponentDescriptor Descriptor = "rerun.datatypes.Uuid";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Uuid` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Uuid* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::FixedSizeListBuilder* builder, const datatypes::Uuid* elements,
            size_t num_elements
        );
    };
} // namespace rerun
