// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/half_sizes2d.fbs".

#include "half_sizes2d.hpp"

#include "../datatypes/vec2d.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char HalfSizes2D::NAME[] = "rerun.components.HalfSizes2D";

        const std::shared_ptr<arrow::DataType>& HalfSizes2D::arrow_datatype() {
            static const auto datatype = rerun::datatypes::Vec2D::arrow_datatype();
            return datatype;
        }

        Result<std::shared_ptr<arrow::FixedSizeListBuilder>> HalfSizes2D::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(rerun::datatypes::Vec2D::new_arrow_array_builder(memory_pool).value);
        }

        Error HalfSizes2D::fill_arrow_array_builder(
            arrow::FixedSizeListBuilder* builder, const HalfSizes2D* elements, size_t num_elements
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

            static_assert(sizeof(rerun::datatypes::Vec2D) == sizeof(HalfSizes2D));
            RR_RETURN_NOT_OK(rerun::datatypes::Vec2D::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rerun::datatypes::Vec2D*>(elements),
                num_elements
            ));

            return Error::ok();
        }

        Result<rerun::DataCell> HalfSizes2D::to_data_cell(
            const HalfSizes2D* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            auto builder_result = HalfSizes2D::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(
                    HalfSizes2D::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            return rerun::DataCell::create(
                HalfSizes2D::NAME,
                HalfSizes2D::arrow_datatype(),
                std::move(array)
            );
        }
    } // namespace components
} // namespace rerun
