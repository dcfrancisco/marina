# Modern Database Backends

Clipper-2025 allows **multiple backend engines**, optional.

## PostgreSQL Backend (SQL) - Future

```clipper
pg := postgres.connect("dbname=test user=danny")
rows := pg.query("SELECT * FROM customer WHERE age > 21")

for row in rows
    Print(row["name"], row["age"])
next
```

Unified DB API allows you to switch engines with minimal code changes.

## MongoDB Backend (NoSQL) - Future

Yes — fully possible.

```clipper
mongo := mongodb.connect("mongodb://localhost:27017")
coll := mongo.collection("customer")

result := coll.find({ "age": { "$gt": 21 } })

for rec in result
    Print(rec["name"], rec["age"])
next
```

Clipper code + NoSQL.
Beautiful combination.

## Engine Abstraction Layer

To support DBF, SQL, and NoSQL:

```
DBEngine
  - open()
  - close()
  - select()
  - insert()
  - update()
  - delete()
  - query()
```

DBF implements it.
Postgres implements it.
Mongo implements it.
SQLite implements it.

Your Clipper-2025 code becomes:

```
db := engine.open("customer")
```

Engine selection is configurable via:

```
config.toml
module.json
runtime flag
connection string
```

## Why This Is Powerful

Clipper-2025 becomes:

* **local database system**
* **embedded database system**
* **SQL client**
* **NoSQL client**
* **hybrid modern DB engine**

Clipper programmers get:

* DBF for single-user or local apps
* Postgres for enterprise
* MongoDB for analytics or document apps
* SQLite for mobile apps
* S3/JSON future backends

All using the same high-level patterns.
