// Run scripts/create_database_demo_dbf.sh first.
// The demo lists and updates both records in examples/database_demo.dbf.
USE "examples/database_demo.dbf"
DBGOTOP
WHILE !DBEOF()
    DBLIST
    SKIP 1
ENDDO
