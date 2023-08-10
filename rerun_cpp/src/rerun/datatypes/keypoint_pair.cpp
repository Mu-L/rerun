// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/keypoint_pair.fbs"

#include "keypoint_pair.hpp"

#include "../datatypes/keypoint_id.hpp"

#include <arrow/api.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType> &KeypointPair::to_arrow_datatype() {
            static const auto datatype = arrow::struct_({
                arrow::field("keypoint0", rerun::datatypes::KeypointId::to_arrow_datatype(), false),
                arrow::field("keypoint1", rerun::datatypes::KeypointId::to_arrow_datatype(), false),
            });
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::StructBuilder>> KeypointPair::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::StructBuilder>(
                to_arrow_datatype(),
                memory_pool,
                std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                    rerun::datatypes::KeypointId::new_arrow_array_builder(memory_pool).ValueOrDie(),
                    rerun::datatypes::KeypointId::new_arrow_array_builder(memory_pool).ValueOrDie(),
                })
            ));
        }

        arrow::Status KeypointPair::fill_arrow_array_builder(
            arrow::StructBuilder *builder, const KeypointPair *elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            {
                auto field_builder = static_cast<arrow::UInt16Builder *>(builder->field_builder(0));
                ARROW_RETURN_NOT_OK(field_builder->Reserve(num_elements));
                for (auto elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                    ARROW_RETURN_NOT_OK(rerun::datatypes::KeypointId::fill_arrow_array_builder(
                        field_builder,
                        &elements[elem_idx].keypoint0,
                        1
                    ));
                }
            }
            {
                auto field_builder = static_cast<arrow::UInt16Builder *>(builder->field_builder(1));
                ARROW_RETURN_NOT_OK(field_builder->Reserve(num_elements));
                for (auto elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                    ARROW_RETURN_NOT_OK(rerun::datatypes::KeypointId::fill_arrow_array_builder(
                        field_builder,
                        &elements[elem_idx].keypoint1,
                        1
                    ));
                }
            }
            ARROW_RETURN_NOT_OK(builder->AppendValues(num_elements, nullptr));

            return arrow::Status::OK();
        }
    } // namespace datatypes
} // namespace rerun
