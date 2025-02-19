# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/visible.fbs".

# You can extend this class by creating a "VisibleExt" class in "visible_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["Visible", "VisibleBatch"]


class Visible(datatypes.Bool, ComponentMixin):
    """**Component**: Whether the container, view, entity or instance is currently visible."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of VisibleExt in visible_ext.py

    # Note: there are no fields here because Visible delegates to datatypes.Bool
    pass


class VisibleBatch(datatypes.BoolBatch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.Visible")


# This is patched in late to avoid circular dependencies.
Visible._BATCH_TYPE = VisibleBatch  # type: ignore[assignment]
