import fastapi
import sqlite3
import uuid
import db as database
from dotenv import load_dotenv
load_dotenv()
import os
ext = fastapi.APIRouter()
db: database.Database = None

@ext.on_event("startup")
async def start():
    global db
    if os.getenv("DB_TYPE") == "POSTGRES":
        db = database.Database(database.DatabaseType.POSTGRES, host=os.getenv("DB_HOST"), user=os.getenv("DB_USER"), password=os.getenv("DB_PASSWORD"), database=os.getenv("DB_NAME"), port=int(os.getenv("DB_PORT")))
    elif os.getenv("DB_TYPE") == "SQLITE":
        db = database.Database(database.DatabaseType.SQLITE, os.getenv("DB_FILE"))
    else:
        raise ValueError("Invalid database type!")
    await db.connect()
    await db.execute("""
CREATE TABLE IF NOT EXISTS redirect (
    id TEXT NOT NULL,
    url TEXT NOT NULL,
    accessed BIGINT NOT NULL
)
               """)
    print(await db.fetch("SELECT * FROM redirect"))
    
@ext.get("/add")
async def add(url: str, request: fastapi.Request) -> fastapi.responses.PlainTextResponse:
    
    """
    Add a redirect URL to the database. Returns the URL to the redirect. (Might support discord embeds in the future)
    """
    
    id = str(uuid.uuid4())
    con = await db.fetch("SELECT url, id FROM redirect WHERE url=?", (url,))
    if len(con) != 0:
        return fastapi.responses.PlainTextResponse(str(request.base_url) + con[0][1])
    try:
        await db.execute("""
    INSERT INTO redirect(
        id,
        url,
        accessed
    ) VALUES (
        ?,?,?
    )
                """, (id,url,0))
        await db.commit()
    except sqlite3.OperationalError:
        await db.rollback()
        return fastapi.responses.PlainTextResponse("Error: Failed to save to database",500)
    return fastapi.responses.PlainTextResponse(str(request.base_url) + id)

@ext.get("/delete/{id:str}")
async def delete(id: str) -> fastapi.responses.PlainTextResponse:
    """
    Delete a redirect URL from the database. Returns a success message.
    """
    try:
        await db.execute("""
DELETE FROM redirect WHERE id = ?
                   """, (id,))
        await db.commit()
    except (sqlite3.OperationalError):
        await db.rollback()
        return fastapi.responses.PlainTextResponse("No redirect url with that ID found!"), 500
    
    return fastapi.responses.PlainTextResponse("Success")

@ext.get("/modify/{id:str}")
async def modify(id: str, url: str) -> fastapi.responses.PlainTextResponse:
    """
    Modify a redirect URL in the database. Returns a success message.
    """
    try:
        await db.execute(
            "UPDATE redirect SET url = ? WHERE id = ?",
            (url, id)
        )
        await db.commit()
    except sqlite3.OperationalError:
        await db.rollback()
        return fastapi.responses.PlainTextResponse("Failed to change the URL!")
    return fastapi.responses.PlainTextResponse("Success")

@ext.get("/info/{id:str}")
async def info(id: str) -> fastapi.responses.PlainTextResponse:
    """
    Info about a redirect URL from the database. Returns the URL and the number of times it has been accessed.
    """
    con = await db.fetch("SELECT url, accessed FROM redirect WHERE id = ?", (id,))
    if len(con) == 0:
        return fastapi.responses.PlainTextResponse("No redirect with that UUID4 found!")
    url, access_num = con[0]
    return fastapi.responses.PlainTextResponse(f"URL: {url}\nAccessed: {access_num} times")



@ext.get("/list")
async def listing() -> fastapi.responses.PlainTextResponse:
    """
    List all redirect URLS with it's ID
    """
    con = await db.fetch("SELECT * FROM redirect")
    print(con)
    if len(con) == 0:
        return fastapi.responses.PlainTextResponse("No redirect URLs has been added yet.",404)
    text = "List of redirect URLs recorded in database.\n"
    for id, url, _ in con:
        text += f"URL: {url}, ID: {id}\n"
    return fastapi.responses.PlainTextResponse(text)
    
@ext.get(
    "/{id:str}"
)
async def access(id: str):
    """
    Access a redirect URL from the database. Returns a redirect to the URL.
    """
    con = await db.fetch("SELECT url, accessed FROM redirect WHERE id = ?", (id,))
    if len(con) == 0:
        return fastapi.responses.PlainTextResponse("No redirect with that UUID4 found!")
    url, access_num = con[0]
    access_num += 1
    await db.execute("UPDATE redirect SET accessed = ? WHERE id = ?", (access_num, id))
    await db.commit()
    return fastapi.responses.RedirectResponse(url)

@ext.get("/info/{id:str}")
async def info(id: str) -> fastapi.responses.PlainTextResponse:
    """
    Info about a redirect URL from the database. Returns the URL and the number of times it has been accessed.
    """
    con = await db.fetch("SELECT url, accessed FROM redirect WHERE id = ?", (id,))
    if len(con) == 0:
        return fastapi.responses.PlainTextResponse("No redirect with that UUID4 found!")
    url, access_num = con[0]
    return fastapi.responses.PlainTextResponse(f"URL: {url}\nAccessed: {access_num} times")

