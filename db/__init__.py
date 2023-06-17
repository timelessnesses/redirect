import enum
import sqlite3
import typing

import asyncpg


class DatabaseType(enum.Enum):
    SQLITE = 1
    POSTGRES = 2


class Database:
    db_type: DatabaseType
    args: tuple
    kwargs: dict
    connection: typing.Union[asyncpg.Pool, sqlite3.Connection]

    def __init__(self, db_type: DatabaseType = DatabaseType.SQLITE, *args, **kwargs):
        self.db_type = db_type
        self.args = args
        self.kwargs = kwargs
        self.connection = None

    async def connect(self):
        if self.db_type == DatabaseType.POSTGRES:
            self.connection = await asyncpg.create_pool(*self.args, **self.kwargs)
        elif self.db_type == DatabaseType.SQLITE:
            self.connection = sqlite3.connect(*self.args, **self.kwargs)

    async def disconnect(self):
        if self.db_type == DatabaseType.POSTGRES:
            await self.connection.close()
        elif self.db_type == DatabaseType.SQLITE:
            self.connection.close()

    async def execute(self, query: str, values: tuple = tuple()):
        query = query.strip()
        if self.db_type == DatabaseType.POSTGRES:
            await self.convert_sqlite3_to_postgres(query, values)
        else:
            await self.convert_postgres_to_sqlite3(query, values)

    async def convert_sqlite3_to_postgres(self, query: str, values: tuple):
        # convert sqlite3 to postgres syntax and it's values
        query = query.split("?")
        print(query)
        for i in range(len(query) - 1):
            query[i] += f"${i+1}"
        print("".join(query), values)
        async with self.connection.acquire() as connection:
            await connection.execute(" ".join(query), *values)

    async def convert_postgres_to_sqlite3(self, query: str, values: tuple):
        self.connection.execute(query, values)
        self.connection.commit()

    async def fetch(self, query: str, values: tuple = tuple()) -> list:
        query = query.strip()
        if self.db_type == DatabaseType.POSTGRES:
            return await self.convert_sqlite3_to_postgres_fetch(query, values)
        else:
            return await self.convert_postgres_to_sqlite3_fetch(query, values)

    async def convert_sqlite3_to_postgres_fetch(
        self, query: str, values: tuple
    ) -> list:
        query_s = query
        query = query.split("?")
        print(query)
        if query_s.count("?") != 0:
            del query[-1]
            for i in range(len(query)):
                query[i] += f"${i+1}"

        print(query)
        return await self.connection.fetch(" ".join(query), *values)

    async def convert_postgres_to_sqlite3_fetch(
        self, query: str, values: tuple
    ) -> list:
        cur: sqlite3.Cursor = self.connection.execute(query, values)
        return cur.fetchall()

    async def rollback(self):
        if self.db_type != DatabaseType.POSTGRES:
            self.connection.rollback()

    async def commit(self):
        if self.db_type != DatabaseType.POSTGRES:
            self.connection.commit()
