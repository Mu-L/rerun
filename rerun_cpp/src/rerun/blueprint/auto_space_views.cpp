// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/auto_space_views.fbs".

#include "auto_space_views.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace blueprint {
        const std::shared_ptr<arrow::DataType>& AutoSpaceViews::arrow_datatype() {
            static const auto datatype = arrow::boolean();
            return datatype;
        }

        Result<std::shared_ptr<arrow::BooleanBuilder>> AutoSpaceViews::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::BooleanBuilder>(memory_pool));
        }

        Error AutoSpaceViews::fill_arrow_array_builder(
            arrow::BooleanBuilder* builder, const AutoSpaceViews* elements, size_t num_elements
        ) {
            if (builder == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (elements == nullptr) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            static_assert(sizeof(*elements) == sizeof(elements->enabled));
            ARROW_RETURN_NOT_OK(builder->AppendValues(
                reinterpret_cast<const uint8_t*>(&elements->enabled),
                static_cast<int64_t>(num_elements)
            ));

            return Error::ok();
        }
    } // namespace blueprint
} // namespace rerun
