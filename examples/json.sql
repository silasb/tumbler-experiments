PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE countries (json_text json);
INSERT INTO countries VALUES('{"name": "hello"}');
COMMIT;
