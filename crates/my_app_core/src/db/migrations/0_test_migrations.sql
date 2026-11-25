CREATE TABLE test(
    id SERIAL PRIMARY KEY,
    test STRING,
    test2 STRING
);

INSERT INTO test (test, test2) values (
                                       "hello", "world"
                                      )