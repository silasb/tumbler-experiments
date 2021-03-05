# Tumble

```
RUST_LOG=debug cargo run --bin tumble-sql -- --config tumble.toml --source test.sql --dest test.sql.tumble
```

```
sqlite3
.read foo.txt.out
```


```
sqlite3
.mode csv
.import blah.100.csv trips
.output trips.sql
.dump
.quit
```


```
sqlite3
pragma compile_options;
CREATE TABLE IF NOT EXISTS countries (json_text json);
INSERT INTO countries VALUES ('{"name": "hello"}');
create index idx_a on a (json_extract(json_text, '$.name'));
select json_extract(json_text, '$.hello') from countries;
```
