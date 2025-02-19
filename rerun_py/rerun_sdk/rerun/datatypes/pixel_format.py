# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/pixel_format.fbs".

# You can extend this class by creating a "PixelFormatExt" class in "pixel_format_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
)

__all__ = ["PixelFormat", "PixelFormatArrayLike", "PixelFormatBatch", "PixelFormatLike"]


from enum import Enum


class PixelFormat(Enum):
    """
    **Datatype**: Specifieds a particular format of an [`archetypes.Image`][rerun.archetypes.Image].

    Most images can be described by a [`datatypes.ColorModel`][rerun.datatypes.ColorModel] and a [`datatypes.ChannelDatatype`][rerun.datatypes.ChannelDatatype],
    e.g. `RGB` and `U8` respectively.

    However, some image formats has chroma downsampling and/or
    use differing number of bits per channel, and that is what this [`datatypes.PixelFormat`][rerun.datatypes.PixelFormat] is for.

    All these formats support random access.

    For more compressed image formats, see [`archetypes.EncodedImage`][rerun.archetypes.EncodedImage].
    """

    Y_U_V12_LimitedRange = 20
    """
    `Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.

    This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].

    First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    the resolution of the Y plane.
    """

    NV12 = 26
    """
    `NV12` (aka `Y_UV12`) is a YUV 4:2:0 chroma downsampled form at with 12 bits per pixel and 8 bits per channel.

    This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].

    First comes entire image in Y in one plane,
    followed by a plane with interleaved lines ordered as U0, V0, U1, V1, etc.
    """

    YUY2 = 27
    """
    `YUY2` (aka 'YUYV', 'YUYV16' or 'NV21'), is a YUV 4:2:2 chroma downsampled format with 16 bits per pixel and 8 bits per channel.

    This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].

    The order of the channels is Y0, U0, Y1, V0, all in the same plane.
    """

    Y8_FullRange = 30
    """
    Monochrome Y plane only, essentially a YUV 4:0:0 planar format.

    Also known as just "gray". This is virtually identical to a 8bit luminance/grayscale (see [`datatypes.ColorModel`][rerun.datatypes.ColorModel]).

    This uses entire range YUV, i.e. Y is expected to be within [0, 255].
    (as opposed to "limited range" YUV as used e.g. in NV12).
    """

    Y_U_V24_LimitedRange = 39
    """
    `Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.

    This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].

    First comes entire image in Y in one plane, followed by the U and V planes.
    """

    Y_U_V24_FullRange = 40
    """
    `Y_U_V24` is a YUV 4:4:4 fully planar YUV format without chroma downsampling, also known as `I444`.

    This uses full range YUV with all components ranging from 0 to 255
    (as opposed to "limited range" YUV as used e.g. in NV12).

    First comes entire image in Y in one plane, followed by the U and V planes.
    """

    Y8_LimitedRange = 41
    """
    Monochrome Y plane only, essentially a YUV 4:0:0 planar format.

    Also known as just "gray".

    This uses limited range YUV, i.e. Y is expected to be within [16, 235].
    If not for this range limitation/remapping, this is almost identical to 8bit luminace/grayscale (see [`datatypes.ColorModel`][rerun.datatypes.ColorModel]).
    """

    Y_U_V12_FullRange = 44
    """
    `Y_U_V12` is a YUV 4:2:0 fully planar YUV format without chroma downsampling, also known as `I420`.

    This uses full range YUV with all components ranging from 0 to 255
    (as opposed to "limited range" YUV as used e.g. in NV12).

    First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    the resolution of the Y plane.
    """

    Y_U_V16_LimitedRange = 49
    """
    `Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.

    This uses limited range YUV, i.e. Y is expected to be within [16, 235] and U/V within [16, 240].

    First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    the horizontal resolution of the Y plane.
    """

    Y_U_V16_FullRange = 50
    """
    `Y_U_V16` is a YUV 4:2:2 fully planar YUV format without chroma downsampling, also known as `I422`.

    This uses full range YUV with all components ranging from 0 to 255
    (as opposed to "limited range" YUV as used e.g. in NV12).

    First comes entire image in Y in one plane, followed by the U and V planes, which each only have half
    the horizontal resolution of the Y plane.
    """

    @classmethod
    def auto(cls, val: str | int | PixelFormat) -> PixelFormat:
        """Best-effort converter, including a case-insensitive string matcher."""
        if isinstance(val, PixelFormat):
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


PixelFormatLike = Union[
    PixelFormat,
    Literal[
        "NV12",
        "Y8_FullRange",
        "Y8_LimitedRange",
        "YUY2",
        "Y_U_V12_FullRange",
        "Y_U_V12_LimitedRange",
        "Y_U_V16_FullRange",
        "Y_U_V16_LimitedRange",
        "Y_U_V24_FullRange",
        "Y_U_V24_LimitedRange",
        "nv12",
        "y8_fullrange",
        "y8_limitedrange",
        "y_u_v12_fullrange",
        "y_u_v12_limitedrange",
        "y_u_v16_fullrange",
        "y_u_v16_limitedrange",
        "y_u_v24_fullrange",
        "y_u_v24_limitedrange",
        "yuy2",
    ],
    int,
]
PixelFormatArrayLike = Union[PixelFormatLike, Sequence[PixelFormatLike]]


class PixelFormatBatch(BaseBatch[PixelFormatArrayLike]):
    _ARROW_DATATYPE = pa.uint8()

    @staticmethod
    def _native_to_pa_array(data: PixelFormatArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (PixelFormat, int, str)):
            data = [data]

        pa_data = [PixelFormat.auto(v).value if v is not None else None for v in data]  # type: ignore[redundant-expr]

        return pa.array(pa_data, type=data_type)
