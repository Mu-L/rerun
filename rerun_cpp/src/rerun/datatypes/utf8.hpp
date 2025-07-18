// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/utf8.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <string>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class StringBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: A string of text, encoded as UTF-8.
    struct Utf8 {
        std::string value;

      public: // START of extensions from utf8_ext.cpp:
        /// Construct from a C string.
        Utf8(const char* utf8_) : value(utf8_) {}

        /// Explicit copy assignment from a C string to avoid ambiguity in some cases.
        Utf8& operator=(const char* utf8_) {
            value = utf8_;
            return *this;
        }

        /// Returns a pointer to the underlying C string.
        const char* c_str() const {
            return value.c_str();
        }

        // END of extensions from utf8_ext.cpp, start of generated code:

      public:
        Utf8() = default;

        Utf8(std::string value_) : value(std::move(value_)) {}

        Utf8& operator=(std::string value_) {
            value = std::move(value_);
            return *this;
        }
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Utf8> {
        static constexpr std::string_view ComponentType = "rerun.datatypes.Utf8";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Utf8` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Utf8* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StringBuilder* builder, const datatypes::Utf8* elements, size_t num_elements
        );
    };
} // namespace rerun
