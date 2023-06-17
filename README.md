# timelessnesses.api.redirect
A SQLite3/PostgreSQL based redirector built with Python and FastAPI and Uvicorn.

## Running this up
### Locally hosting (Manual)
#### Requirement
1. Python 3.11 (I used feature that isn't exist in old python)
2. Any build essentials for any manual compiling modules (if you are in debian based distros then you may not need this)
#### Steps
1. Install poetry through pip
2. `poetry install`
3. Edit .env by renaming any of the .env templates or just copy this example.

```sh
DB_TYPE=POSTGRES or SQLITE
DB_FILE=path to save the sqlite file. for sqlite only
DB_HOST=database host. postgres only
DB_PORT=database port. postgres only
DB_USER=username to login. postgres only
DB_PASSWORD=password to login. postgres only
DB_NAME=database name to connect to. postgres only
```

4. `poetry run uvicorn main:app`

### Locally hosting (Docker compose)

#### Requirement

1. Docker
2. Docker compose extension

#### Steps

1. `docker compose up -d`
2. Profit  
Docker going to use PostgreSQL by default.

## Performance

PostgreSQL

```shell
Summary:
  Success rate:	1.0000
  Total:	10.0014 secs
  Slowest:	0.5746 secs
  Fastest:	0.0167 secs
  Average:	0.0851 secs
  Requests/sec:	585.2174

  Total data:	194.34 KiB
  Size/request:	34 B
  Size/sec:	19.43 KiB

Response time histogram:
  0.017 [1]    |
  0.072 [2039] |■■■■■■■■■■■■■■■■■■■■■■■
  0.128 [2764] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.184 [741]  |■■■■■■■■
  0.240 [205]  |■■
  0.296 [74]   |
  0.351 [20]   |
  0.407 [4]    |
  0.463 [3]    |
  0.519 [1]    |
  0.575 [1]    |

Latency distribution:
  10% in 0.0262 secs
  25% in 0.0349 secs
  50% in 0.0794 secs
  75% in 0.1057 secs
  90% in 0.1441 secs
  95% in 0.1854 secs
  99% in 0.2623 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0033 secs, 0.0016 secs, 0.0046 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 5853 responses

```

SQLite

```shell
Summary:
  Success rate:	1.0000
  Total:	10.0005 secs
  Slowest:	0.1707 secs
  Fastest:	0.0021 secs
  Average:	0.0597 secs
  Requests/sec:	836.0558

  Total data:	277.61 KiB
  Size/request:	34 B
  Size/sec:	27.76 KiB

Response time histogram:
  0.002 [1]    |
  0.019 [293]  |■■
  0.036 [731]  |■■■■■■■
  0.053 [2226] |■■■■■■■■■■■■■■■■■■■■■■
  0.070 [3131] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.086 [951]  |■■■■■■■■■
  0.103 [723]  |■■■■■■■
  0.120 [121]  |■
  0.137 [144]  |■
  0.154 [8]    |
  0.171 [32]   |

Latency distribution:
  10% in 0.0316 secs
  25% in 0.0487 secs
  50% in 0.0563 secs
  75% in 0.0678 secs
  90% in 0.0886 secs
  95% in 0.0982 secs
  99% in 0.1289 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0012 secs, 0.0001 secs, 0.0053 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 8361 responses
```

## Note
If you are running it behind nginx, then you should add this so it reflects the domain and the port and not `localhost:port`
```nginx
proxy_set_header Host $host;
proxy_set_header X-Real-IP $remote_addr;
proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
```
