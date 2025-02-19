# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/marker_shape.fbs".

# You can extend this class by creating a "MarkerShapeExt" class in "marker_shape_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    ComponentBatchMixin,
    ComponentDescriptor,
)

__all__ = ["MarkerShape", "MarkerShapeArrayLike", "MarkerShapeBatch", "MarkerShapeLike"]


from enum import Enum


class MarkerShape(Enum):
    """**Component**: The visual appearance of a point in e.g. a 2D plot."""

    Circle = 1
    """`⏺`"""

    Diamond = 2
    """`◆`"""

    Square = 3
    """`◼️`"""

    Cross = 4
    """`x`"""

    Plus = 5
    """`+`"""

    Up = 6
    """`▲`"""

    Down = 7
    """`▼`"""

    Left = 8
    """`◀`"""

    Right = 9
    """`▶`"""

    Asterisk = 10
    """`*`"""

    @classmethod
    def auto(cls, val: str | int | MarkerShape) -> MarkerShape:
        """Best-effort converter, including a case-insensitive string matcher."""
        if isinstance(val, MarkerShape):
            return val
        if isinstance(val, int):
            return cls(val)
        try:
            return cls[val]
        except KeyError:
            val_lower = val.lower()
            for variant in cls:
                if variant.name.lower() == val_lower:
                    return variant
        raise ValueError(f"Cannot convert {val} to {cls.__name__}")

    def __str__(self) -> str:
        """Returns the variant name."""
        return self.name


MarkerShapeLike = Union[
    MarkerShape,
    Literal[
        "Asterisk",
        "Circle",
        "Cross",
        "Diamond",
        "Down",
        "Left",
        "Plus",
        "Right",
        "Square",
        "Up",
        "asterisk",
        "circle",
        "cross",
        "diamond",
        "down",
        "left",
        "plus",
        "right",
        "square",
        "up",
    ],
    int,
]
MarkerShapeArrayLike = Union[MarkerShapeLike, Sequence[MarkerShapeLike]]


class MarkerShapeBatch(BaseBatch[MarkerShapeArrayLike], ComponentBatchMixin):
    _ARROW_DATATYPE = pa.uint8()
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.MarkerShape")

    @staticmethod
    def _native_to_pa_array(data: MarkerShapeArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (MarkerShape, int, str)):
            data = [data]

        pa_data = [MarkerShape.auto(v).value if v is not None else None for v in data]  # type: ignore[redundant-expr]

        return pa.array(pa_data, type=data_type)
