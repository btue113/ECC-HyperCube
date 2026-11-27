window.BENCHMARK_DATA = {
  "lastUpdate": 1795771447799,
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
        "date": 1795767171038,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 17.984,
            "extra": "188.3% of baseline (raw 24.31 vs 12.91 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2979.459,
            "extra": "498.2% of baseline (raw 4089.03 vs 820.58 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795767318883,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.032,
            "extra": "188.8% of baseline (raw 24.32 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2978.63,
            "extra": "498.1% of baseline (raw 4085.51 vs 821.12 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795767466679,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.03,
            "extra": "188.8% of baseline (raw 24.32 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2980.183,
            "extra": "498.4% of baseline (raw 4087.87 vs 820.41 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795767614530,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.008,
            "extra": "188.6% of baseline (raw 24.33 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2979.865,
            "extra": "498.3% of baseline (raw 4086.02 vs 820.14 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795767762442,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.04,
            "extra": "188.9% of baseline (raw 24.33 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2978.721,
            "extra": "498.1% of baseline (raw 4087.78 vs 820.67 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795767910238,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.055,
            "extra": "189.1% of baseline (raw 24.34 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2981.394,
            "extra": "498.6% of baseline (raw 4087.13 vs 820.59 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768058002,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 18.029,
            "extra": "188.8% of baseline (raw 24.31 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 2977.574,
            "extra": "497.9% of baseline (raw 4085.78 vs 820.58 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768129930,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.448,
            "extra": "98.9% of baseline (raw 12.72 vs 12.86 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.98,
            "extra": "100.2% of baseline (raw 821.32 vs 819.95 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768244121,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.831,
            "extra": "102.9% of baseline (raw 13.27 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 594.843,
            "extra": "99.5% of baseline (raw 816.37 vs 820.62 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 201.447,
            "extra": "1246.6% of baseline (raw 259.75 vs 20.85 us/word); 26 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768324682,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.534,
            "extra": "99.8% of baseline (raw 12.87 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 588.853,
            "extra": "98.5% of baseline (raw 808.41 vs 820.98 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 40.352,
            "extra": "249.7% of baseline (raw 52.07 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768465055,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.537,
            "extra": "99.9% of baseline (raw 12.87 vs 12.87 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.106,
            "extra": "99.8% of baseline (raw 820.57 vs 821.39 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 40.886,
            "extra": "253.0% of baseline (raw 52.99 vs 20.95 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 49.23,
            "extra": "484.5% of baseline (raw 64.89 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 156.289,
            "extra": "485.8% of baseline (raw 203.84 vs 41.96 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 1161.534,
            "extra": "599.7% of baseline (raw 1597.30 vs 265.96 us/word); 2 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768574382,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.875,
            "extra": "103.4% of baseline (raw 13.32 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.19,
            "extra": "100.0% of baseline (raw 820.96 vs 820.72 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 20.767,
            "extra": "128.5% of baseline (raw 26.80 vs 20.90 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 25.289,
            "extra": "248.9% of baseline (raw 33.29 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 80.786,
            "extra": "251.1% of baseline (raw 105.59 vs 42.05 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 531.949,
            "extra": "274.7% of baseline (raw 732.21 vs 265.96 us/word); 2 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768685905,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.536,
            "extra": "99.9% of baseline (raw 12.88 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.366,
            "extra": "99.9% of baseline (raw 820.16 vs 820.96 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 20.414,
            "extra": "126.3% of baseline (raw 26.46 vs 20.95 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 24.772,
            "extra": "243.8% of baseline (raw 32.66 vs 13.38 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 79.152,
            "extra": "246.0% of baseline (raw 103.36 vs 42.01 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 633.796,
            "extra": "327.2% of baseline (raw 870.52 vs 265.59 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768793140,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.474,
            "extra": "99.2% of baseline (raw 12.79 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 584.047,
            "extra": "97.7% of baseline (raw 801.66 vs 820.82 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.25,
            "extra": "100.6% of baseline (raw 21.18 vs 21.02 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 20.109,
            "extra": "197.9% of baseline (raw 26.42 vs 13.35 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 66.094,
            "extra": "205.5% of baseline (raw 86.13 vs 41.94 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 533.672,
            "extra": "275.5% of baseline (raw 732.02 vs 265.51 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768891387,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.629,
            "extra": "100.8% of baseline (raw 13.00 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 592.328,
            "extra": "99.0% of baseline (raw 812.68 vs 820.26 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.318,
            "extra": "101.0% of baseline (raw 21.04 vs 20.89 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.347,
            "extra": "190.4% of baseline (raw 25.53 vs 13.40 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.818,
            "extra": "148.6% of baseline (raw 62.48 vs 42.04 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 265.47,
            "extra": "137.1% of baseline (raw 365.43 vs 266.76 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795768989969,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.621,
            "extra": "100.7% of baseline (raw 12.98 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 592.741,
            "extra": "99.1% of baseline (raw 812.35 vs 820.49 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.28,
            "extra": "100.7% of baseline (raw 21.03 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.392,
            "extra": "190.9% of baseline (raw 25.54 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.851,
            "extra": "148.7% of baseline (raw 62.70 vs 42.03 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 266.408,
            "extra": "137.6% of baseline (raw 365.69 vs 265.68 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769088483,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.617,
            "extra": "100.7% of baseline (raw 12.99 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 592.813,
            "extra": "99.1% of baseline (raw 812.91 vs 820.72 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.333,
            "extra": "101.1% of baseline (raw 21.08 vs 20.86 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.389,
            "extra": "190.8% of baseline (raw 25.49 vs 13.35 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.803,
            "extra": "148.6% of baseline (raw 62.55 vs 42.02 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 266.85,
            "extra": "137.8% of baseline (raw 366.10 vs 266.06 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769187039,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.631,
            "extra": "100.8% of baseline (raw 12.99 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 592.358,
            "extra": "99.1% of baseline (raw 812.65 vs 820.76 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.247,
            "extra": "100.5% of baseline (raw 21.04 vs 20.84 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.424,
            "extra": "191.2% of baseline (raw 25.50 vs 13.34 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.733,
            "extra": "148.4% of baseline (raw 62.51 vs 42.06 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.494,
            "extra": "138.1% of baseline (raw 366.77 vs 265.53 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769285665,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.632,
            "extra": "100.9% of baseline (raw 12.99 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 591.763,
            "extra": "99.0% of baseline (raw 812.69 vs 821.45 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.389,
            "extra": "101.4% of baseline (raw 21.17 vs 20.88 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.416,
            "extra": "191.1% of baseline (raw 25.50 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.8,
            "extra": "148.6% of baseline (raw 62.40 vs 42.05 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 266.209,
            "extra": "137.4% of baseline (raw 365.18 vs 265.69 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769384285,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.646,
            "extra": "101.0% of baseline (raw 13.01 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 592.857,
            "extra": "99.1% of baseline (raw 812.51 vs 819.74 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.295,
            "extra": "100.8% of baseline (raw 21.07 vs 20.93 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.369,
            "extra": "190.6% of baseline (raw 25.50 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.809,
            "extra": "148.6% of baseline (raw 62.58 vs 41.99 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 266.176,
            "extra": "137.4% of baseline (raw 365.16 vs 265.69 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769484723,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.938,
            "extra": "104.1% of baseline (raw 13.42 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.865,
            "extra": "100.0% of baseline (raw 820.24 vs 820.41 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.259,
            "extra": "100.6% of baseline (raw 20.96 vs 20.86 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.3,
            "extra": "190.0% of baseline (raw 25.41 vs 13.38 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.816,
            "extra": "148.6% of baseline (raw 62.51 vs 42.02 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.807,
            "extra": "138.3% of baseline (raw 367.56 vs 265.79 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.274,
            "extra": "100.5% of baseline (raw 33.31 vs 32.84 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769585140,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.964,
            "extra": "104.3% of baseline (raw 13.42 vs 12.87 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.623,
            "extra": "99.9% of baseline (raw 820.06 vs 820.56 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.237,
            "extra": "100.5% of baseline (raw 20.94 vs 20.84 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.271,
            "extra": "189.7% of baseline (raw 25.39 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.791,
            "extra": "148.6% of baseline (raw 62.51 vs 42.07 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.603,
            "extra": "138.2% of baseline (raw 367.90 vs 265.81 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.211,
            "extra": "100.2% of baseline (raw 32.64 vs 32.38 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769685609,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.964,
            "extra": "104.3% of baseline (raw 13.41 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 596.976,
            "extra": "99.8% of baseline (raw 819.94 vs 821.36 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.273,
            "extra": "100.7% of baseline (raw 20.99 vs 20.91 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.334,
            "extra": "190.3% of baseline (raw 25.42 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.793,
            "extra": "148.6% of baseline (raw 62.48 vs 42.04 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.267,
            "extra": "138.0% of baseline (raw 366.89 vs 266.10 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.608,
            "extra": "97.5% of baseline (raw 32.19 vs 33.02 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769786193,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.929,
            "extra": "104.0% of baseline (raw 13.41 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.166,
            "extra": "100.0% of baseline (raw 820.32 vs 820.57 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.246,
            "extra": "100.5% of baseline (raw 20.94 vs 20.83 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.325,
            "extra": "190.2% of baseline (raw 25.45 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 48.037,
            "extra": "149.3% of baseline (raw 62.51 vs 41.98 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.523,
            "extra": "138.1% of baseline (raw 367.37 vs 265.91 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.09,
            "extra": "99.7% of baseline (raw 32.95 vs 33.07 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769886676,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.937,
            "extra": "104.0% of baseline (raw 13.42 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 600.268,
            "extra": "100.4% of baseline (raw 820.15 vs 819.31 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.267,
            "extra": "100.7% of baseline (raw 20.98 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.321,
            "extra": "190.2% of baseline (raw 25.40 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.77,
            "extra": "148.5% of baseline (raw 62.29 vs 41.99 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 268.73,
            "extra": "138.7% of baseline (raw 368.05 vs 265.54 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.83,
            "extra": "98.5% of baseline (raw 32.50 vs 32.99 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795769987201,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.951,
            "extra": "104.2% of baseline (raw 13.41 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.224,
            "extra": "99.9% of baseline (raw 820.21 vs 820.93 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.293,
            "extra": "100.8% of baseline (raw 21.02 vs 20.87 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.228,
            "extra": "189.3% of baseline (raw 25.37 vs 13.43 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.84,
            "extra": "148.7% of baseline (raw 62.39 vs 41.94 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.878,
            "extra": "138.3% of baseline (raw 368.39 vs 265.62 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.222,
            "extra": "100.3% of baseline (raw 32.88 vs 32.37 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770087609,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.962,
            "extra": "104.3% of baseline (raw 13.44 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.224,
            "extra": "99.9% of baseline (raw 819.76 vs 820.75 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.266,
            "extra": "100.7% of baseline (raw 20.99 vs 20.84 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.254,
            "extra": "189.5% of baseline (raw 25.37 vs 13.38 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.696,
            "extra": "148.3% of baseline (raw 62.37 vs 42.01 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.749,
            "extra": "138.2% of baseline (raw 367.05 vs 265.87 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.169,
            "extra": "100.0% of baseline (raw 32.69 vs 32.83 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770188003,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.94,
            "extra": "104.1% of baseline (raw 13.41 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.074,
            "extra": "100.0% of baseline (raw 820.00 vs 820.01 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.273,
            "extra": "100.7% of baseline (raw 21.00 vs 20.83 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.335,
            "extra": "190.3% of baseline (raw 25.38 vs 13.34 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.879,
            "extra": "148.8% of baseline (raw 62.50 vs 41.91 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 266.656,
            "extra": "137.7% of baseline (raw 367.22 vs 265.88 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.925,
            "extra": "98.9% of baseline (raw 32.36 vs 33.27 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770288396,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.936,
            "extra": "104.0% of baseline (raw 13.45 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 599.125,
            "extra": "100.2% of baseline (raw 820.72 vs 819.87 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.23,
            "extra": "100.4% of baseline (raw 20.96 vs 20.86 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.255,
            "extra": "189.5% of baseline (raw 25.38 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.746,
            "extra": "148.4% of baseline (raw 62.48 vs 42.05 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 268.181,
            "extra": "138.5% of baseline (raw 368.09 vs 265.84 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.738,
            "extra": "102.6% of baseline (raw 33.23 vs 32.39 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770388975,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.969,
            "extra": "104.4% of baseline (raw 13.45 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.105,
            "extra": "100.0% of baseline (raw 820.72 vs 820.52 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 15.977,
            "extra": "98.9% of baseline (raw 21.02 vs 21.26 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.325,
            "extra": "190.2% of baseline (raw 25.41 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.826,
            "extra": "148.7% of baseline (raw 62.51 vs 42.03 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.988,
            "extra": "138.4% of baseline (raw 368.01 vs 266.04 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.249,
            "extra": "100.4% of baseline (raw 32.59 vs 32.33 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770489414,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.944,
            "extra": "104.1% of baseline (raw 13.41 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.486,
            "extra": "99.9% of baseline (raw 820.26 vs 820.96 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.263,
            "extra": "100.6% of baseline (raw 20.99 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.32,
            "extra": "190.2% of baseline (raw 25.42 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.877,
            "extra": "148.8% of baseline (raw 62.51 vs 42.03 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.375,
            "extra": "138.0% of baseline (raw 366.62 vs 265.93 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.561,
            "extra": "101.8% of baseline (raw 32.99 vs 32.51 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770589945,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.954,
            "extra": "104.2% of baseline (raw 13.45 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.178,
            "extra": "100.0% of baseline (raw 820.61 vs 820.77 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.275,
            "extra": "100.7% of baseline (raw 20.96 vs 20.83 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.304,
            "extra": "190.0% of baseline (raw 25.42 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.827,
            "extra": "148.7% of baseline (raw 62.42 vs 42.00 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.131,
            "extra": "137.9% of baseline (raw 367.25 vs 266.37 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.116,
            "extra": "99.8% of baseline (raw 32.98 vs 33.04 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770690457,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.916,
            "extra": "103.8% of baseline (raw 13.41 vs 12.91 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.337,
            "extra": "99.9% of baseline (raw 820.07 vs 820.83 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.258,
            "extra": "100.6% of baseline (raw 21.09 vs 20.88 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 19.341,
            "extra": "190.4% of baseline (raw 25.43 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 47.903,
            "extra": "148.9% of baseline (raw 62.51 vs 42.01 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 267.555,
            "extra": "138.1% of baseline (raw 366.75 vs 265.68 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.795,
            "extra": "98.4% of baseline (raw 32.44 vs 32.77 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770784896,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.554,
            "extra": "100.0% of baseline (raw 12.90 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 595.265,
            "extra": "99.5% of baseline (raw 817.54 vs 821.23 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.128,
            "extra": "99.8% of baseline (raw 20.84 vs 20.87 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.156,
            "extra": "100.0% of baseline (raw 13.36 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.148,
            "extra": "99.9% of baseline (raw 42.01 vs 42.04 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.319,
            "extra": "99.8% of baseline (raw 265.62 vs 265.90 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.691,
            "extra": "102.4% of baseline (raw 32.98 vs 32.20 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770879278,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.545,
            "extra": "99.9% of baseline (raw 12.88 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.106,
            "extra": "100.0% of baseline (raw 820.44 vs 820.60 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.266,
            "extra": "100.7% of baseline (raw 20.91 vs 20.83 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.169,
            "extra": "100.1% of baseline (raw 13.35 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.074,
            "extra": "99.7% of baseline (raw 41.90 vs 42.00 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 194.043,
            "extra": "100.2% of baseline (raw 266.30 vs 265.88 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.714,
            "extra": "98.0% of baseline (raw 32.22 vs 32.85 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
        "date": 1795770973707,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.565,
            "extra": "100.2% of baseline (raw 12.89 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.072,
            "extra": "100.0% of baseline (raw 819.39 vs 819.93 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.114,
            "extra": "99.7% of baseline (raw 20.85 vs 20.87 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.202,
            "extra": "100.4% of baseline (raw 13.40 vs 13.35 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.284,
            "extra": "100.4% of baseline (raw 42.06 vs 41.99 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.707,
            "extra": "100.0% of baseline (raw 266.15 vs 266.11 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.102,
            "extra": "99.7% of baseline (raw 32.65 vs 32.72 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
          "id": "194c5c454e5aea79099f94235b3c747ff9fdf1c0",
          "message": "Refresh RM ensemble timings and add WER curve figure\n\nRemeasured the RM ensemble decode times after the cheap first pass (M3 Pro, 8 threads, same method as before): (128,64) 8x4 is now 2.3x faster than software SCL8 and (512,256) 32x8 17x slower (was 25x). WER results are unchanged.\n\ndocs/wer/wer_curves.svg plots WER vs Eb/N0 at n = 128, 256, 512 and 1024 for the RM ensemble, GLDPC and AFF3CT polar SCL8/ASCL128 + CRC-11, from 72 fresh points in docs/wer/wer.csv. scripts/wer_curves.py measures and plots them. docs/, scripts/ and ci/ are excluded from the published crate.",
          "timestamp": "2026-11-25T22:37:06-08:00",
          "tree_id": "2b8a5b6ff57f55315eaff5d3e44726f41eec8954",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/194c5c454e5aea79099f94235b3c747ff9fdf1c0"
        },
        "date": 1795771068141,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.54,
            "extra": "99.9% of baseline (raw 12.87 vs 12.90 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.393,
            "extra": "100.1% of baseline (raw 820.45 vs 819.78 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.127,
            "extra": "99.8% of baseline (raw 20.82 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.165,
            "extra": "100.0% of baseline (raw 13.33 vs 13.34 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.173,
            "extra": "100.0% of baseline (raw 41.91 vs 41.93 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.423,
            "extra": "99.9% of baseline (raw 265.49 vs 265.82 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.308,
            "extra": "100.7% of baseline (raw 33.24 vs 33.19 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
          "id": "fee9cfbfb1ba5016178f9e068d3bb31dec8511a7",
          "message": "Perf: measure every commit against a pinned baseline on the same runner [skip ci]\n\nubuntu-latest runners come in CPUs 1.3-1.6x apart, which raised a false 1.54x alert on a docs-only commit. ci/perf.py (was perf_backfill.py) now builds the commit and baseline 179e6b2 from today's perf_ci.rs, runs them alternately three times on the same runner, and reports the commit's time as a percentage of the baseline's, with absolute us/word, word errors and the runner CPU in extra. The alert threshold drops from 150% to 115%. The backfill uses the same method.",
          "timestamp": "2026-11-26T00:36:24-08:00",
          "tree_id": "6a04c91592717a5a4824e6eb7246c08d82b169d8",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/fee9cfbfb1ba5016178f9e068d3bb31dec8511a7"
        },
        "date": 1795771162641,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.548,
            "extra": "100.0% of baseline (raw 12.89 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.852,
            "extra": "100.0% of baseline (raw 820.47 vs 820.69 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.192,
            "extra": "100.2% of baseline (raw 20.84 vs 20.82 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.169,
            "extra": "100.1% of baseline (raw 13.36 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.121,
            "extra": "99.8% of baseline (raw 42.04 vs 42.10 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.596,
            "extra": "100.0% of baseline (raw 265.73 vs 265.44 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.899,
            "extra": "98.8% of baseline (raw 32.70 vs 32.91 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
          "id": "faf74730677ad62a4a54ea4310f05e2eb3809eba",
          "message": "Polish README badges, scratch methods, cheap first pass, and contributing link",
          "timestamp": "2026-11-26T15:59:44-08:00",
          "tree_id": "57820459e3592c3ae12a19dc24aea53fcf5a8cdf",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/faf74730677ad62a4a54ea4310f05e2eb3809eba"
        },
        "date": 1795771257202,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.543,
            "extra": "99.9% of baseline (raw 12.88 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.379,
            "extra": "100.1% of baseline (raw 820.12 vs 820.17 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.107,
            "extra": "99.7% of baseline (raw 20.80 vs 20.85 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.155,
            "extra": "100.0% of baseline (raw 13.36 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.1,
            "extra": "99.8% of baseline (raw 41.95 vs 42.12 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.85,
            "extra": "100.1% of baseline (raw 265.98 vs 265.61 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 21.77,
            "extra": "98.2% of baseline (raw 32.47 vs 32.88 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
          "id": "25ceb05ec1653eeb422ec02dd01c7f80fecc1798",
          "message": "Perf: chart normalized us/word and stabilize CI benchmark runs",
          "timestamp": "2026-11-26T15:59:44-08:00",
          "tree_id": "aa3f277968da078cb04c73c170c2a773c453f58b",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/25ceb05ec1653eeb422ec02dd01c7f80fecc1798"
        },
        "date": 1795771351649,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.548,
            "extra": "100.0% of baseline (raw 12.89 vs 12.89 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 597.646,
            "extra": "99.9% of baseline (raw 819.73 vs 820.43 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.164,
            "extra": "100.0% of baseline (raw 20.86 vs 20.88 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.162,
            "extra": "100.0% of baseline (raw 13.36 vs 13.36 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.23,
            "extra": "100.2% of baseline (raw 42.06 vs 41.95 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.959,
            "extra": "100.1% of baseline (raw 265.95 vs 265.73 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.375,
            "extra": "101.0% of baseline (raw 32.73 vs 32.31 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
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
          "id": "dc7b4dedacc411c4facffd281617ec92531b77eb",
          "message": "Perf: pin runs to core 1, note REF_BASE_US next to PERF_BASELINE\n\nCore 0 handles most interrupts on Linux runners, so pin perf_ci to core 1 (core 0\non a single-core machine). Changing PERF_BASELINE also requires updating\nREF_BASE_US; say so in perf.yml. Ignore ci/__pycache__/.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-27T00:07:23-08:00",
          "tree_id": "040bd2e8212258b16c1b88f100fc395a6c3d5d98",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/dc7b4dedacc411c4facffd281617ec92531b77eb"
        },
        "date": 1795771445997,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "us/word",
            "value": 9.556,
            "extra": "100.1% of baseline (raw 12.89 vs 12.88 us/word); 0 / 20000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "us/word",
            "value": 598.057,
            "extra": "100.0% of baseline (raw 820.44 vs 820.38 us/word); 0 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "us/word",
            "value": 16.166,
            "extra": "100.0% of baseline (raw 20.84 vs 20.80 us/word); 25 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "us/word",
            "value": 10.146,
            "extra": "99.9% of baseline (raw 13.35 vs 13.37 us/word); 1 / 4000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "us/word",
            "value": 32.197,
            "extra": "100.1% of baseline (raw 41.93 vs 41.91 us/word); 0 / 2000 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "us/word",
            "value": 193.682,
            "extra": "100.0% of baseline (raw 265.63 vs 265.47 us/word); 1 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "us/word",
            "value": 22.273,
            "extra": "100.5% of baseline (raw 32.89 vs 32.72 us/word); 8 / 500 word errors; INTEL(R) XEON(R) PLATINUM 8573C; rustc 1.99.0 (b940084d7 2026-09-28)"
          }
        ]
      }
    ]
  }
}
