// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/quaternion.fbs".

#include "quaternion.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<datatypes::Quaternion>::arrow_datatype() {
        static const auto datatype =
            arrow::fixed_size_list(arrow::field("item", arrow::float32(), false), 4);
        return datatype;
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<datatypes::Quaternion>::to_arrow(
        const datatypes::Quaternion* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<datatypes::Quaternion>::fill_arrow_array_builder(
                static_cast<arrow::FixedSizeListBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }

    rerun::Error Loggable<datatypes::Quaternion>::fill_arrow_array_builder(
        arrow::FixedSizeListBuilder* builder, const datatypes::Quaternion* elements,
        size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        auto value_builder = static_cast<arrow::FloatBuilder*>(builder->value_builder());

        ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements)));
        static_assert(sizeof(elements[0].xyzw) == sizeof(elements[0]));
        ARROW_RETURN_NOT_OK(value_builder->AppendValues(
            elements[0].xyzw.data(),
            static_cast<int64_t>(num_elements * 4),
            nullptr
        ));

        return Error::ok();
    }
} // namespace rerun
