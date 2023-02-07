window.BENCHMARK_DATA = {
  "lastUpdate": 1675800337550,
  "repoUrl": "https://github.com/alancai98/partiql-lang-rust",
  "entries": {
    "PartiQL (rust) Benchmark": [
      {
        "commit": {
          "author": {
            "email": "27716912+am357@users.noreply.github.com",
            "name": "Arash Maymandi",
            "username": "am357"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "48b6d896c889c1e547f387e5ea85565bc3f9237c",
          "message": "Add serde to partiql-logical (#290)\n\nAdds serde w/ Serialize and Deserialize traits to partiql-logical.",
          "timestamp": "2023-02-01T16:29:55-08:00",
          "tree_id": "b89dc94acee56f52c37157f600faa4efcd0ea112",
          "url": "https://github.com/alancai98/partiql-lang-rust/commit/48b6d896c889c1e547f387e5ea85565bc3f9237c"
        },
        "date": 1675800336730,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse-1",
            "value": 5608,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "parse-15",
            "value": 52810,
            "range": "± 265",
            "unit": "ns/iter"
          },
          {
            "name": "parse-30",
            "value": 104911,
            "range": "± 691",
            "unit": "ns/iter"
          },
          {
            "name": "compile-1",
            "value": 16471,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "compile-15",
            "value": 48907,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "compile-30",
            "value": 88405,
            "range": "± 681",
            "unit": "ns/iter"
          },
          {
            "name": "plan-1",
            "value": 19788,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "plan-15",
            "value": 366030,
            "range": "± 1042",
            "unit": "ns/iter"
          },
          {
            "name": "plan-30",
            "value": 740812,
            "range": "± 1710",
            "unit": "ns/iter"
          },
          {
            "name": "eval-1",
            "value": 23062858,
            "range": "± 182884",
            "unit": "ns/iter"
          },
          {
            "name": "eval-15",
            "value": 144956026,
            "range": "± 489065",
            "unit": "ns/iter"
          },
          {
            "name": "eval-30",
            "value": 281967144,
            "range": "± 1886010",
            "unit": "ns/iter"
          },
          {
            "name": "join",
            "value": 16543,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "simple",
            "value": 5471,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "simple-no",
            "value": 2308,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "numbers",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse-simple",
            "value": 650,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse-ion",
            "value": 2422,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse-group",
            "value": 8271,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "parse-complex",
            "value": 21823,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "parse-complex-fexpr",
            "value": 33457,
            "range": "± 101",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}