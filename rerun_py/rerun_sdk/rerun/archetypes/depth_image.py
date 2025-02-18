# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

# You can extend this class by creating a "DepthImageExt" class in "depth_image_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components, datatypes
from .._baseclasses import Archetype
from ..error_utils import catch_and_log_exceptions
from .depth_image_ext import DepthImageExt

__all__ = ["DepthImage"]


@define(str=False, repr=False, init=False)
class DepthImage(DepthImageExt, Archetype):
    """
    **Archetype**: A depth image.

    The shape of the `TensorData` must be mappable to an `HxW` tensor.
    Each pixel corresponds to a depth value in units specified by `meter`.

    Example
    -------
    ### Depth to 3D example:
    ```python
    import numpy as np
    import rerun as rr

    depth_image = 65535 * np.ones((8, 12), dtype=np.uint16)
    depth_image[0:4, 0:6] = 20000
    depth_image[4:8, 6:12] = 45000

    rr.init("rerun_example_depth_image", spawn=True)

    # If we log a pinhole camera model, the depth gets automatically back-projected to 3D
    rr.log(
        "world/camera",
        rr.Pinhole(
            width=depth_image.shape[1],
            height=depth_image.shape[0],
            focal_length=20,
        ),
    )

    # Log the tensor.
    rr.log("world/camera/depth", rr.DepthImage(depth_image, meter=10_000.0))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/1200w.png">
      <img src="https://static.rerun.io/depth_image_3d/f78674bdae0eb25786c6173307693c5338f38b87/full.png" width="640">
    </picture>
    </center>
    """

    def __init__(
        self: Any,
        data: datatypes.TensorDataLike,
        *,
        meter: components.DepthMeterLike | None = None,
        draw_order: components.DrawOrderLike | None = None,
    ):
        """
        Create a new instance of the DepthImage archetype.

        Parameters
        ----------
        data:
            The depth-image data. Should always be a rank-2 tensor.
        meter:
            An optional floating point value that specifies how long a meter is in the native depth units.

            For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
            and a range of up to ~65 meters (2^16 / 1000).
        draw_order:
            An optional floating point value that specifies the 2D drawing order.

            Objects with higher values are drawn on top of those with lower values.
        """

        # You can define your own __init__ function as a member of DepthImageExt in depth_image_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(data=data, meter=meter, draw_order=draw_order)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            data=None,  # type: ignore[arg-type]
            meter=None,  # type: ignore[arg-type]
            draw_order=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> DepthImage:
        """Produce an empty DepthImage, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    data: components.TensorDataBatch = field(
        metadata={"component": "required"},
        converter=DepthImageExt.data__field_converter_override,  # type: ignore[misc]
    )
    # The depth-image data. Should always be a rank-2 tensor.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    meter: components.DepthMeterBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DepthMeterBatch._optional,  # type: ignore[misc]
    )
    # An optional floating point value that specifies how long a meter is in the native depth units.
    #
    # For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    # and a range of up to ~65 meters (2^16 / 1000).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    draw_order: components.DrawOrderBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DrawOrderBatch._optional,  # type: ignore[misc]
    )
    # An optional floating point value that specifies the 2D drawing order.
    #
    # Objects with higher values are drawn on top of those with lower values.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
