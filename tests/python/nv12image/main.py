#!/usr/bin/env python3
"""Testing NV12 image encoding."""
from __future__ import annotations

import argparse
import os
from typing import Any

import cv2
import numpy as np
import rerun


def bgra2nv12(bgra: Any) -> np.ndarray:
    yuv = cv2.cvtColor(bgra, cv2.COLOR_BGRA2YUV_I420)
    uv_row_cnt = yuv.shape[0] // 3
    uv_plane = np.transpose(yuv[uv_row_cnt * 2 :].reshape(2, -1), [1, 0])
    yuv[uv_row_cnt * 2 :] = uv_plane.reshape(uv_row_cnt, -1)
    return yuv


def main() -> None:
    parser = argparse.ArgumentParser(description="Displaying NV12 encoded images.")
    rerun.script_add_args(parser)
    args = parser.parse_args()

    rerun.script_setup(args, "rerun_test_nv12image")

    # Make sure you use a colorful image!
    dir_path = os.path.dirname(os.path.realpath(__file__))
    img_path = f"{dir_path}/../../../crates/re_ui/data/logo_dark_mode.png"
    img_bgra = cv2.imread(img_path, cv2.IMREAD_UNCHANGED)

    img_rgb = cv2.cvtColor(img_bgra, cv2.COLOR_BGRA2RGB)
    rerun.log("img_reference", rerun.Image(img_rgb))

    rerun.log(
        "img_nv12",
        rerun.ImageEncoded(
            contents=bytes(bgra2nv12(img_bgra)),
            format=rerun.ImageFormat.NV12((img_bgra.shape[0], img_bgra.shape[1])),
        ),
    )

    rerun.script_teardown(args)


if __name__ == "__main__":
    main()
