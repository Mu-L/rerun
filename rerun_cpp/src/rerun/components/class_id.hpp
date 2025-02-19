// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/class_id.fbs".

#pragma once

#include "../component_descriptor.hpp"
#include "../datatypes/class_id.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: A 16-bit ID representing a type of semantic class.
    struct ClassId {
        rerun::datatypes::ClassId id;

      public:
        ClassId() = default;

        ClassId(rerun::datatypes::ClassId id_) : id(id_) {}

        ClassId& operator=(rerun::datatypes::ClassId id_) {
            id = id_;
            return *this;
        }

        ClassId(uint16_t id_) : id(id_) {}

        ClassId& operator=(uint16_t id_) {
            id = id_;
            return *this;
        }

        /// Cast to the underlying ClassId datatype
        operator rerun::datatypes::ClassId() const {
            return id;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::ClassId) == sizeof(components::ClassId));

    /// \private
    template <>
    struct Loggable<components::ClassId> {
        static constexpr ComponentDescriptor Descriptor = "rerun.components.ClassId";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::ClassId>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::ClassId` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::ClassId* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::ClassId>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::ClassId>::to_arrow(&instances->id, num_instances);
            }
        }
    };
} // namespace rerun
