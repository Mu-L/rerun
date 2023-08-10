# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)

__all__ = ["KeypointId", "KeypointIdArray", "KeypointIdArrayLike", "KeypointIdLike", "KeypointIdType"]


@define
class KeypointId:
    """
    A 16-bit ID representing a type of semantic keypoint within a class.

    `KeypointId`s are only meaningful within the context of a [`rerun.components.ClassDescription`][].

    Used to look up an [`rerun.components.AnnotationInfo`][] for a Keypoint within the
    [`rerun.components.AnnotationContext`].
    """

    id: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        return np.asarray(self.id, dtype=dtype)

    def __int__(self) -> int:
        return int(self.id)


if TYPE_CHECKING:
    KeypointIdLike = Union[KeypointId, int]
else:
    KeypointIdLike = Any

KeypointIdArrayLike = Union[
    KeypointId,
    Sequence[KeypointIdLike],
    int,
    npt.NDArray[np.uint8],
    npt.NDArray[np.uint16],
    npt.NDArray[np.uint32],
    npt.NDArray[np.uint64],
]


# --- Arrow support ---


class KeypointIdType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.uint16(), "rerun.datatypes.KeypointId")


class KeypointIdArray(BaseExtensionArray[KeypointIdArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.KeypointId"
    _EXTENSION_TYPE = KeypointIdType

    @staticmethod
    def _native_to_pa_array(data: KeypointIdArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError


KeypointIdType._ARRAY_TYPE = KeypointIdArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(KeypointIdType())
