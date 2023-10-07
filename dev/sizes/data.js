window.BENCHMARK_DATA = {
  "lastUpdate": 1696680751715,
  "repoUrl": "https://github.com/rerun-io/rerun",
  "entries": {
    "Sizes": [
      {
        "commit": {
          "author": {
            "email": "jeremy@rerun.io",
            "name": "Jeremy Leibs",
            "username": "jleibs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5285fd363d2788896f9d46344c437e47765ff74f",
          "message": "Connect to 127.0.0.1 when checking for existing viewer (#3696)\n\n### What\r\n0.0.0.0 isn't valid on some hosts.\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] I have tested [demo.rerun.io](https://demo.rerun.io/pr/3696) (if\r\napplicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3696)\r\n- [Docs\r\npreview](https://rerun.io/preview/42af3a55fb4e9e5dcc8c4d1f74a983cf1fdbc648/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/42af3a55fb4e9e5dcc8c4d1f74a983cf1fdbc648/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-05T16:41:51+02:00",
          "tree_id": "3478ee5ba7fdc1c1dadeb34211025ab63894cdd0",
          "url": "https://github.com/rerun-io/rerun/commit/5285fd363d2788896f9d46344c437e47765ff74f"
        },
        "date": 1696518552377,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.37",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "132550499+rerun-bot@users.noreply.github.com",
            "name": "rerun-bot",
            "username": "rerun-bot"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "301351a665f2ef0cfae066897338939e3ed0fe15",
          "message": "Release 0.9.0 (#3688)\n\n# Release 0.9.0\r\n\r\n### Next steps\r\n- [Test the release](#testing)\r\n- If this is an `alpha` release, you can just merge the pull request.\r\n- Otherwise:\r\n  - For any added commits, run the release workflow in `rc` mode again\r\n  - After testing, run the release workflow in `release` mode\r\n- Once the final release workflow finishes, [create a GitHub\r\nrelease](https://github.com/rerun-io/rerun/releases/new)\r\n\r\n### Testing\r\n- [x] Docs\r\n- NOTE: wait for docs deploy + [`docs.rs`\r\nbuild](https://docs.rs/releases/queue)\r\n  - [x] [ref.rerun.io](https://ref.rerun.io/docs/rust/dev/rerun/) (Rust)\r\n- [x] [ref.rerun.io](https://ref.rerun.io/docs/python/dev/common/)\r\n(Python)\r\n- [x] [rerun.io/docs](https://rerun.io/preview/pr%3Arelease-0.9.0/docs)\r\n- [x]\r\n[rerun.io/examples](https://rerun.io/preview/pr%3Arelease-0.9.0/examples)\r\n  - [x] [docs.rs](https://docs.rs/rerun/0.9.0-rc.3/rerun/)\r\n- Web\r\n  - NOTE: wait for these to build and deploy\r\n  - [x] [demo.rerun.io](https://demo.rerun.io/version/0.9.0-rc.3)\r\n  - [x] [app.rerun.io](https://app.rerun.io/version/0.9.0-rc.3)\r\n- [x] Windows\r\n  - [x] Python Wheel\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust crate\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust install\r\n    - [x] Web\r\n    - [x] Native\r\n- [x] Linux\r\n  - [x] Python Wheel\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust crate\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust install\r\n    - [x] Web\r\n    - [x] Native\r\n- [x] Mac\r\n  - [x] Python Wheel\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust crate\r\n    - [x] Web\r\n    - [x] Native\r\n  - [x] Rust install\r\n    - [x] Web\r\n    - [x] Native\r\n\r\n---------\r\n\r\nCo-authored-by: jprochazk <1665677+jprochazk@users.noreply.github.com>\r\nCo-authored-by: Emil Ernerfeldt <emil.ernerfeldt@gmail.com>\r\nCo-authored-by: Andreas Reich <r_andreas2@web.de>",
          "timestamp": "2023-10-05T18:43:13+02:00",
          "tree_id": "3200bbab229dfc941bbeb7282e356ca6c5bc2e20",
          "url": "https://github.com/rerun-io/rerun/commit/301351a665f2ef0cfae066897338939e3ed0fe15"
        },
        "date": 1696526610193,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.35",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "andreas@rerun.io",
            "name": "Andreas Reich",
            "username": "Wumpf"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a9c08b56b6bb798b4aefade7c39900bb1add80c8",
          "message": "Fix strange input box in DNA demo caused by trailing backticks (#3702)\n\n<!--\r\nOpen the PR up as a draft until you feel it is ready for a proper\r\nreview.\r\n\r\nDo not make PR:s from your own `main` branch, as that makes it difficult\r\nfor reviewers to add their own fixes.\r\n\r\nAdd any improvements to the branch as new commits to make it easier for\r\nreviewers to follow the progress. All commits will be squashed to a\r\nsingle commit once the PR is merged into `main`.\r\n\r\nMake sure you mention any issues that this PR closes in the description,\r\nas well as any other related issues.\r\n\r\nTo get an auto-generated PR description you can put \"copilot:summary\" or\r\n\"copilot:walkthrough\" anywhere.\r\n-->\r\n\r\n### What\r\n\r\nWell, this was unexpected!\r\n\r\n![image](https://github.com/rerun-io/rerun/assets/1220815/a61f538d-f389-4200-aedc-f05fd27a28fa)\r\n\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] I have tested [demo.rerun.io](https://demo.rerun.io/pr/3702) (if\r\napplicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3702)\r\n- [Docs\r\npreview](https://rerun.io/preview/21581126a3d4ce84b4bd4717cfaf0befddc4dd31/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/21581126a3d4ce84b4bd4717cfaf0befddc4dd31/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-05T19:21:42+02:00",
          "tree_id": "08ede61dcf1d68ba11765c67eb673639434d4294",
          "url": "https://github.com/rerun-io/rerun/commit/a9c08b56b6bb798b4aefade7c39900bb1add80c8"
        },
        "date": 1696528259133,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.35",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeremy@rerun.io",
            "name": "Jeremy Leibs",
            "username": "jleibs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6981ac9762b3f9e46e1c7c5773a5436454693eb1",
          "message": "Fix link to AsComponents (#3704)\n\n### What\r\nPreview:\r\nhttps://www.rerun.io/preview/719ade86b846a390e7411ab4385ea1912684c75f/docs/howto/extend/custom-data\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] I have tested [demo.rerun.io](https://demo.rerun.io/pr/3704) (if\r\napplicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3704)\r\n- [Docs\r\npreview](https://rerun.io/preview/719ade86b846a390e7411ab4385ea1912684c75f/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/719ade86b846a390e7411ab4385ea1912684c75f/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-05T19:41:02+02:00",
          "tree_id": "a588f119113cc1f98c445fe9fe6e4547a437b783",
          "url": "https://github.com/rerun-io/rerun/commit/6981ac9762b3f9e46e1c7c5773a5436454693eb1"
        },
        "date": 1696529498222,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.35",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "emil.ernerfeldt@gmail.com",
            "name": "Emil Ernerfeldt",
            "username": "emilk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "96c8988236509578d3109ca52f691f68a952c7e9",
          "message": "Fix a bunch of lints found by Rust 1.73 (#3714)\n\nFix a bunch of lints found by Rust 1.73\r\n\r\nI also enabled the new `clippy::needless_pass_by_ref_mut` lint, which I\r\nalready love.\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] I have tested [demo.rerun.io](https://demo.rerun.io/pr/3714) (if\r\napplicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3714)\r\n- [Docs\r\npreview](https://rerun.io/preview/e5ecd5c70154e2e7cd593d24bb48faed14b9e4e5/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/e5ecd5c70154e2e7cd593d24bb48faed14b9e4e5/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-06T09:45:13+02:00",
          "tree_id": "4db90ee1bec424f1cb5dbe4076ec242777355853",
          "url": "https://github.com/rerun-io/rerun/commit/96c8988236509578d3109ca52f691f68a952c7e9"
        },
        "date": 1696580314044,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.36",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeremy@rerun.io",
            "name": "Jeremy Leibs",
            "username": "jleibs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ff37fe99e4c739c87d9a0ddbc749e1da6d331703",
          "message": "Fix broken link to migration guide (#3721)\n\n### What\r\n\r\n### Checklist\r\n* [ ] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [ ] I've included a screenshot or gif (if applicable)\r\n* [ ] I have tested [demo.rerun.io](https://demo.rerun.io/pr/{{\r\npr.number }}) (if applicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/{{ pr.number }})\r\n- [Docs preview](https://rerun.io/preview/{{ pr.commit }}/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples preview](https://rerun.io/preview/{{ pr.commit }}/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-06T15:13:03+02:00",
          "tree_id": "14c440c92dfd5f120668c2da2c871d11e21e0fef",
          "url": "https://github.com/rerun-io/rerun/commit/ff37fe99e4c739c87d9a0ddbc749e1da6d331703"
        },
        "date": 1696599822745,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.36",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.35",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49431240+abey79@users.noreply.github.com",
            "name": "Antoine Beyeler",
            "username": "abey79"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2f4e78dd6ecabf8238d2d6549db4322027c74772",
          "message": "Fixed default stroke width handling in `log_line_strip_Xd` and `log_obbs` (#3720)\n\n### What\r\n\r\nFixes:\r\n- #3711 \r\n\r\nTested with:\r\n\r\n```python\r\nimport rerun as rr\r\n\r\nrr.init(\"rerun_example_line_strip3d\", spawn=True)\r\n\r\npoints_3d = [\r\n    [0, 0, 0],\r\n    [0, 0, 1],\r\n    [1, 0, 0],\r\n    [1, 0, 1],\r\n    [1, 1, 0],\r\n    [1, 1, 1],\r\n    [0, 1, 0],\r\n    [0, 1, 1],\r\n]\r\n\r\npoints_2d = [\r\n    [\r\n        [0, 0],\r\n        [1, 0],\r\n        [1, 1],\r\n        [0, 1],\r\n    ],\r\n    [\r\n        [2, 2],\r\n        [3, 3],\r\n        [3, 4],\r\n        [5, 2],\r\n    ],\r\n]\r\n\r\nrr.log_line_strips_3d(\"strip\", [points_3d])\r\nrr.log_line_strips_2d(\"strip2\", points_2d)\r\nrr.log_obbs(\"obb\", positions=[[0, 0, 0], [10, 10, 10]], half_sizes=[[1, 1, 1], [2, 2, 2]])\r\n\r\nrr.log_transform3d(\"with_width\", rr.Translation3D([3, 3, 3]))\r\nrr.log_line_strips_3d(\"with_width/strip\", [points_3d], stroke_widths=0.2)\r\nrr.log_line_strips_2d(\"with_width/strip2\", points_2d, stroke_widths=[0.2, 0.3])\r\nrr.log_obbs(\"with_width/obb\", positions=[[0, 0, 0], [10, 10, 10]], half_sizes=[[1, 1, 1], [2, 2, 2]], stroke_widths=[0.2, 0.3])\r\n```\r\n\r\n<img width=\"1137\" alt=\"image\"\r\nsrc=\"https://github.com/rerun-io/rerun/assets/49431240/44190351-bde5-4103-b2f5-e7f8c51444da\">\r\n\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] ~~I have tested [demo.rerun.io](https://demo.rerun.io/pr/3720) (if\r\napplicable)~~\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3720)\r\n- [Docs\r\npreview](https://rerun.io/preview/d4a3314f43e1e62547478362cc04b157a34b462c/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/d4a3314f43e1e62547478362cc04b157a34b462c/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-06T16:27:06+02:00",
          "tree_id": "1fc5f2866885c26f52723b6f4d7a8ee9ceb07054",
          "url": "https://github.com/rerun-io/rerun/commit/2f4e78dd6ecabf8238d2d6549db4322027c74772"
        },
        "date": 1696604214178,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.36",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeremy@rerun.io",
            "name": "Jeremy Leibs",
            "username": "jleibs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "25f157f90770a8858785fea5c439b750742317fb",
          "message": "Warn/raise when passing incompatible objects to log (#3727)\n\n### What\r\n- Resolves: https://github.com/rerun-io/rerun/issues/3710\r\n\r\nBefore:\r\n```\r\ntransform3d_simple.py:11: RerunWarning: log: TypeError('TranslationAndMat3x3' object is not iterable)\r\n  rr.log(\"base/translated\", rr.TranslationAndMat3x3(translation=[1, 0, 0]))\r\n```\r\n\r\nAfter:\r\n```\r\ntransform3d_simple.py:11: RerunWarning: log: TypeError(Expected an object implementing rerun.AsComponents or an iterable of rerun.ComponentBatchLike, but got <class 'rerun.datatypes.translation_and_mat3x3.TranslationAndMat3x3'> instead.)\r\n  rr.log(\"base/translated\", rr.TranslationAndMat3x3(translation=[1, 0, 0]))\r\n```\r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] I've included a screenshot or gif (if applicable)\r\n* [x] I have tested [demo.rerun.io](https://demo.rerun.io/pr/3727) (if\r\napplicable)\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3727)\r\n- [Docs\r\npreview](https://rerun.io/preview/379b347a96593ecf6a7efc5bf7b98d3e248e5489/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/379b347a96593ecf6a7efc5bf7b98d3e248e5489/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-07T13:38:48+02:00",
          "tree_id": "f7100a8c27199f326428502293195bb55d72e5bc",
          "url": "https://github.com/rerun-io/rerun/commit/25f157f90770a8858785fea5c439b750742317fb"
        },
        "date": 1696680736844,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Wasm",
            "value": "15.36",
            "unit": "MiB"
          },
          {
            "name": "JS",
            "value": "113.04",
            "unit": "kiB"
          },
          {
            "name": "arkit_scenes.rrd",
            "value": "40.37",
            "unit": "MiB"
          },
          {
            "name": "detect_and_track_objects.rrd",
            "value": "55.19",
            "unit": "MiB"
          },
          {
            "name": "dicom_mri.rrd",
            "value": "63.36",
            "unit": "MiB"
          },
          {
            "name": "dna.rrd",
            "value": "0.62",
            "unit": "MiB"
          },
          {
            "name": "human_pose_tracking.rrd",
            "value": "54.41",
            "unit": "MiB"
          },
          {
            "name": "plots.rrd",
            "value": "0.19",
            "unit": "MiB"
          },
          {
            "name": "structure_from_motion.rrd",
            "value": "6.85",
            "unit": "MiB"
          }
        ]
      }
    ]
  }
}