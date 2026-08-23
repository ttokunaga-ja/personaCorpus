#!/usr/bin/env python3
"""Run a synthetic aggregate-only SQLite query with a read-only guard."""
from __future__ import annotations
import sqlite3

SAMPLE = [("Online", 46_300_000), ("Stores", 37_900_000)]

def run(query: str) -> list[tuple[object, ...]]:
    forbidden = ("insert", "update", "delete", "drop", "attach", "pragma")
    if any(word in query.lower() for word in forbidden):
        raise ValueError("only read-only synthetic SELECT queries are permitted")
    connection = sqlite3.connect(":memory:")
    connection.execute("create table channel(channel text, gmv_yen integer)")
    connection.executemany("insert into channel values (?, ?)", SAMPLE)
    return connection.execute(query).fetchall()

if __name__ == "__main__":
    print(run("select channel, gmv_yen from channel order by gmv_yen desc"))
