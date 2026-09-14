PS C:\Users\huang> cd  C:\repositories\rust\rust-72-changes\src\practice
PS C:\repositories\rust\rust-72-changes\src\practice> sqlite3.exe .\electricity_data.db
SQLite version 3.53.2 2026-06-03 19:12:13
Enter ".help" for usage hints.
sqlite> .databases
main: C:\repositories\rust\rust-72-changes\src\practice\electricity_data.db r/w
sqlite> .tables
electricity_records
sqlite> .schema electricity_records
CREATE TABLE electricity_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    record_date TEXT NOT NULL,
    room TEXT NOT NULL,
    payable REAL,
    paid REAL
);
CREATE INDEX idx_date ON electricity_records(record_date);
CREATE INDEX idx_room ON electricity_records(room);
sqlite>select count(1) from electricity_records;

select count(1) from

update electricity_records set room = 'LCY' where room = '李**';


http://localhost:3000/practice/electricity_query_sqlite.html


sqlite> BEGIN EXCLUSIVE TRANSACTION;
sqlite>  update electricity_records set room = 'LCY' where room = '李**';
sqlite> commit;

BEGIN;
update electricity_records set room = 'CBN' where room = '广电';
COMMIT;


SELECT CURRENT_TIMESTAMP;

-- 获取完整的本地日期和时间
SELECT datetime('now', 'localtime');


-- [SQLite 教程](https://www.runoob.com/sqlite/sqlite-tutorial.html)
