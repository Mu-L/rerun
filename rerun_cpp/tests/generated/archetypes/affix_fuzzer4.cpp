// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#include "affix_fuzzer4.hpp"

#include <rerun/collection_adapter_builtins.hpp>

namespace rerun::archetypes {
    AffixFuzzer4 AffixFuzzer4::clear_fields() {
        auto archetype = AffixFuzzer4();
        archetype.fuzz2101 =
            ComponentBatch::empty<rerun::components::AffixFuzzer1>(Descriptor_fuzz2101)
                .value_or_throw();
        archetype.fuzz2102 =
            ComponentBatch::empty<rerun::components::AffixFuzzer2>(Descriptor_fuzz2102)
                .value_or_throw();
        archetype.fuzz2103 =
            ComponentBatch::empty<rerun::components::AffixFuzzer3>(Descriptor_fuzz2103)
                .value_or_throw();
        archetype.fuzz2104 =
            ComponentBatch::empty<rerun::components::AffixFuzzer4>(Descriptor_fuzz2104)
                .value_or_throw();
        archetype.fuzz2105 =
            ComponentBatch::empty<rerun::components::AffixFuzzer5>(Descriptor_fuzz2105)
                .value_or_throw();
        archetype.fuzz2106 =
            ComponentBatch::empty<rerun::components::AffixFuzzer6>(Descriptor_fuzz2106)
                .value_or_throw();
        archetype.fuzz2107 =
            ComponentBatch::empty<rerun::components::AffixFuzzer7>(Descriptor_fuzz2107)
                .value_or_throw();
        archetype.fuzz2108 =
            ComponentBatch::empty<rerun::components::AffixFuzzer8>(Descriptor_fuzz2108)
                .value_or_throw();
        archetype.fuzz2109 =
            ComponentBatch::empty<rerun::components::AffixFuzzer9>(Descriptor_fuzz2109)
                .value_or_throw();
        archetype.fuzz2110 =
            ComponentBatch::empty<rerun::components::AffixFuzzer10>(Descriptor_fuzz2110)
                .value_or_throw();
        archetype.fuzz2111 =
            ComponentBatch::empty<rerun::components::AffixFuzzer11>(Descriptor_fuzz2111)
                .value_or_throw();
        archetype.fuzz2112 =
            ComponentBatch::empty<rerun::components::AffixFuzzer12>(Descriptor_fuzz2112)
                .value_or_throw();
        archetype.fuzz2113 =
            ComponentBatch::empty<rerun::components::AffixFuzzer13>(Descriptor_fuzz2113)
                .value_or_throw();
        archetype.fuzz2114 =
            ComponentBatch::empty<rerun::components::AffixFuzzer14>(Descriptor_fuzz2114)
                .value_or_throw();
        archetype.fuzz2115 =
            ComponentBatch::empty<rerun::components::AffixFuzzer15>(Descriptor_fuzz2115)
                .value_or_throw();
        archetype.fuzz2116 =
            ComponentBatch::empty<rerun::components::AffixFuzzer16>(Descriptor_fuzz2116)
                .value_or_throw();
        archetype.fuzz2117 =
            ComponentBatch::empty<rerun::components::AffixFuzzer17>(Descriptor_fuzz2117)
                .value_or_throw();
        archetype.fuzz2118 =
            ComponentBatch::empty<rerun::components::AffixFuzzer18>(Descriptor_fuzz2118)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> AffixFuzzer4::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(18);
        if (fuzz2101.has_value()) {
            columns.push_back(fuzz2101.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2102.has_value()) {
            columns.push_back(fuzz2102.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2103.has_value()) {
            columns.push_back(fuzz2103.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2104.has_value()) {
            columns.push_back(fuzz2104.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2105.has_value()) {
            columns.push_back(fuzz2105.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2106.has_value()) {
            columns.push_back(fuzz2106.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2107.has_value()) {
            columns.push_back(fuzz2107.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2108.has_value()) {
            columns.push_back(fuzz2108.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2109.has_value()) {
            columns.push_back(fuzz2109.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2110.has_value()) {
            columns.push_back(fuzz2110.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2111.has_value()) {
            columns.push_back(fuzz2111.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2112.has_value()) {
            columns.push_back(fuzz2112.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2113.has_value()) {
            columns.push_back(fuzz2113.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2114.has_value()) {
            columns.push_back(fuzz2114.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2115.has_value()) {
            columns.push_back(fuzz2115.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2116.has_value()) {
            columns.push_back(fuzz2116.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2117.has_value()) {
            columns.push_back(fuzz2117.value().partitioned(lengths_).value_or_throw());
        }
        if (fuzz2118.has_value()) {
            columns.push_back(fuzz2118.value().partitioned(lengths_).value_or_throw());
        }
        return columns;
    }

    Collection<ComponentColumn> AffixFuzzer4::columns() {
        if (fuzz2101.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2101.value().length(), 1));
        }
        if (fuzz2102.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2102.value().length(), 1));
        }
        if (fuzz2103.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2103.value().length(), 1));
        }
        if (fuzz2104.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2104.value().length(), 1));
        }
        if (fuzz2105.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2105.value().length(), 1));
        }
        if (fuzz2106.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2106.value().length(), 1));
        }
        if (fuzz2107.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2107.value().length(), 1));
        }
        if (fuzz2108.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2108.value().length(), 1));
        }
        if (fuzz2109.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2109.value().length(), 1));
        }
        if (fuzz2110.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2110.value().length(), 1));
        }
        if (fuzz2111.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2111.value().length(), 1));
        }
        if (fuzz2112.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2112.value().length(), 1));
        }
        if (fuzz2113.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2113.value().length(), 1));
        }
        if (fuzz2114.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2114.value().length(), 1));
        }
        if (fuzz2115.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2115.value().length(), 1));
        }
        if (fuzz2116.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2116.value().length(), 1));
        }
        if (fuzz2117.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2117.value().length(), 1));
        }
        if (fuzz2118.has_value()) {
            return columns(std::vector<uint32_t>(fuzz2118.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>> AsComponents<archetypes::AffixFuzzer4>::as_batches(
        const archetypes::AffixFuzzer4& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(18);

        if (archetype.fuzz2101.has_value()) {
            cells.push_back(archetype.fuzz2101.value());
        }
        if (archetype.fuzz2102.has_value()) {
            cells.push_back(archetype.fuzz2102.value());
        }
        if (archetype.fuzz2103.has_value()) {
            cells.push_back(archetype.fuzz2103.value());
        }
        if (archetype.fuzz2104.has_value()) {
            cells.push_back(archetype.fuzz2104.value());
        }
        if (archetype.fuzz2105.has_value()) {
            cells.push_back(archetype.fuzz2105.value());
        }
        if (archetype.fuzz2106.has_value()) {
            cells.push_back(archetype.fuzz2106.value());
        }
        if (archetype.fuzz2107.has_value()) {
            cells.push_back(archetype.fuzz2107.value());
        }
        if (archetype.fuzz2108.has_value()) {
            cells.push_back(archetype.fuzz2108.value());
        }
        if (archetype.fuzz2109.has_value()) {
            cells.push_back(archetype.fuzz2109.value());
        }
        if (archetype.fuzz2110.has_value()) {
            cells.push_back(archetype.fuzz2110.value());
        }
        if (archetype.fuzz2111.has_value()) {
            cells.push_back(archetype.fuzz2111.value());
        }
        if (archetype.fuzz2112.has_value()) {
            cells.push_back(archetype.fuzz2112.value());
        }
        if (archetype.fuzz2113.has_value()) {
            cells.push_back(archetype.fuzz2113.value());
        }
        if (archetype.fuzz2114.has_value()) {
            cells.push_back(archetype.fuzz2114.value());
        }
        if (archetype.fuzz2115.has_value()) {
            cells.push_back(archetype.fuzz2115.value());
        }
        if (archetype.fuzz2116.has_value()) {
            cells.push_back(archetype.fuzz2116.value());
        }
        if (archetype.fuzz2117.has_value()) {
            cells.push_back(archetype.fuzz2117.value());
        }
        if (archetype.fuzz2118.has_value()) {
            cells.push_back(archetype.fuzz2118.value());
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
