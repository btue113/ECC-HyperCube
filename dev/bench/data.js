window.BENCHMARK_DATA = {
  "lastUpdate": 1795674280801,
  "repoUrl": "https://github.com/btue113/ECC-HyperCube",
  "entries": {
    "ecc-hypercube decode time": [
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "a56c800640fa1efb4d172ae53e3444218308ad5e",
          "message": "Rewrite ECC-HyperCube in Rust\n\nPort of the 2013 C++ Reed-Muller hypercube encoder/decoder\n(github.com/bomot113/ECC-HyperCube). Output is bit-for-bit identical\nto the C++ tool; vendored Boost and gtest are no longer needed.",
          "timestamp": "2026-10-06T23:31:12-07:00",
          "tree_id": "9d9ce660c83ea6a708cee221437407cb30a300e2",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/a56c800640fa1efb4d172ae53e3444218308ad5e"
        },
        "date": 1795673847464,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.319,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2693.462,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "8f0577c0df9dba301d0ef7043fc6db97ee81d9bd",
          "message": "Add reference test against the original algorithm\n\ntests/reference.rs is a direct port of the 2013 C++ decoder: explicit\ncube element enumeration, Reed's majority vote over 2^(m-r)-1 parallel\ncubes, and level-by-level cancellation. The library is checked against\nit for every (m, p) with m <= 10, on clean codewords, codewords with\nup to half their bits flipped, and uniformly random words.\n\nWords beyond the error bound matter here: within the bound any correct\ndecoder returns the same message, but beyond it only an exact\nreimplementation of the voting order and tie behaviour does. This is\nthe oracle the following performance commits are checked against.\n\nReference: I. S. Reed, \"A class of multiple-error-correcting codes and\nthe decoding scheme\", Trans. IRE PGIT-4, pp. 38-49, 1954.",
          "timestamp": "2026-10-07T19:20:42-07:00",
          "tree_id": "d8cf1e44084864c2d747125ee955b7d09e6b766e",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/8f0577c0df9dba301d0ef7043fc6db97ee81d9bd"
        },
        "date": 1795673860334,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.189,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2693.394,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "5c8ff6c04f7c178e61f195ef608ac5583be600ba",
          "message": "Relicense under MIT OR Apache-2.0\n\nThe Rust rewrite moves from GPL-2.0-or-later to the Rust ecosystem's\nusual dual license, MIT OR Apache-2.0, so it can be used in\npermissively licensed and commercial projects.\n\nTue Bui is the sole author of every commit here and of the original\n2013 C++ code (bomot113/ECC-HyperCube), so the relicensing needs no\nother contributor's consent. The original C++ release stays available\nunder GPL-2.0-or-later; relicensing doesn't revoke the terms already\ngranted to copies of it.\n\n- LICENSE (GPL-2.0) replaced by LICENSE-MIT and LICENSE-APACHE, the\n  latter downloaded verbatim from apache.org\n- Cargo.toml: license = \"MIT OR Apache-2.0\"\n- README: license section with the standard contribution clause, and a\n  note on the original C++ release's license",
          "timestamp": "2026-10-08T23:16:54-07:00",
          "tree_id": "c8aa44dcebae56d6b5b33e7c1d5e8f9f0adc58c2",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/5c8ff6c04f7c178e61f195ef608ac5583be600ba"
        },
        "date": 1795673873364,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.242,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2728.452,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "e654e83c8e32be992ca5a1b6c31680fcd300d509",
          "message": "README: history of the 2013 original and links to the blog",
          "timestamp": "2026-10-09T23:23:08-07:00",
          "tree_id": "6e8b94035cf732463acedfdcc54fd18dafcc3961",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/e654e83c8e32be992ca5a1b6c31680fcd300d509"
        },
        "date": 1795673886310,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.167,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2701.295,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "afb6bb6638c6ad679e96f685a18e1fc93e027c46",
          "message": "README: where to ask questions",
          "timestamp": "2026-10-10T19:42:41-07:00",
          "tree_id": "086fc020274ae056ba1b9cf8bb07f16c7539d388",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/afb6bb6638c6ad679e96f685a18e1fc93e027c46"
        },
        "date": 1795673899237,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.161,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2704.569,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "3dd5a553614039914367373be16fab48f864355c",
          "message": "README: contact@tuebui.com",
          "timestamp": "2026-10-10T19:51:24-07:00",
          "tree_id": "69faccc1a061ef8b714f02235ae17c3f2d48186a",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/3dd5a553614039914367373be16fab48f864355c"
        },
        "date": 1795673912140,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.166,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2693.352,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "20930b76614f02c90771c53fb4385ab72970daea",
          "message": "README: new blog addresses",
          "timestamp": "2026-10-11T15:17:34-07:00",
          "tree_id": "e302711ab712d035eb0f3f1ce4b01a02b7cc39bf",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/20930b76614f02c90771c53fb4385ab72970daea"
        },
        "date": 1795673925103,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.214,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2693.872,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "178cb027701c303011739bb2c1bdba05cfde2cf7",
          "message": "Speed up the 2013 algorithm without changing it\n\nSame unit cubes, parallel-cube votes in the same order, same majority and the\nsame level-by-level cancellation; only the arithmetic is cheaper:\n\n- a superset XOR transform computes all unit-cube parities at once, for\n  encoding, the clean-word check, cancellation and data recovery\n  (O(m 2^m) instead of one cube at a time);\n- voting packs vertices into 64-bit words, so one masked popcount covers a\n  voting cube's lowest six dimensions;\n- voting patterns are stored once per vertex weight, and full voter lists are\n  built only on request.\n\ntests/reference.rs (all m <= 10, words beyond the error bound) and a 1,134-job\ncross-check against v0.2-plain-port (m = 2-15) give identical output. Adds\nbenches/codec.rs.",
          "timestamp": "2026-10-27T22:18:25-07:00",
          "tree_id": "13b3075c782c7be883d3bf561c7b60d72f727723",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/178cb027701c303011739bb2c1bdba05cfde2cf7"
        },
        "date": 1795673930444,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.378,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 578.785,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "2eb213538e5d955f803f8d22c5badc16bc9a231a",
          "message": "Add recursive hypercube SCL soft-decision decoder",
          "timestamp": "2026-10-29T21:11:18-07:00",
          "tree_id": "735df96d31612ed29cc126aa836be3baf7e38b5b",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/2eb213538e5d955f803f8d22c5badc16bc9a231a"
        },
        "date": 1795673940727,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.296,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 575.482,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 186.241,
            "extra": "26 / 4000 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "78489d007b470b682e067a2a2db5137f6f6fecde",
          "message": "Optimize SCL soft decoder with reusable workspace and min-sum",
          "timestamp": "2026-10-31T13:35:35-07:00",
          "tree_id": "1368d2b35d0869626cc7e581e77add0c8a0db0d4",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/78489d007b470b682e067a2a2db5137f6f6fecde"
        },
        "date": 1795673947295,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.272,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 575.998,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 37.941,
            "extra": "25 / 4000 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "f3c3da934a97d7372921b6063f491e3d1746bef5",
          "message": "Keep the library to long-published RM techniques\n\nMove the CRC-on-parity-shell (with_crc, beta_weight) and PAC (with_pac)\nconstructions out of the library into benches/polar_5g.rs as comparison\nbaselines (PolarCode5G::rm_crc / rm_pac with a certified PAC decoder and a\nstartup self-check), since they rely on techniques covered by active\npatents. The library keeps RM encoding, the 2013 majority voter, recursive\nlist decoding, the automorphism ensemble and the ML certificate.\n\nExclude the comparison benchmark and the root brainstorming notes and\nfigures from the published package.\n\nBump to 0.4.0 and update CITATION.cff (release date, author email, and the\nSoftware Heritage snapshot of the 2013 repository).\n\nRework the README around why the crate matters: headline results against\n5G NR polar codes with caveats, the geometric teaching angle, and prior art\nfor the automorphism ensemble and ML certificate. Drop the plain-port speed\ntable and unlink blog parts that are not yet published.",
          "timestamp": "2026-11-20T15:41:18-08:00",
          "tree_id": "bdfc6447c39d28299490c09620f98441b9ce04db",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/f3c3da934a97d7372921b6063f491e3d1746bef5"
        },
        "date": 1795673962098,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.289,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 676.124,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 44.953,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 55.182,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 175.185,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 1222.778,
            "extra": "2 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "ad4fda7d0ab96d919880f62b1347a5bce963b0a6",
          "message": "Vectorize the soft list decoder in safe Rust\n\nReplace SclDecoder with ListDecoder<L>, which stores the L list paths\nside by side (one [f32; L] per vertex) so every f, g and combine step is\none loop over L values that the compiler vectorizes. No unsafe code and\nno platform intrinsics. List sizes round up to a power of two, at most 256.\n\nSubtrees that are all frozen, a repetition code RM(0, d), or (greedy\nonly) all free RM(d, d) are finished in one step, as in Dumer's\nrecursive decoding of RM codes (2004, 2006).\n\nSingle-thread speedup over the previous decoder: 1.3-1.9x (e.g.\n(256,93) 32x8 ensemble 111 -> 71 us). Decoded words and certificates\nmatch the previous decoder on 51,500 test frames except for two float32\nnear-ties; no new certificates.\n\nAdds benches/simd_bench.rs for single-thread timing.",
          "timestamp": "2026-11-20T15:55:51-08:00",
          "tree_id": "05ab1e9c707df09045cf96e1af6b171d940f8851",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/ad4fda7d0ab96d919880f62b1347a5bce963b0a6"
        },
        "date": 1795673974201,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.379,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 576.236,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 20.146,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 25.086,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 78.377,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 502.538,
            "extra": "2 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "0e17bf453dc9e2fa2f232f5a8930ae3d8ff649a2",
          "message": "Add nested Plotkin automorphism-ensemble decoding for n >= 512\n\nFor cw_dims >= 9 (n >= 512) and 2 <= r <= m - 2, decode_soft_ensemble runs copy 0 as plain SCL on the whole cube (preserving the never-worse-than-SCL guarantee and pruned_bound ML certificate), then recurses one Plotkin split deep: RM(r, m) = (c1 ^ u, c1) with inner ensembles on u in RM(r-1, m-1) and c1 in RM(r, m-1) inside each outer GA(m, 2) relabelling.\n\nAt n = 512, pure RM matches or beats AFF3CT 5G polar + CRC11 SCL-32 on WER across (512, 130) and (512, 256) (300k-word runs) at 2-5x the time per word.",
          "timestamp": "2026-11-21T00:52:54-08:00",
          "tree_id": "a3c2fd45bcbcc500905c6aa0e69f09c469dc0eae",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/0e17bf453dc9e2fa2f232f5a8930ae3d8ff649a2"
        },
        "date": 1795673986816,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.286,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 575.734,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 19.976,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 24.727,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 77.399,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 598.59,
            "extra": "0 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "ba1e74881dfcb4e8a89f88f05ad70cf654d92088",
          "message": "Decode first-order RM subtrees in one FWHT step\n\nThe list decoder now finishes RM(1, d) subtrees (d >= 2) with a fast\nWalsh-Hadamard transform (Green 1966; Dumer 2004): one transform scores\nall 2^(d+1) codewords per path, and each path forks once into its best\ncandidates instead of once per information bit. The leaf is exact, so the\npruned bound stays a valid ML certificate, and RM(1, m) decodes as ML.\n\nsimd_bench (single thread, median of 3): 1.00-1.16x faster, never slower;\nsingle-SCL word errors drop 12-18% (e.g. (256,93) L=8: 499 -> 407).\nThe Walsh index is stored as u32 so subtrees up to MAX_DIMS fit; the new\ntest covers RM(1, 15..17).",
          "timestamp": "2026-11-21T01:10:36-08:00",
          "tree_id": "e85377ffd387d03f7de74467374c0a12e89adc36",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/ba1e74881dfcb4e8a89f88f05ad70cf654d92088"
        },
        "date": 1795674000553,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.281,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 575.421,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.595,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.522,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 63.493,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 543.705,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "70a02380f53a814eda4254b70a44870a16d8f82e",
          "message": "Stop ensemble early when runner-up costs > 1.8x best\n\nAdds EARLY_STOP_COST_RATIO = 1.8 for list_size >= 4 in flat and nested ensemble decoding: when runner_up_cost > 1.8 * best_cost, exit early without running more copies.\n\nScale-invariant (no SNR estimate, no CRC, no extra compute). Across 1.25M test frames spanning n = 128..1024 (L = 4 and L = 8), 1.8 added zero extra errors while cutting decode time by 1.1-1.85x (e.g. (256,93) 2.5 dB 8x32: 100 -> 73 us; (512,256) 3.0 dB 4x16: 299 -> 174 us, 8x32: 419 -> 226 us).",
          "timestamp": "2026-11-21T22:03:06-08:00",
          "tree_id": "d9826fc651c269947e9b9bfa3e512ae0f712a4d4",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/70a02380f53a814eda4254b70a44870a16d8f82e"
        },
        "date": 1795674013725,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.6,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 582.063,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.488,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.251,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 48.595,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 259.253,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "884e60daf6225c64b13b90bf4f76bb8ac6cfc40a",
          "message": "Step 1: Add Hypercube GLDPC (Rate 3/8) with pure tanh-domain HR-FWHT SISO",
          "timestamp": "2026-11-22T01:02:43-08:00",
          "tree_id": "63537bc94ed7b4250e1c02cb540361d58f167227",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/884e60daf6225c64b13b90bf4f76bb8ac6cfc40a"
        },
        "date": 1795674026196,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.265,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 589.24,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.491,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 20.24,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.564,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 257.767,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "629b0dae7f56f065b893fbf3eac016088db37769",
          "message": "Step 2: Add tanh-domain momentum damping (0.75/0.25) for iter >= 1",
          "timestamp": "2026-11-22T01:03:06-08:00",
          "tree_id": "67b87c03fba7f66c35a261265ab0a61493c6683b",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/629b0dae7f56f065b893fbf3eac016088db37769"
        },
        "date": 1795674038443,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.247,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 571.67,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.669,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.996,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.653,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 259.921,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "5438572b68d91723ecce3609e96977317b087e7e",
          "message": "Step 3: Replace scalar f32::tanh init with Pade [5/4] + half-angle doubling",
          "timestamp": "2026-11-22T01:03:29-08:00",
          "tree_id": "9a127c56f8fd453d6b089eacbe65eeffb1b8c62c",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/5438572b68d91723ecce3609e96977317b087e7e"
        },
        "date": 1795674050596,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.273,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 571.584,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.646,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.67,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.637,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 255.097,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "e0f39622e332292b01a084f347d83404218fb17a",
          "message": "Step 4b: Raise max_iters from 12 to 24 (5.5x lower WER, 0 undetected)",
          "timestamp": "2026-11-22T01:03:55-08:00",
          "tree_id": "af45976b4b2b59125a3f66d7f28531d01e6de866",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/e0f39622e332292b01a084f347d83404218fb17a"
        },
        "date": 1795674062640,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.272,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 572.274,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.608,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.954,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.228,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 258.133,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "f6f1961ebd2aea99f95cb6bb82c06ff034c2a47d",
          "message": "Step 5a/5b: Branchless syndrome & clamp_t, SIMD x-outer gather/scatter, skip iter-0 div",
          "timestamp": "2026-11-22T01:04:45-08:00",
          "tree_id": "2dadab26931fcac209e79a45f8610a86acf93673",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/f6f1961ebd2aea99f95cb6bb82c06ff034c2a47d"
        },
        "date": 1795674074919,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.267,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 571.553,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.623,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.934,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.408,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 259.436,
            "extra": "1 / 500 word errors"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "e54500805e85c5157d11383b78ac367bafb4f7bb",
          "message": "Step 6: Add exact Rate-1/2 GLDPC via RM(2,4) + RM(3,5) HR-FWHT",
          "timestamp": "2026-11-22T04:02:41-08:00",
          "tree_id": "d0b796c545bc95818de8bb1b4fa6ce6ff2023693",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/e54500805e85c5157d11383b78ac367bafb4f7bb"
        },
        "date": 1795674087722,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.281,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.511,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.425,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.719,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.992,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 286.022,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.282,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "cf55a58a387da5a2c049e31ef18e2179e0891668",
          "message": "Step 7: Raise max_iters from 24 to 50 (1.5-4x lower WER, zero avg time cost)",
          "timestamp": "2026-11-22T04:03:00-08:00",
          "tree_id": "35a5fb47ba15ce87c714b7d0556d496d4066318f",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/cf55a58a387da5a2c049e31ef18e2179e0891668"
        },
        "date": 1795674100927,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.353,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.238,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.367,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.686,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.613,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 252.992,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.085,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "0192d390b1cf0712131795b80b4edc8221f1aad0",
          "message": "Add n=256/2048 and random-codeword AWGN unit tests for HypercubeGldpc",
          "timestamp": "2026-11-22T15:02:35-08:00",
          "tree_id": "64a7fb681c982f4ee78209d8166bf492fbbd0d53",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/0192d390b1cf0712131795b80b4edc8221f1aad0"
        },
        "date": 1795674113737,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.279,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 574.316,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.364,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.669,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.654,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 252.946,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.894,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "ee1fc752b01f22dbc22c97378c129440f90c8185",
          "message": "Update README with RM + Hypercube GLDPC results and reproducible wer example",
          "timestamp": "2026-11-22T21:11:31-08:00",
          "tree_id": "32a48029857310f3d49cbc649d48b3e094d972f0",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/ee1fc752b01f22dbc22c97378c129440f90c8185"
        },
        "date": 1795674126573,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.283,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.442,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.373,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.672,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.049,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 254.735,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.944,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "bc6d297f72a00945fa0a01af278f0a77202a0420",
          "message": "Highlight hypercube teaching progression in README",
          "timestamp": "2026-11-22T21:16:20-08:00",
          "tree_id": "11af06a4d0e17002785d907ab4e6ea6ddc3902a8",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/bc6d297f72a00945fa0a01af278f0a77202a0420"
        },
        "date": 1795674139254,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.316,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 574.313,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.605,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.883,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.135,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 256.189,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.304,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "ac136a19733dd448b30ded2ae2458faf8519c6cb",
          "message": "Define GLDPC on first use in README",
          "timestamp": "2026-11-22T21:18:34-08:00",
          "tree_id": "375709e9a18e81e1d471d479d77c014b54dfac33",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/ac136a19733dd448b30ded2ae2458faf8519c6cb"
        },
        "date": 1795674152080,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.335,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 575.201,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.632,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.499,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 48.404,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 270.278,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 23.109,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "245d527c2f97002a4c5fff5a917f7cedb3131890",
          "message": "Credit AI agents (Gemini and Claude) in README subtitle",
          "timestamp": "2026-11-22T21:22:15-08:00",
          "tree_id": "e6d59c86453ea1cab11d074dcf593b1d69bfedbc",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/245d527c2f97002a4c5fff5a917f7cedb3131890"
        },
        "date": 1795674165596,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.817,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 606.326,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.764,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 20.181,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 49.915,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 275.653,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.002,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "1b23b507dfcf2242b4a71edae1e66d8e40cc09f2",
          "message": "Add BibTeX Citation block to README",
          "timestamp": "2026-11-22T21:26:49-08:00",
          "tree_id": "674b92f74f4c211330685990ba0d012c30d5c158",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/1b23b507dfcf2242b4a71edae1e66d8e40cc09f2"
        },
        "date": 1795674178979,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.537,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 599.271,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.709,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.635,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 48.64,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 258.438,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.237,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "7bd3b6d46ebdedb48e3b1ae0e1cf870c886c5aef",
          "message": "Polish README prior art/citations and bump version to 0.5.0",
          "timestamp": "2026-11-22T21:34:09-08:00",
          "tree_id": "49dd4f28f80a7dc2a77e029942214cd5027a45a2",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/7bd3b6d46ebdedb48e3b1ae0e1cf870c886c5aef"
        },
        "date": 1795674192584,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.296,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 572.681,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.682,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.986,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 46.229,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 257.939,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.15,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "2b6e14a4cd7d9539992fa52114d6a13b0fbf6538",
          "message": "Describe GLDPC in crate metadata and fix README ensemble and patent wording",
          "timestamp": "2026-11-22T21:45:00-08:00",
          "tree_id": "5f88c88c280ef80c49764eaf43e88b00f15c841c",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/2b6e14a4cd7d9539992fa52114d6a13b0fbf6538"
        },
        "date": 1795674205162,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.238,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.684,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.427,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.638,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.634,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 254.655,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.357,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "ed9e24012801e4cecd67de47b95e222574661b47",
          "message": "Add Zenodo metadata and release 0.5.1",
          "timestamp": "2026-11-22T21:59:57-08:00",
          "tree_id": "dc198f61a13616c1c4397c6d6cd192f4bf4ba554",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/ed9e24012801e4cecd67de47b95e222574661b47"
        },
        "date": 1795674218012,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 10.031,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.933,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.512,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.813,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.999,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 255.179,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.362,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "f7bd72b1a960a10206271e017520953e7fbbbf98",
          "message": "Add Zenodo concept DOI to citation metadata and README",
          "timestamp": "2026-11-22T22:01:06-08:00",
          "tree_id": "b4f1cdb060c4d4583d96451a30ae79d6e81e6e43",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/f7bd72b1a960a10206271e017520953e7fbbbf98"
        },
        "date": 1795674230684,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.282,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.007,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.336,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.598,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.679,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 252.376,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.954,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "4c8ecf38ca7e88147031580e39a3d0306ee77c01",
          "message": "Add GitHub Actions CI, CONTRIBUTING.md, and native CPU build note",
          "timestamp": "2026-11-25T17:21:16-08:00",
          "tree_id": "4f57ba92fb0243e049c9d4c40ba17f895186a366",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/4c8ecf38ca7e88147031580e39a3d0306ee77c01"
        },
        "date": 1795674243363,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.263,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 573.564,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.423,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 18.623,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 45.886,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 252.754,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.692,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "77397a7375e7841959a72bb05c15535c4907b126",
          "message": "Add cheap first pass for RM ensemble and lock-free RmScratch\n\nEnsemble decoding first tries a small list (L = 1, then L = 4 for lists of 16 or\nmore) on the identity cube and returns early when the decision clears copy 0's\nbar: the distance test or cost <= pruned bound (both prove maximum likelihood), or\nthe ensemble's existing heuristics (fewer than d/2 flips, runner-up ratio at\nL >= 4). Paired over 1.4M AWGN frames (RM(3,7), RM(4,8), RM(2,8), 8x8), 2 decisions\ndiffer from the full ensemble; decoding is 1.4-2.9x faster at medium and high SNR.\n\nRmScratch holds the cached list decoders, keyed by dimension and list width, so\ncallers can decode lock-free from many threads through the *_with_scratch methods;\nthe existing methods keep using the internal Mutex.",
          "timestamp": "2026-11-25T21:30:43-08:00",
          "tree_id": "e1283b23e7b6365230c3d9018d553bcc5b8ef6bb",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/77397a7375e7841959a72bb05c15535c4907b126"
        },
        "date": 1795674255371,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.186,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 572.146,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.718,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 9.849,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 30.911,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 186.038,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.813,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "606b78af367a97cd386820ffc5010c3fbab02126",
          "message": "Add per-commit perf tracking: fixed decode workloads charted on GitHub Pages\n\nexamples/perf_ci.rs times fixed single-thread decode workloads (hard, SCL, ensemble, GLDPC) and writes customSmallerIsBetter JSON; .github/workflows/perf.yml runs it on every push to main and appends the results to a chart on gh-pages (/dev/bench/).",
          "timestamp": "2026-11-25T21:46:49-08:00",
          "tree_id": "6d849e1639ff39648ece4695a7edcf0dc8d0cb2d",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/606b78af367a97cd386820ffc5010c3fbab02126"
        },
        "date": 1795674267469,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.13,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 572.021,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.739,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 9.853,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 30.88,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 186.712,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 20.992,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "committer": {
            "email": "bomot113@gmail.com",
            "name": "Tue Bui",
            "username": "btue113"
          },
          "distinct": true,
          "id": "179e6b2045ed92e0083cef39068e0972f93ec365",
          "message": "Add perf backfill: measure every past commit with today's workloads\n\nexamples/perf_ci.rs now skips a workload that panics and tags workloads by the API they need (perf-group). ci/perf_backfill.py exports each first-parent commit, strips the groups that commit lacks, builds it in a fresh target dir and runs it; the manual Perf backfill workflow does this on one runner and replaces the chart history on gh-pages.",
          "timestamp": "2026-11-25T22:16:20-08:00",
          "tree_id": "6c8ee5be698cfcf3b807114649d8efc88ed6624e",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/179e6b2045ed92e0083cef39068e0972f93ec365"
        },
        "date": 1795674279519,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.208,
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 572.732,
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.535,
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 9.761,
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 30.716,
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 186.397,
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.112,
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      }
    ]
  }
}
