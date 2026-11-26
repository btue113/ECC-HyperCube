window.BENCHMARK_DATA = {
  "lastUpdate": 1795673825400,
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
          "id": "606b78af367a97cd386820ffc5010c3fbab02126",
          "message": "Add per-commit perf tracking: fixed decode workloads charted on GitHub Pages\n\nexamples/perf_ci.rs times fixed single-thread decode workloads (hard, SCL, ensemble, GLDPC) and writes customSmallerIsBetter JSON; .github/workflows/perf.yml runs it on every push to main and appends the results to a chart on gh-pages (/dev/bench/).",
          "timestamp": "2026-11-25T21:46:49-08:00",
          "tree_id": "6d849e1639ff39648ece4695a7edcf0dc8d0cb2d",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/606b78af367a97cd386820ffc5010c3fbab02126"
        },
        "date": 1795672047736,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "value": 12.565,
            "unit": "us/word",
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "value": 856.79,
            "unit": "us/word",
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "value": 21.224,
            "unit": "us/word",
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "value": 13.329,
            "unit": "us/word",
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "value": 41.823,
            "unit": "us/word",
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "value": 253.43,
            "unit": "us/word",
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "value": 32.747,
            "unit": "us/word",
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Tue Bui",
            "username": "btue113",
            "email": "bomot113@gmail.com"
          },
          "committer": {
            "name": "Tue Bui",
            "username": "btue113",
            "email": "bomot113@gmail.com"
          },
          "id": "606b78af367a97cd386820ffc5010c3fbab02126",
          "message": "Add per-commit perf tracking: fixed decode workloads charted on GitHub Pages\n\nexamples/perf_ci.rs times fixed single-thread decode workloads (hard, SCL, ensemble, GLDPC) and writes customSmallerIsBetter JSON; .github/workflows/perf.yml runs it on every push to main and appends the results to a chart on gh-pages (/dev/bench/).",
          "timestamp": "2026-11-26T05:46:49Z",
          "url": "https://github.com/btue113/ECC-HyperCube/commit/606b78af367a97cd386820ffc5010c3fbab02126"
        },
        "date": 1795672547533,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "value": 12.524,
            "unit": "us/word",
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "value": 856.597,
            "unit": "us/word",
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "value": 21.108,
            "unit": "us/word",
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "value": 13.459,
            "unit": "us/word",
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "value": 41.722,
            "unit": "us/word",
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "value": 253.068,
            "unit": "us/word",
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "value": 33.018,
            "unit": "us/word",
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
        "date": 1795673825047,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "hard decode (255,163) 7 flips",
            "value": 15.265,
            "unit": "us/word",
            "extra": "0 / 20000 word errors; CubeCode::new(8, 3)"
          },
          {
            "name": "hard decode (4095,2510) 31 flips",
            "value": 1038.629,
            "unit": "us/word",
            "extra": "0 / 500 word errors; CubeCode::new(12, 5)"
          },
          {
            "name": "RM(3,7) (128,64) SCL-8 3.0 dB",
            "value": 25.236,
            "unit": "us/word",
            "extra": "25 / 4000 word errors"
          },
          {
            "name": "RM(3,7) (128,64) ensemble 8x8 3.0 dB",
            "value": 15.768,
            "unit": "us/word",
            "extra": "1 / 4000 word errors"
          },
          {
            "name": "RM(4,8) (256,163) ensemble 8x8 3.5 dB",
            "value": 49.821,
            "unit": "us/word",
            "extra": "0 / 2000 word errors"
          },
          {
            "name": "RM(4,9) (512,256) ensemble 4x16 3.0 dB",
            "value": 298.424,
            "unit": "us/word",
            "extra": "1 / 500 word errors"
          },
          {
            "name": "GLDPC (1024,513) 2.0 dB",
            "value": 42.081,
            "unit": "us/word",
            "extra": "8 / 500 word errors; 50 iterations max"
          }
        ]
      }
    ]
  }
}