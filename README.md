# timelessnesses.api.redirect

A rust version of the python version.

## How to use

1. Build it or grab it from release.
2. Create .env with text below.
```properties
DB_TYPE=either POSTGRES or SQLITE3
SQLITE_PATH=SQLITE3 database file location
DB_HOST=POSTGRES database host
DB_PORT=POSTGRES database port
DB_USER=POSTGRES database username
DB_PASSWORD=POSTGRES database user password
DB_NAME=POSTGRES database name
```
3. Run the program

## Benchmark

### PostgreSQL

Read:
```sh
 timelessnesses@timelessnesses> oha -z 10s --rand-regex-url "http://localhost:8000/[a-z][a-z][a-z]"
Summary:
  Success rate: 100.00%
  Total:        10.0011 secs
  Slowest:      0.7697 secs
  Fastest:      0.0025 secs
  Average:      0.0365 secs
  Requests/sec: 1366.8512

  Total data:   600.73 KiB
  Size/request: 45 B
  Size/sec:     60.07 KiB

Response time histogram:
  0.002 [1]     |
  0.079 [12880] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.156 [656]   |■
  0.233 [33]    |
  0.309 [0]     |
  0.386 [0]     |
  0.463 [50]    |
  0.540 [0]     |
  0.616 [0]     |
  0.693 [0]     |
  0.770 [50]    |

Response time distribution:
  10% in 0.0145 secs
  25% in 0.0193 secs
  50% in 0.0271 secs
  75% in 0.0379 secs
  90% in 0.0566 secs
  95% in 0.0838 secs
  99% in 0.1540 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0051 secs, 0.0003 secs, 0.0072 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [400] 13670 responses
```

Write: 
```sh
 timelessnesses@timelessnesses> oha -z 10s --rand-regex-url "http://localhost:8000/add\?url=[a-z][a-z][a-z]"
Summary:
  Success rate: 100.00%
  Total:        10.0051 secs
  Slowest:      0.2297 secs
  Fastest:      0.0269 secs
  Average:      0.1279 secs
  Requests/sec: 386.8012

  Total data:   179.08 KiB
  Size/request: 47 B
  Size/sec:     17.90 KiB

Response time histogram:
  0.027 [1]    |
  0.047 [8]    |
  0.067 [151]  |■■■
  0.088 [947]  |■■■■■■■■■■■■■■■■■■■■■■■■
  0.108 [137]  |■■■
  0.128 [93]   |■■
  0.149 [1027] |■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.169 [1214] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.189 [215]  |■■■■■
  0.209 [61]   |■
  0.230 [16]   |

Response time distribution:
  10% in 0.0727 secs
  25% in 0.0832 secs
  50% in 0.1416 secs
  75% in 0.1568 secs
  90% in 0.1658 secs
  95% in 0.1739 secs
  99% in 0.1972 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0024 secs, 0.0003 secs, 0.0036 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 3870 responses
```

## SQLite3

Read: 
```sh
 timelessnesses@timelessnesses> oha -z 10s --rand-regex-url "http://localhost:8000/[a-z][a-z][a-z]"
Summary:
  Success rate: 100.00%
  Total:        10.0017 secs
  Slowest:      0.2856 secs
  Fastest:      0.0007 secs
  Average:      0.0191 secs
  Requests/sec: 2602.8498

  Total data:   1.17 MiB
  Size/request: 46 B
  Size/sec:     119.47 KiB

Response time histogram:
  0.001 [1]     |
  0.029 [22812] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.058 [2128]  |■■
  0.086 [577]   |
  0.115 [240]   |
  0.143 [182]   |
  0.172 [43]    |
  0.200 [0]     |
  0.229 [0]     |
  0.257 [0]     |
  0.286 [50]    |

Response time distribution:
  10% in 0.0057 secs
  25% in 0.0094 secs
  50% in 0.0141 secs
  75% in 0.0204 secs
  90% in 0.0329 secs
  95% in 0.0522 secs
  99% in 0.1181 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0033 secs, 0.0007 secs, 0.0052 secs
  DNS-lookup:   0.0001 secs, 0.0000 secs, 0.0023 secs

Status code distribution:
  [404] 26032 responses
  [400] 1 responses
```

Write:
```sh
 timelessnesses@timelessnesses> oha -z 10s --rand-regex-url "http://localhost:8000/add\?url=[a-z][a-z][a-z]"
Summary:
  Success rate: 100.00%
  Total:        10.0009 secs
  Slowest:      0.4208 secs
  Fastest:      0.0172 secs
  Average:      0.2878 secs
  Requests/sec: 170.9840

  Total data:   96.86 KiB
  Size/request: 58 B
  Size/sec:     9.68 KiB

Response time histogram:
  0.017 [1]   |
  0.058 [8]   |
  0.098 [12]  |
  0.138 [97]  |■■■■
  0.179 [164] |■■■■■■■
  0.219 [96]  |■■■■
  0.259 [22]  |
  0.300 [189] |■■■■■■■■
  0.340 [744] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.380 [318] |■■■■■■■■■■■■■
  0.421 [59]  |■■

Response time distribution:
  10% in 0.1512 secs
  25% in 0.2799 secs
  50% in 0.3151 secs
  75% in 0.3363 secs
  90% in 0.3580 secs
  95% in 0.3724 secs
  99% in 0.4043 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0098 secs, 0.0077 secs, 0.0126 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 1710 responses
```