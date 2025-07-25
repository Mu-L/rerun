# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer21Ext" class in "affix_fuzzer21_ext.py".

from __future__ import annotations

from rerun._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

from .. import datatypes

__all__ = ["AffixFuzzer21", "AffixFuzzer21Batch"]


class AffixFuzzer21(datatypes.AffixFuzzer21, ComponentMixin):
    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of AffixFuzzer21Ext in affix_fuzzer21_ext.py

    # Note: there are no fields here because AffixFuzzer21 delegates to datatypes.AffixFuzzer21


class AffixFuzzer21Batch(datatypes.AffixFuzzer21Batch, ComponentBatchMixin):
    _COMPONENT_TYPE: str = "rerun.testing.components.AffixFuzzer21"


# This is patched in late to avoid circular dependencies.
AffixFuzzer21._BATCH_TYPE = AffixFuzzer21Batch  # type: ignore[assignment]
