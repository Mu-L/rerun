window.BENCHMARK_DATA = {
  "lastUpdate": 1696681720937,
  "repoUrl": "https://github.com/rerun-io/rerun",
  "entries": {
    "Rust Benchmark": [
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
        "date": 1696517690254,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 4118040,
            "range": "± 349893",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 395,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 301,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 427,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3882283,
            "range": "± 244495",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1856687,
            "range": "± 31237",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 30295000,
            "range": "± 897155",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 146155622,
            "range": "± 1329295",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 174659948,
            "range": "± 1199281",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 350005144,
            "range": "± 4678981",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 210669656,
            "range": "± 1631633",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 69571154,
            "range": "± 1050441",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 285546895,
            "range": "± 2994200",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 21186649,
            "range": "± 1673208",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3624448,
            "range": "± 219791",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 489766,
            "range": "± 2377",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 24517936,
            "range": "± 1819575",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 440778,
            "range": "± 3995",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 8865565,
            "range": "± 632147",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 9325942,
            "range": "± 645163",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 141379,
            "range": "± 393",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5535,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 100262,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 261912,
            "range": "± 2667",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 38559,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2236,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 41137,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1833045849,
            "range": "± 14111821",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1260252,
            "range": "± 23258",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1862860336,
            "range": "± 18160729",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1354190,
            "range": "± 11304",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1193701,
            "range": "± 17061",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3640,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1198776,
            "range": "± 2359",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30935,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696524995256,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3112653,
            "range": "± 15627",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 406,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 297,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 455,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3165977,
            "range": "± 12622",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1887517,
            "range": "± 7991",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 20339707,
            "range": "± 1383288",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 139448352,
            "range": "± 2139741",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 166215180,
            "range": "± 1193554",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 325298961,
            "range": "± 4166575",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 201747861,
            "range": "± 1640273",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 60905263,
            "range": "± 2065663",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 270912815,
            "range": "± 3258137",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14885505,
            "range": "± 177149",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3297863,
            "range": "± 17024",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 507234,
            "range": "± 2802",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19481142,
            "range": "± 374840",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 430256,
            "range": "± 1193",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7817942,
            "range": "± 34412",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8272200,
            "range": "± 38731",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 143443,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5579,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89129,
            "range": "± 2739",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 247812,
            "range": "± 1513",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 37143,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2265,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 40848,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1838152258,
            "range": "± 4370240",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1273693,
            "range": "± 21656",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1871165900,
            "range": "± 4394489",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1342283,
            "range": "± 4952",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1251513,
            "range": "± 14819",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3643,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1258936,
            "range": "± 3744",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30743,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696527224797,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3091003,
            "range": "± 4391",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 387,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 297,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 428,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3159655,
            "range": "± 9513",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1882562,
            "range": "± 9279",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 19475195,
            "range": "± 727973",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 137947362,
            "range": "± 1174876",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 173773177,
            "range": "± 1072960",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 342646254,
            "range": "± 3587666",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 203392889,
            "range": "± 1258093",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 60765009,
            "range": "± 1648322",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 273812098,
            "range": "± 2670411",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14586324,
            "range": "± 412723",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3214293,
            "range": "± 10912",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 534249,
            "range": "± 14115",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19052242,
            "range": "± 226433",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 415366,
            "range": "± 928",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7668421,
            "range": "± 15259",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8120804,
            "range": "± 21767",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 153246,
            "range": "± 561",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5646,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89720,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 247760,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 38253,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2330,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 40966,
            "range": "± 417",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1799061762,
            "range": "± 8234660",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1274856,
            "range": "± 15468",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1822870484,
            "range": "± 4958306",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1363035,
            "range": "± 118668",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1184689,
            "range": "± 5986",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3636,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1195432,
            "range": "± 16515",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 29425,
            "range": "± 425",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696528372328,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 2983119,
            "range": "± 78573",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 362,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 285,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 400,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 2941055,
            "range": "± 81073",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1756888,
            "range": "± 48812",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 18422648,
            "range": "± 807682",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 128272952,
            "range": "± 1878743",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 162227435,
            "range": "± 2542666",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 317927049,
            "range": "± 4007821",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 193046729,
            "range": "± 2991932",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 57644008,
            "range": "± 1927100",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 257992434,
            "range": "± 4110261",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14070848,
            "range": "± 337295",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3041910,
            "range": "± 61135",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 485523,
            "range": "± 11618",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 18344244,
            "range": "± 439946",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 413448,
            "range": "± 9401",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7494601,
            "range": "± 172179",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 7710617,
            "range": "± 233977",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 137932,
            "range": "± 3226",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5257,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 84722,
            "range": "± 1974",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 233834,
            "range": "± 5840",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 34945,
            "range": "± 783",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2092,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 39759,
            "range": "± 1089",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1785864853,
            "range": "± 3906160",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1181241,
            "range": "± 29501",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1776657971,
            "range": "± 6496135",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1263884,
            "range": "± 27409",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1146531,
            "range": "± 25743",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3428,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1145953,
            "range": "± 28862",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 27815,
            "range": "± 630",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696579106678,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3173668,
            "range": "± 29657",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 387,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 305,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 432,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3204870,
            "range": "± 26763",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1881904,
            "range": "± 7198",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 19542606,
            "range": "± 1635007",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 137346402,
            "range": "± 2665292",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 173592901,
            "range": "± 1630324",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 334256465,
            "range": "± 3555474",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 202550373,
            "range": "± 2911813",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 63470027,
            "range": "± 2402506",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 272935581,
            "range": "± 3675048",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14426798,
            "range": "± 852394",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3206422,
            "range": "± 41877",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 510492,
            "range": "± 1313",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19232179,
            "range": "± 807729",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 430554,
            "range": "± 1027",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7727841,
            "range": "± 69319",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8161989,
            "range": "± 141745",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 144258,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5689,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89818,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 249083,
            "range": "± 879",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 37167,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2185,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 41936,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1836581197,
            "range": "± 7587500",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1252880,
            "range": "± 14372",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1877328492,
            "range": "± 10003617",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1340604,
            "range": "± 3732",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1231409,
            "range": "± 5375",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3610,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1251972,
            "range": "± 3377",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 29517,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696598791914,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3147299,
            "range": "± 8562",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 400,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 323,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 458,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3171607,
            "range": "± 8901",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1877857,
            "range": "± 9421",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 18864748,
            "range": "± 604908",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 131783464,
            "range": "± 1258249",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 167850199,
            "range": "± 953906",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 327802417,
            "range": "± 3423498",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 199112155,
            "range": "± 1271388",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 57099123,
            "range": "± 1466808",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 263910521,
            "range": "± 1685946",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14770773,
            "range": "± 99181",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3159531,
            "range": "± 12181",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 508445,
            "range": "± 1510",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19281105,
            "range": "± 231636",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 429985,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7789981,
            "range": "± 12210",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8236689,
            "range": "± 23508",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 144938,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5603,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 90636,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 249624,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 37221,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2221,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 41970,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1800645277,
            "range": "± 4523281",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1300695,
            "range": "± 7041",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1810686113,
            "range": "± 10509466",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1352798,
            "range": "± 22406",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1192965,
            "range": "± 12313",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3615,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1194607,
            "range": "± 11202",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30748,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696603146679,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3087834,
            "range": "± 136337",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 385,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 297,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 434,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3127863,
            "range": "± 25930",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1891234,
            "range": "± 6636",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 19527456,
            "range": "± 1566107",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 134719310,
            "range": "± 2484749",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 169893706,
            "range": "± 789860",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 326015595,
            "range": "± 1909471",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 200068997,
            "range": "± 1767360",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 62061707,
            "range": "± 2266937",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 269718006,
            "range": "± 3023570",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14326934,
            "range": "± 492978",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3244675,
            "range": "± 68236",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 506347,
            "range": "± 1283",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 18978697,
            "range": "± 552850",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 415476,
            "range": "± 1178",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7742587,
            "range": "± 75755",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8150918,
            "range": "± 93446",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 145075,
            "range": "± 307",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5617,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89243,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 249858,
            "range": "± 1714",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 38152,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2255,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 41912,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1852920023,
            "range": "± 20237750",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1292612,
            "range": "± 5290",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1844024480,
            "range": "± 3489830",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1340773,
            "range": "± 9813",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1239700,
            "range": "± 7638",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3623,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1245058,
            "range": "± 4242",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30705,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
        "date": 1696679545575,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3087084,
            "range": "± 7970",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 384,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 298,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 427,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3122869,
            "range": "± 4519",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1887015,
            "range": "± 6983",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 18792040,
            "range": "± 266129",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 135396481,
            "range": "± 673462",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 167987297,
            "range": "± 1005003",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 331014409,
            "range": "± 2754187",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 197927654,
            "range": "± 723534",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 58495266,
            "range": "± 769408",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 264839160,
            "range": "± 1663568",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14538217,
            "range": "± 28944",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3256196,
            "range": "± 12981",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 534227,
            "range": "± 1468",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19083212,
            "range": "± 81578",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 429365,
            "range": "± 1182",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7861212,
            "range": "± 10050",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8274638,
            "range": "± 25104",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 144731,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5624,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89524,
            "range": "± 267",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 246523,
            "range": "± 1345",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 38213,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2219,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 41963,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1797240795,
            "range": "± 5058569",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1270174,
            "range": "± 11484",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1801719640,
            "range": "± 3241469",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1341349,
            "range": "± 19786",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1199857,
            "range": "± 8141",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3755,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1209580,
            "range": "± 7104",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30697,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
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
          "id": "fe8d1e0d550f7d3bc10590e0b7dd5fc4f3d13f3e",
          "message": "Add checkbox to PR template to remind about better PR titles (#3724)\n\n### What\r\n\r\nThis PR adds the fourth checkbox below 👇🏻 \r\n\r\n### Checklist\r\n* [x] I have read and agree to [Contributor\r\nGuide](https://github.com/rerun-io/rerun/blob/main/CONTRIBUTING.md) and\r\nthe [Code of\r\nConduct](https://github.com/rerun-io/rerun/blob/main/CODE_OF_CONDUCT.md)\r\n* [x] ~~I've included a screenshot or gif (if applicable)~~\r\n* [x] ~~I have tested [demo.rerun.io](https://demo.rerun.io/pr/3724) (if\r\napplicable)~~\r\n* [x] The PR title and labels are set such as to maximize their\r\nusefulness for the next release's CHANGELOG\r\n\r\n- [PR Build Summary](https://build.rerun.io/pr/3724)\r\n- [Docs\r\npreview](https://rerun.io/preview/fafee5d2455b7674e935d6b5bb7fb1050a463b37/docs)\r\n<!--DOCS-PREVIEW-->\r\n- [Examples\r\npreview](https://rerun.io/preview/fafee5d2455b7674e935d6b5bb7fb1050a463b37/examples)\r\n<!--EXAMPLES-PREVIEW-->\r\n- [Recent benchmark results](https://ref.rerun.io/dev/bench/)\r\n- [Wasm size tracking](https://ref.rerun.io/dev/sizes/)",
          "timestamp": "2023-10-07T14:15:15+02:00",
          "tree_id": "98fa5537877cc91b18c2342910f12ae8d7c103f0",
          "url": "https://github.com/rerun-io/rerun/commit/fe8d1e0d550f7d3bc10590e0b7dd5fc4f3d13f3e"
        },
        "date": 1696681714117,
        "tool": "cargo",
        "benches": [
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/insert/default",
            "value": 3162441,
            "range": "± 38897",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at/default",
            "value": 386,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/primary/default",
            "value": 301,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/latest_at_missing/secondaries/default",
            "value": 438,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/packed=false/range/default",
            "value": 3201260,
            "range": "± 55697",
            "unit": "ns/iter"
          },
          {
            "name": "datastore/num_rows=1000/num_instances=1000/gc/default",
            "value": 1875217,
            "range": "± 11418",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_message_bundles",
            "value": 21836050,
            "range": "± 1954939",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/generate_messages",
            "value": 139733773,
            "range": "± 2510960",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_log_msg",
            "value": 171883756,
            "range": "± 1250194",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/encode_total",
            "value": 339543930,
            "range": "± 3552885",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_log_msg",
            "value": 205051939,
            "range": "± 1949541",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_message_bundles",
            "value": 65015538,
            "range": "± 2544042",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow/decode_total",
            "value": 277109143,
            "range": "± 3690699",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_message_bundles",
            "value": 14406972,
            "range": "± 302412",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/generate_messages",
            "value": 3267460,
            "range": "± 41786",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_log_msg",
            "value": 531802,
            "range": "± 3631",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/encode_total",
            "value": 19283605,
            "range": "± 767368",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_log_msg",
            "value": 417267,
            "range": "± 1179",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_message_bundles",
            "value": 7790088,
            "range": "± 46902",
            "unit": "ns/iter"
          },
          {
            "name": "mono_points_arrow_batched/decode_total",
            "value": 8276599,
            "range": "± 41754",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_message_bundles",
            "value": 144498,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/generate_messages",
            "value": 5629,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_log_msg",
            "value": 89577,
            "range": "± 249",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/encode_total",
            "value": 248981,
            "range": "± 748",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_log_msg",
            "value": 37983,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_message_bundles",
            "value": 2210,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "batch_points_arrow/decode_total",
            "value": 40942,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/insert",
            "value": 1846014183,
            "range": "± 5950321",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_points2/query",
            "value": 1283808,
            "range": "± 8817",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/insert",
            "value": 1878775900,
            "range": "± 4897247",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_mono_strings2/query",
            "value": 1319727,
            "range": "± 4897",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/insert",
            "value": 1214045,
            "range": "± 18478",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_points2/query",
            "value": 3608,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/insert",
            "value": 1229105,
            "range": "± 16007",
            "unit": "ns/iter"
          },
          {
            "name": "arrow_batch_strings2/query",
            "value": 30684,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "tuid/Tuid::random",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}