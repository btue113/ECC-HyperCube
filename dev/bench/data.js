window.BENCHMARK_DATA = {
  "lastUpdate": 1795683741118,
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
        "date": 1795682259466,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 184.27,
            "extra": "17.84 us/word (baseline 9.68); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 471.81,
            "extra": "2852.76 us/word (baseline 604.63); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682309398,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 187.16,
            "extra": "18.13 us/word (baseline 9.69); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 471.43,
            "extra": "2837.19 us/word (baseline 601.83); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682358488,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 187.59,
            "extra": "17.54 us/word (baseline 9.35); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 472.59,
            "extra": "2745.84 us/word (baseline 581.02); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682407317,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 186.55,
            "extra": "17.25 us/word (baseline 9.25); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 473.87,
            "extra": "2747.21 us/word (baseline 579.74); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682455896,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 185.75,
            "extra": "17.32 us/word (baseline 9.33); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 471.44,
            "extra": "2748.98 us/word (baseline 583.10); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682505525,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 190.18,
            "extra": "17.98 us/word (baseline 9.45); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 469.01,
            "extra": "2830.80 us/word (baseline 603.57); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682554160,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 183.91,
            "extra": "17.24 us/word (baseline 9.38); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 468.97,
            "extra": "2729.62 us/word (baseline 582.04); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682579109,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.56,
            "extra": "9.31 us/word (baseline 9.26); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.42,
            "extra": "579.58 us/word (baseline 577.17); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682618388,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.53,
            "extra": "9.33 us/word (baseline 9.19); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 99.1,
            "extra": "573.28 us/word (baseline 578.50); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 1186.4,
            "extra": "185.36 us/word (baseline 15.62); 26 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682647172,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 98.84,
            "extra": "9.57 us/word (baseline 9.68); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.78,
            "extra": "603.45 us/word (baseline 598.76); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 244.33,
            "extra": "39.50 us/word (baseline 16.17); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682696497,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 103.38,
            "extra": "9.63 us/word (baseline 9.31); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.49,
            "extra": "587.20 us/word (baseline 584.32); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 242.64,
            "extra": "38.20 us/word (baseline 15.74); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 478.71,
            "extra": "47.36 us/word (baseline 9.89); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 478.59,
            "extra": "153.26 us/word (baseline 32.02); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 625.72,
            "extra": "1206.73 us/word (baseline 192.85); 2 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682736238,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.93,
            "extra": "9.72 us/word (baseline 9.53); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.9,
            "extra": "595.15 us/word (baseline 589.83); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 132.51,
            "extra": "20.93 us/word (baseline 15.79); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 258.14,
            "extra": "25.90 us/word (baseline 10.03); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 256.22,
            "extra": "79.78 us/word (baseline 31.14); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 268.58,
            "extra": "519.65 us/word (baseline 193.48); 2 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682776820,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 105.29,
            "extra": "9.77 us/word (baseline 9.28); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 99.91,
            "extra": "599.06 us/word (baseline 599.61); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 126.5,
            "extra": "20.22 us/word (baseline 15.99); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 251.32,
            "extra": "25.51 us/word (baseline 10.15); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 249.47,
            "extra": "79.49 us/word (baseline 31.86); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 317.83,
            "extra": "613.69 us/word (baseline 193.09); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682817258,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 102.94,
            "extra": "9.63 us/word (baseline 9.36); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 96.48,
            "extra": "580.87 us/word (baseline 602.07); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 96.99,
            "extra": "15.92 us/word (baseline 16.42); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 193.4,
            "extra": "19.94 us/word (baseline 10.31); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 198.77,
            "extra": "64.46 us/word (baseline 32.43); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 269.36,
            "extra": "528.82 us/word (baseline 196.32); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682854488,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.65,
            "extra": "9.46 us/word (baseline 9.31); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 101.11,
            "extra": "588.00 us/word (baseline 581.52); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 97.11,
            "extra": "15.61 us/word (baseline 16.08); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 189.72,
            "extra": "19.01 us/word (baseline 10.02); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 151.22,
            "extra": "46.95 us/word (baseline 31.05); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 139.84,
            "extra": "262.39 us/word (baseline 187.64); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682892040,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 102.0,
            "extra": "9.57 us/word (baseline 9.38); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 98.85,
            "extra": "582.19 us/word (baseline 588.95); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.7,
            "extra": "15.89 us/word (baseline 16.10); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 186.55,
            "extra": "19.05 us/word (baseline 10.21); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 147.98,
            "extra": "47.46 us/word (baseline 32.07); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 135.03,
            "extra": "261.60 us/word (baseline 193.74); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682929754,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.66,
            "extra": "9.42 us/word (baseline 9.35); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 99.57,
            "extra": "580.28 us/word (baseline 582.81); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.92,
            "extra": "15.91 us/word (baseline 15.93); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 192.55,
            "extra": "19.26 us/word (baseline 10.00); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 148.69,
            "extra": "47.22 us/word (baseline 31.76); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 136.83,
            "extra": "263.12 us/word (baseline 192.30); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795682967612,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.12,
            "extra": "9.38 us/word (baseline 9.36); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.03,
            "extra": "587.99 us/word (baseline 587.80); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 103.26,
            "extra": "16.23 us/word (baseline 15.72); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 194.04,
            "extra": "19.50 us/word (baseline 10.05); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 148.98,
            "extra": "47.61 us/word (baseline 31.96); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 141.83,
            "extra": "268.35 us/word (baseline 189.20); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683005651,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 99.04,
            "extra": "9.55 us/word (baseline 9.65); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 98.49,
            "extra": "588.99 us/word (baseline 598.04); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.7,
            "extra": "16.12 us/word (baseline 16.17); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 198.2,
            "extra": "19.94 us/word (baseline 10.06); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 149.37,
            "extra": "48.44 us/word (baseline 32.43); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 139.45,
            "extra": "270.00 us/word (baseline 193.62); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683044470,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.44,
            "extra": "9.75 us/word (baseline 9.61); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 98.44,
            "extra": "592.08 us/word (baseline 601.49); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 101.27,
            "extra": "16.45 us/word (baseline 16.25); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 191.96,
            "extra": "19.84 us/word (baseline 10.34); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 149.97,
            "extra": "48.41 us/word (baseline 32.28); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 137.0,
            "extra": "267.59 us/word (baseline 195.31); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683083420,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.64,
            "extra": "9.77 us/word (baseline 9.61); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 99.48,
            "extra": "597.07 us/word (baseline 600.22); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.24,
            "extra": "16.13 us/word (baseline 16.25); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 190.05,
            "extra": "19.27 us/word (baseline 10.14); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 150.56,
            "extra": "47.58 us/word (baseline 31.60); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 140.26,
            "extra": "265.75 us/word (baseline 189.47); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 105.41,
            "extra": "22.43 us/word (baseline 21.28); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683122520,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.84,
            "extra": "9.82 us/word (baseline 9.64); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 101.35,
            "extra": "604.63 us/word (baseline 596.56); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.28,
            "extra": "16.11 us/word (baseline 16.39); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 189.54,
            "extra": "19.43 us/word (baseline 10.25); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 147.96,
            "extra": "48.16 us/word (baseline 32.55); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 136.5,
            "extra": "266.60 us/word (baseline 195.32); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 101.38,
            "extra": "22.39 us/word (baseline 22.09); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683161312,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.64,
            "extra": "9.64 us/word (baseline 9.49); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 97.39,
            "extra": "587.16 us/word (baseline 602.89); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.6,
            "extra": "16.05 us/word (baseline 16.11); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 187.64,
            "extra": "19.09 us/word (baseline 10.18); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 144.95,
            "extra": "46.76 us/word (baseline 32.26); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 138.76,
            "extra": "266.47 us/word (baseline 192.03); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.59,
            "extra": "22.12 us/word (baseline 22.21); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683200195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 104.16,
            "extra": "9.68 us/word (baseline 9.29); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 98.14,
            "extra": "588.36 us/word (baseline 599.49); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.49,
            "extra": "16.00 us/word (baseline 16.09); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 194.11,
            "extra": "19.69 us/word (baseline 10.15); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 153.0,
            "extra": "48.34 us/word (baseline 31.59); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 136.96,
            "extra": "260.71 us/word (baseline 190.35); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 102.19,
            "extra": "21.97 us/word (baseline 21.50); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683238679,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 98.78,
            "extra": "9.50 us/word (baseline 9.62); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.48,
            "extra": "597.18 us/word (baseline 594.32); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 102.03,
            "extra": "16.02 us/word (baseline 15.71); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 191.19,
            "extra": "19.39 us/word (baseline 10.14); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 146.63,
            "extra": "47.18 us/word (baseline 32.18); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 133.5,
            "extra": "256.43 us/word (baseline 192.09); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.6,
            "extra": "21.74 us/word (baseline 21.83); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683277588,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.4,
            "extra": "9.73 us/word (baseline 9.60); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.86,
            "extra": "605.64 us/word (baseline 600.47); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.45,
            "extra": "16.19 us/word (baseline 16.28); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 192.95,
            "extra": "19.58 us/word (baseline 10.15); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 150.0,
            "extra": "48.19 us/word (baseline 32.12); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 137.7,
            "extra": "266.96 us/word (baseline 193.88); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.22,
            "extra": "22.08 us/word (baseline 22.26); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683316345,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.86,
            "extra": "9.78 us/word (baseline 9.60); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 101.23,
            "extra": "603.77 us/word (baseline 596.44); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.74,
            "extra": "16.40 us/word (baseline 16.44); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 193.54,
            "extra": "19.89 us/word (baseline 10.28); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 152.0,
            "extra": "48.76 us/word (baseline 32.08); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 138.02,
            "extra": "269.43 us/word (baseline 195.21); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 98.26,
            "extra": "21.85 us/word (baseline 22.24); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683355125,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.96,
            "extra": "9.74 us/word (baseline 9.55); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.4,
            "extra": "603.14 us/word (baseline 600.73); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.97,
            "extra": "16.21 us/word (baseline 16.38); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 192.43,
            "extra": "19.67 us/word (baseline 10.22); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 149.87,
            "extra": "48.35 us/word (baseline 32.26); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 137.1,
            "extra": "266.74 us/word (baseline 194.55); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 101.18,
            "extra": "22.42 us/word (baseline 22.15); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683393934,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.0,
            "extra": "9.57 us/word (baseline 9.57); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 99.81,
            "extra": "593.43 us/word (baseline 594.55); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 100.9,
            "extra": "16.30 us/word (baseline 16.16); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 193.56,
            "extra": "19.67 us/word (baseline 10.16); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 150.38,
            "extra": "48.21 us/word (baseline 32.06); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 139.51,
            "extra": "269.12 us/word (baseline 192.91); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 102.63,
            "extra": "22.48 us/word (baseline 21.91); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683432985,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.8,
            "extra": "9.72 us/word (baseline 9.64); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.14,
            "extra": "603.28 us/word (baseline 602.42); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 101.52,
            "extra": "16.27 us/word (baseline 16.03); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 194.57,
            "extra": "19.82 us/word (baseline 10.19); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 149.21,
            "extra": "48.07 us/word (baseline 32.22); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 137.33,
            "extra": "267.82 us/word (baseline 195.02); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.22,
            "extra": "22.19 us/word (baseline 22.36); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683472139,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.01,
            "extra": "9.74 us/word (baseline 9.64); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 102.46,
            "extra": "606.79 us/word (baseline 592.24); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 101.09,
            "extra": "16.34 us/word (baseline 16.16); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 196.27,
            "extra": "19.82 us/word (baseline 10.10); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 152.31,
            "extra": "48.99 us/word (baseline 32.17); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 139.55,
            "extra": "268.85 us/word (baseline 192.65); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 100.05,
            "extra": "22.11 us/word (baseline 22.09); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683512140,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 101.95,
            "extra": "9.89 us/word (baseline 9.70); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.87,
            "extra": "608.95 us/word (baseline 603.68); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.85,
            "extra": "16.24 us/word (baseline 16.27); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 191.51,
            "extra": "19.69 us/word (baseline 10.28); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 150.32,
            "extra": "48.83 us/word (baseline 32.48); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 138.22,
            "extra": "268.55 us/word (baseline 194.29); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 98.81,
            "extra": "22.15 us/word (baseline 22.41); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683552025,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.2,
            "extra": "9.74 us/word (baseline 9.72); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.34,
            "extra": "601.08 us/word (baseline 599.02); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 100.26,
            "extra": "16.38 us/word (baseline 16.34); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 195.98,
            "extra": "19.69 us/word (baseline 10.05); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 148.85,
            "extra": "48.46 us/word (baseline 32.56); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 136.58,
            "extra": "266.67 us/word (baseline 195.25); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 100.79,
            "extra": "22.28 us/word (baseline 22.11); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683589766,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 99.68,
            "extra": "9.72 us/word (baseline 9.75); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.14,
            "extra": "607.73 us/word (baseline 606.88); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.52,
            "extra": "16.34 us/word (baseline 16.42); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 100.34,
            "extra": "10.35 us/word (baseline 10.32); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 99.99,
            "extra": "32.47 us/word (baseline 32.47); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 100.37,
            "extra": "196.59 us/word (baseline 195.87); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.78,
            "extra": "22.22 us/word (baseline 22.27); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683627209,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 99.26,
            "extra": "9.74 us/word (baseline 9.81); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.18,
            "extra": "610.95 us/word (baseline 609.86); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.2,
            "extra": "16.64 us/word (baseline 16.78); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.62,
            "extra": "10.48 us/word (baseline 10.62); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 99.36,
            "extra": "33.18 us/word (baseline 33.39); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 99.85,
            "extra": "199.16 us/word (baseline 199.45); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 100.4,
            "extra": "22.32 us/word (baseline 22.23); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683665109,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 102.25,
            "extra": "9.72 us/word (baseline 9.50); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 101.46,
            "extra": "603.67 us/word (baseline 594.99); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 99.8,
            "extra": "16.23 us/word (baseline 16.27); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.83,
            "extra": "10.13 us/word (baseline 10.25); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 99.11,
            "extra": "31.93 us/word (baseline 32.21); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 96.9,
            "extra": "190.87 us/word (baseline 196.98); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 98.84,
            "extra": "22.02 us/word (baseline 22.27); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683702519,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 100.57,
            "extra": "9.61 us/word (baseline 9.55); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 98.31,
            "extra": "587.88 us/word (baseline 597.99); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 100.16,
            "extra": "16.10 us/word (baseline 16.07); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 102.45,
            "extra": "10.27 us/word (baseline 10.02); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 100.4,
            "extra": "31.82 us/word (baseline 31.69); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 99.15,
            "extra": "190.09 us/word (baseline 191.72); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.48,
            "extra": "21.63 us/word (baseline 21.75); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
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
        "date": 1795683739815,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "unit": "% of baseline",
            "value": 98.74,
            "extra": "9.63 us/word (baseline 9.75); 0 / 20000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "unit": "% of baseline",
            "value": 100.07,
            "extra": "601.77 us/word (baseline 601.33); 0 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.63,
            "extra": "16.31 us/word (baseline 16.54); 25 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "unit": "% of baseline",
            "value": 98.14,
            "extra": "10.20 us/word (baseline 10.39); 1 / 4000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "unit": "% of baseline",
            "value": 98.39,
            "extra": "32.41 us/word (baseline 32.94); 0 / 2000 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "unit": "% of baseline",
            "value": 100.13,
            "extra": "195.30 us/word (baseline 195.05); 1 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "unit": "% of baseline",
            "value": 99.53,
            "extra": "22.06 us/word (baseline 22.16); 8 / 500 word errors; AMD EPYC 9V45 96-Core Processor"
          }
        ]
      }
    ]
  }
}
