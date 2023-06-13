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

### Note
If you are running it behind nginx, then you should add this so it reflects the domain and the port and not `localhost:port`
```nginx
proxy_set_header Host $host;
proxy_set_header X-Real-IP $remote_addr;
proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
```
